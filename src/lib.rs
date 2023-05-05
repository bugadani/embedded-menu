#![cfg_attr(not(test), no_std)]

pub mod adapters;
pub mod builder;
pub mod interaction;
pub mod items;

mod margin;
mod plumbing;

use crate::{
    adapters::invert::BinaryColorDrawTargetExt,
    builder::MenuBuilder,
    interaction::{programmed::Programmed, InputType, InteractionController, InteractionType},
    margin::MarginExt,
    plumbing::MenuExt,
};
use core::marker::PhantomData;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::Size,
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::{BinaryColor, PixelColor, Rgb888},
    prelude::{Dimensions, DrawTargetExt, Point},
    primitives::{Line, Primitive, PrimitiveStyle, Rectangle, Styled},
    Drawable,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*, view_group::ViewGroup};
use embedded_text::{
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

pub trait MenuItemTrait<R: Copy>: View {
    fn interact(&mut self) -> MenuEvent<R>;
    fn title(&self) -> &str;
    fn details(&self) -> &str;
}

enum MenuDisplayMode {
    List,
    Details,
}

impl MenuDisplayMode {
    fn is_list(&self) -> bool {
        matches!(self, Self::List)
    }
}

pub struct Animated {
    current: i32,
    target: i32,
    frames: i32,
}

impl Animated {
    pub fn new(current: i32, frames: i32) -> Self {
        Self {
            current,
            target: current,
            frames,
        }
    }

    pub fn update(&mut self) {
        let rounding = if self.current < self.target {
            self.frames - 1
        } else {
            1 - self.frames
        };

        let distance = self.target - self.current;
        self.current += (distance + rounding) / self.frames;
    }

    pub fn update_target(&mut self, target: i32) {
        self.target = target;
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn target(&self) -> i32 {
        self.target
    }
}

pub enum MenuEvent<R: Copy> {
    Nothing,
    NavigationEvent(R),
    DataEvent(R),
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayScrollbar {
    Display,
    Hide,
    Auto,
}

#[derive(Copy, Clone, Debug)]
pub struct MenuStyle<C: PixelColor> {
    pub(crate) color: C,
    pub(crate) indicator_color: C,
    pub(crate) scrollbar: DisplayScrollbar,
}

impl<C: PixelColor> MenuStyle<C> {
    pub fn new(color: C, indicator_color: C) -> Self {
        Self {
            color,
            indicator_color,
            scrollbar: DisplayScrollbar::Auto,
        }
    }
}

mod private {
    pub struct NoItems;
}

use private::NoItems;

pub struct Menu<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    bounds: Rectangle,
    items: VG,
    interaction: IT,
    selected: u32,
    recompute_targets: bool,
    list_offset: i32,
    indicator_offset: Animated,
    idle_timeout_threshold: Option<u16>,
    idle_timeout: u16,
    display_mode: MenuDisplayMode,
    style: MenuStyle<C>,
}

impl<R: Copy> Menu<Programmed, NoItems, R, BinaryColor> {
    pub fn builder(
        title: &'static str,
        bounds: Rectangle,
    ) -> MenuBuilder<Programmed, NoItems, R, BinaryColor> {
        MenuBuilder::new(title, bounds)
    }
}

impl<IT, VG, R, C> Menu<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController,
    VG: ViewGroup + MenuExt<R>,
    C: PixelColor,
{
    fn change_selected_item(&mut self, new_selected: u32) {
        if new_selected != self.selected {
            self.selected = new_selected;
            self.recompute_targets = true;
        }
    }

    pub fn reset_interaction(&mut self) {
        self.interaction.reset();
    }

    pub fn interact(&mut self, input: IT::Input) -> MenuEvent<R> {
        if let Some(threshold) = self.idle_timeout_threshold {
            if input.is_active() {
                self.idle_timeout = threshold;
                self.display_mode = MenuDisplayMode::List;
            } else if self.idle_timeout > 0 {
                self.idle_timeout -= 1;
                if self.idle_timeout == 0 {
                    self.display_mode = MenuDisplayMode::Details;
                }
            }
        }

        let count = ViewGroup::len(&self.items) as u32;
        match self.interaction.update(input) {
            InteractionType::Nothing => MenuEvent::Nothing,
            InteractionType::Next => {
                let selected = self.selected.checked_sub(1).unwrap_or(count - 1);

                self.change_selected_item(selected);
                MenuEvent::Nothing
            }
            InteractionType::Previous => {
                let selected = (self.selected + 1) % count;

                self.change_selected_item(selected);
                MenuEvent::Nothing
            }
            InteractionType::Select => self.items.interact_with(self.selected),
        }
    }
}

impl<IT, VG, R, C> View for Menu<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController,
    VG: ViewGroup + MenuExt<R>,
    C: PixelColor,
{
    /// Move the origin of an object by a given number of (x, y) pixels,
    /// by returning a new object
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    /// Returns the bounding box of the `View` as a `Rectangle`
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl<IT, VG, R, C> Menu<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController + Drawable<Color = C>,
    VG: ViewGroup + MenuExt<R> + Drawable<Color = C>,
    C: PixelColor + From<Rgb888>,
{
    fn header<'t>(
        &self,
        title: &'t str,
        display: &impl Dimensions,
    ) -> Link<Styled<Line, PrimitiveStyle<C>>, Chain<TextBox<'t, MonoTextStyle<'static, C>>>> {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let font = &FONT_6X10;

        let text_style = MonoTextStyleBuilder::<C>::new()
            .font(font)
            .text_color(self.style.color)
            .build();
        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);
        LinearLayout::vertical(
            Chain::new(TextBox::with_textbox_style(
                title,
                Rectangle::new(
                    Point::zero(),
                    Size::new(display_size.width, font.character_size.height),
                ),
                text_style,
                TextBoxStyleBuilder::new()
                    .height_mode(HeightMode::FitToText)
                    .build(),
            ))
            .append(
                Line::new(
                    Point::zero(),
                    Point::new(display_area.bottom_right().unwrap().x, 0),
                )
                .into_styled(thin_stroke),
            ),
        )
        .arrange()
        .into_inner()
    }

    pub fn update(&mut self, display: &impl Dimensions) {
        if !self.display_mode.is_list() {
            return;
        }

        let display_size = display.bounding_box().size();

        let menu_title = self.header(self.title, display);
        let title_height = menu_title.size().height as i32;

        let menu_height = display_size.height as i32 - title_height;

        // Height of the selection indicator
        let menuitem_height = self.items.bounds_of(0).size().height;
        let indicator_height = menuitem_height as i32 - 1;

        // Reset positions
        self.items
            .align_to_mut(&menu_title, horizontal::Left, vertical::TopToBottom);

        // animations
        if self.recompute_targets {
            self.recompute_targets = false;

            self.indicator_offset
                .update_target(self.items.bounds_of(self.selected).top_left.y);
        }

        self.indicator_offset.update();

        // Ensure selection indicator is always visible
        let top_distance = self.indicator_offset.current() - self.list_offset;
        self.list_offset += if top_distance > 0 {
            // Indicator is below display top. We only have to
            // move if indicator bottom is below display bottom.
            (top_distance + indicator_height - menu_height).max(0)
        } else {
            // We need to move up
            top_distance
        };

        self.items.translate_mut(Point::new(1, -self.list_offset));
    }

    fn display_details<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let text_style = MonoTextStyleBuilder::<C>::new()
            .font(&FONT_6X10)
            .text_color(self.style.color)
            .build();

        let header = self.header(self.items.title_of(self.selected), display);

        // TODO: embedded-layout should allow appending views to linear layout at this point
        let size = header.size();
        LinearLayout::vertical(
            header.append(
                TextBox::new(
                    self.items.details_of(self.selected),
                    Rectangle::new(
                        Point::zero(),
                        Size::new(size.width, display_size.height - size.height - 2),
                    ),
                    text_style,
                )
                .with_margin(2, 0, 0, 1),
            ),
        )
        .arrange()
        .draw(display)
    }
}

impl<IT, VG, R> Menu<IT, VG, R, BinaryColor>
where
    R: Copy,
    IT: InteractionController + Drawable<Color = BinaryColor>,
    VG: ViewGroup + MenuExt<R> + Drawable<Color = BinaryColor>,
{
    fn display_list<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);

        let menu_title = self.header(self.title, display);
        menu_title.draw(display)?;

        let menu_height = display_size.height - menu_title.size().height;

        // Height of the first menu item
        let menuitem_height = self.items.bounds_of(0).size().height;
        let selection_indicator_height = menuitem_height - 2; // We don't want to extend under the baseline

        let scrollbar_area = Rectangle::new(Point::zero(), Size::new(2, menu_height)).align_to(
            &menu_title,
            horizontal::Right,
            vertical::TopToBottom,
        );

        let list_height = self.items.bounds().size().height;
        let draw_scrollbar = match self.style.scrollbar {
            DisplayScrollbar::Display => true,
            DisplayScrollbar::Hide => false,
            DisplayScrollbar::Auto => list_height > menu_height,
        };

        let menu_list_width = if draw_scrollbar {
            display_size.width - scrollbar_area.size().width
        } else {
            display_size.width
        };

        let menu_display_area = Rectangle::new(
            Point::zero(),
            Size::new(menu_list_width, menu_height),
        )
        .align_to(&menu_title, horizontal::Left, vertical::TopToBottom);

        // selection indicator
        let mut interaction_display = display.cropped(
            &Rectangle::new(
                Point::zero(),
                Size::new(menu_list_width, selection_indicator_height),
            )
            .align_to(&menu_title, horizontal::Left, vertical::TopToBottom)
            .translate(Point::new(
                0,
                self.indicator_offset.current() - self.list_offset + 1,
            )),
        );

        self.interaction.draw(&mut interaction_display)?;

        // FIXME: this is terrible
        let mut inverting_overlay = display.invert_area(&Rectangle::new(
            Point::new(
                0,
                self.indicator_offset.current() - self.list_offset + 1 + menuitem_height as i32,
            ),
            Size::new(
                self.interaction.fill_area_width(menu_list_width),
                menuitem_height,
            ),
        ));

        self.items
            .draw(&mut inverting_overlay.clipped(&menu_display_area))?;

        if draw_scrollbar {
            let scale = |value| (value * menu_height / list_height) as i32;

            let scrollbar_height = scale(menu_height).max(1);
            let mut scrollbar_display = display.cropped(&scrollbar_area);

            // Start scrollbar from y=1, so we have a margin on top instead of bottom
            Line::new(Point::new(0, 1), Point::new(0, scrollbar_height))
                .into_styled(thin_stroke)
                .translate(Point::new(1, scale(self.list_offset as u32)))
                .draw(&mut scrollbar_display)?;
        }

        Ok(())
    }
}

impl<IT, VG, R> Drawable for Menu<IT, VG, R, BinaryColor>
where
    R: Copy,
    IT: InteractionController + Drawable<Color = BinaryColor>,
    VG: ViewGroup + MenuExt<R> + Drawable<Color = BinaryColor>,
{
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        match self.display_mode {
            MenuDisplayMode::List => self.display_list(display),
            MenuDisplayMode::Details => self.display_details(display),
        }
    }
}
