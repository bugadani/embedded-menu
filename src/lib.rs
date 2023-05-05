#![cfg_attr(not(test), no_std)]

pub mod adapters;
pub mod builder;
pub mod interaction;
pub mod items;

mod margin;
mod plumbing;
mod styled;

use crate::{
    adapters::invert::{BinaryColorDrawTargetExt, ColorInvertingOverlay},
    builder::MenuBuilder,
    interaction::{programmed::Programmed, InteractionController, InteractionType},
    margin::MarginExt,
    plumbing::MenuExt,
    styled::StyledMenuItem,
};
use core::marker::PhantomData;
use embedded_graphics::{
    draw_target::{Cropped, DrawTarget},
    geometry::Size,
    mono_font::{ascii::FONT_6X10, MonoFont, MonoTextStyle},
    pixelcolor::{BinaryColor, PixelColor, Rgb888},
    prelude::{Dimensions, DrawTargetExt, Point},
    primitives::{Line, Primitive, PrimitiveStyle, Rectangle, Styled},
    Drawable,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*, view_group::ViewGroup};
use embedded_text::{
    style::{HeightMode, TextBoxStyle},
    TextBox,
};

pub trait MenuItem {
    type Data: Copy;

    fn interact(&mut self) -> MenuEvent<Self::Data>;
    fn title(&self) -> &str;
    fn details(&self) -> &str;
    fn value(&self) -> &str;
    fn longest_value_str(&self) -> &str;
}

enum MenuDisplayMode {
    List,
    Details,
}

impl MenuDisplayMode {
    fn is_list(&self) -> bool {
        matches!(self, Self::List)
    }

    fn is_details(&self) -> bool {
        matches!(self, Self::Details)
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
    pub(crate) scrollbar: DisplayScrollbar,
    pub(crate) font: &'static MonoFont<'static>,
    pub(crate) title_font: &'static MonoFont<'static>,
}

impl Default for MenuStyle<BinaryColor> {
    fn default() -> Self {
        Self::new(BinaryColor::On)
    }
}

impl<C> MenuStyle<C>
where
    C: PixelColor,
{
    pub fn new(color: C) -> Self {
        Self {
            color,
            scrollbar: DisplayScrollbar::Auto,
            font: &FONT_6X10,
            title_font: &FONT_6X10,
        }
    }

    pub fn with_font(self, font: &'static MonoFont<'static>) -> Self {
        Self { font, ..self }
    }

    pub fn with_title_font(self, title_font: &'static MonoFont<'static>) -> Self {
        Self { title_font, ..self }
    }

    pub fn text_style(&self) -> MonoTextStyle<'static, C> {
        MonoTextStyle::new(self.font, self.color)
    }

    pub fn title_style(&self) -> MonoTextStyle<'static, C> {
        MonoTextStyle::new(self.title_font, self.color)
    }
}

mod private {
    pub struct NoItems;
}

use private::NoItems;

trait SelectionIndicator: Sized {
    type Color: PixelColor;
    type Display<'a, D: DrawTarget<Color = Self::Color> + 'a>;

    fn new(anim_frames: i32) -> Self;

    fn update_target(&mut self, y: i32);

    fn offset(&self) -> i32;

    fn update(&mut self);

    fn draw<'d, D, R>(
        &self,
        indicator_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &'d mut D,
        op: impl Fn(&mut Cropped<'_, Self::Display<'d, D>>) -> Result<R, D::Error>,
    ) -> Result<R, D::Error>
    where
        D: DrawTarget<Color = Self::Color> + 'd;
}

struct SimpleSelectionIndicator {
    y_offset: Animated,
}

impl SelectionIndicator for SimpleSelectionIndicator {
    type Color = BinaryColor;
    type Display<'a, D: DrawTarget<Color = Self::Color> + 'a> = ColorInvertingOverlay<'a, D>;

    fn new(anim_frames: i32) -> Self {
        Self {
            y_offset: Animated::new(0, anim_frames),
        }
    }

    fn update_target(&mut self, y: i32) {
        self.y_offset.update_target(y);
    }

    fn offset(&self) -> i32 {
        self.y_offset.current()
    }

    fn update(&mut self) {
        self.y_offset.update();
    }

    fn draw<'d, D, R>(
        &self,
        indicator_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &'d mut D,
        op: impl Fn(&mut Cropped<'_, Self::Display<'d, D>>) -> Result<R, D::Error>,
    ) -> Result<R, D::Error>
    where
        D: DrawTarget<Color = Self::Color> + 'd,
    {
        Rectangle::new(
            Point::new(0, screen_offset),
            Size::new(fill_width.max(1), indicator_height),
        )
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)?;

        let display_top_left = display.bounding_box().top_left;
        let display_width = display.bounding_box().size.width;
        let display_height = display.bounding_box().size.height;

        let mut inverting = display.invert_area(&Rectangle::new(
            Point::new(0, screen_offset),
            Size::new(fill_width, indicator_height),
        ));
        op(&mut inverting.cropped(&Rectangle::new(
            display_top_left + Point::new(2, 0),
            Size::new(display_width - 2, display_height),
        )))
    }
}

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
    idle_timeout_threshold: Option<u16>,
    idle_timeout: u16,
    display_mode: MenuDisplayMode,
    style: MenuStyle<C>,
    indicator: SimpleSelectionIndicator,
}

impl<R: Copy, C: PixelColor> Menu<Programmed, NoItems, R, C> {
    pub fn builder(title: &'static str, bounds: Rectangle) -> MenuBuilder<Programmed, NoItems, R, C>
    where
        MenuStyle<C>: Default,
    {
        Self::build_with_style(title, bounds, MenuStyle::default())
    }

    pub fn build_with_style(
        title: &'static str,
        bounds: Rectangle,
        style: MenuStyle<C>,
    ) -> MenuBuilder<Programmed, NoItems, R, C>
    where
        MenuStyle<C>: Default,
    {
        MenuBuilder::new(title, bounds, style)
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

    fn selected_has_details(&self) -> bool {
        !self.items.details_of(self.selected).is_empty()
    }

    pub fn interact(&mut self, input: IT::Input) -> Option<MenuEvent<R>> {
        if let Some(threshold) = self.idle_timeout_threshold {
            self.idle_timeout = threshold;
            self.display_mode = MenuDisplayMode::List;
        }

        let count = ViewGroup::len(&self.items) as u32;
        match self.interaction.update(input) {
            Some(InteractionType::Next) => {
                let selected = self.selected.checked_sub(1).unwrap_or(count - 1);

                self.change_selected_item(selected);
                None
            }
            Some(InteractionType::Previous) => {
                let selected = (self.selected + 1) % count;

                self.change_selected_item(selected);
                None
            }
            Some(InteractionType::Select) => Some(self.items.interact_with(self.selected)),
            _ => None,
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
    VG: ViewGroup + MenuExt<R> + StyledMenuItem<BinaryColor>,
    C: PixelColor + From<Rgb888>,
{
    fn header<'t>(
        &self,
        title: &'t str,
        display: &impl Dimensions,
    ) -> Link<Styled<Line, PrimitiveStyle<C>>, Chain<TextBox<'t, MonoTextStyle<'static, C>>>> {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let text_style = self.style.title_style();
        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);
        LinearLayout::vertical(
            Chain::new(TextBox::with_textbox_style(
                title,
                Rectangle::new(
                    Point::zero(),
                    Size::new(display_size.width, text_style.font.character_size.height),
                ),
                text_style,
                TextBoxStyle::with_height_mode(HeightMode::FitToText),
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
        self.idle_timeout = self.idle_timeout.saturating_sub(1);
        if self.idle_timeout == 0 {
            self.display_mode = MenuDisplayMode::Details;
        }

        if !self.display_mode.is_list() {
            return;
        }

        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let menu_title = self.header(self.title, display);
        let title_height = menu_title.size().height as i32;

        let menu_height = display_size.height as i32 - title_height;

        // Reset positions
        self.items
            .align_to_mut(&display_area, horizontal::Left, vertical::Top);

        // Height of the selection indicator
        let selected_item_bounds = self.items.bounds_of(self.selected);

        // animations
        if self.recompute_targets {
            self.recompute_targets = false;

            self.indicator
                .update_target(selected_item_bounds.top_left.y);
        }

        self.indicator.update();

        // Ensure selection indicator is always visible
        let top_distance = self.indicator.offset() - self.list_offset;
        self.list_offset += if top_distance > 0 {
            let indicator_height = selected_item_bounds.size().height as i32;

            // Indicator is below display top. We only have to
            // move if indicator bottom is below display bottom.
            (top_distance + indicator_height - menu_height).max(0)
        } else {
            // We need to move up
            top_distance
        };

        self.items.translate_mut(Point::new(0, -self.list_offset));
    }

    fn display_details<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let header = self.header(self.items.title_of(self.selected), display);

        // TODO: embedded-layout should allow appending views to linear layout at this point
        let size = header.size();
        LinearLayout::vertical(
            header.append(
                TextBox::new(
                    self.items.details_of(self.selected),
                    Rectangle::new(
                        Point::zero(),
                        Size::new(size.width, display_size.height - size.height),
                    ),
                    self.style.text_style(),
                )
                .with_margin(0, 0, 0, 1),
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
    VG: ViewGroup + MenuExt<R> + StyledMenuItem<BinaryColor>,
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
        let menuitem_height = self.items.bounds_of(self.selected).size().height;

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

        self.indicator.draw(
            menuitem_height,
            self.indicator.offset() - self.list_offset + menu_title.size().height as i32,
            self.interaction.fill_area_width(menu_list_width),
            &mut display.clipped(&menu_display_area),
            |display| self.items.draw_styled(&self.style, display),
        )?;

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
    VG: ViewGroup + MenuExt<R> + StyledMenuItem<BinaryColor>,
{
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        if self.display_mode.is_details() && self.selected_has_details() {
            self.display_details(display)
        } else {
            self.display_list(display)
        }
    }
}
