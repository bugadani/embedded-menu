#![cfg_attr(not(test), no_std)]

pub mod adapters;
pub mod interaction;
pub mod items;

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
use embedded_layout::{
    layout::linear::{LinearLayout, Vertical},
    object_chain::ChainElement,
    prelude::*,
    view_group::ViewGroup,
};
use embedded_text::TextBox;
use interaction::{programmed::Programmed, InputType, InteractionController, InteractionType};

pub enum MenuDisplayMode {
    List,
    Details,
}

#[derive(Clone, Copy)]
pub struct Margin<V: View> {
    pub(crate) view: V,
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

impl<V: View> Margin<V> {
    pub fn new(view: V, top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Self {
            view,
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn inner(&self) -> &V {
        &self.view
    }

    pub fn inner_mut(&mut self) -> &mut V {
        &mut self.view
    }
}

impl<V: View> View for Margin<V> {
    /// Move the origin of an object by a given number of (x, y) pixels,
    /// by returning a new object
    fn translate_impl(&mut self, by: Point) {
        self.view.translate_mut(by);
    }

    /// Returns the bounding box of the `View` as a `Rectangle`
    fn bounds(&self) -> Rectangle {
        let bounds = self.view.bounds();
        let bottom_right = bounds.bottom_right().unwrap_or(bounds.top_left);
        Rectangle::with_corners(
            Point::new(bounds.top_left.x - self.left, bounds.top_left.y - self.top),
            Point::new(bottom_right.x + self.right, bottom_right.y + self.bottom),
        )
    }
}

pub trait MarginExt: View + Sized {
    fn with_margin(self, top: i32, right: i32, bottom: i32, left: i32) -> Margin<Self>;
}

impl<T> MarginExt for T
where
    T: View,
{
    fn with_margin(self, top: i32, right: i32, bottom: i32, left: i32) -> Margin<Self> {
        Margin::new(self, top, right, bottom, left)
    }
}

impl<'a, C, V> Drawable for Margin<V>
where
    C: PixelColor,
    V: Drawable<Color = C> + View,
{
    type Color = C;
    type Output = V::Output;

    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<D>(&self, display: &mut D) -> Result<V::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.view.draw(display)
    }
}

pub struct Animated {
    current: i32,
    target: i32,
    delta: i32,
}

impl Animated {
    pub fn new(current: i32, delta: i32) -> Self {
        Self {
            current,
            target: current,
            delta,
        }
    }

    pub fn update(&mut self) {
        if self.current == self.target {
            // nothing to do
        } else if self.current < self.target {
            self.current = self.current.saturating_add(self.delta).min(self.target);
        } else {
            self.current = self.current.saturating_sub(self.delta).max(self.target);
        }
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

pub trait MenuItemTrait<R: Copy>: View {
    // Draw itemtype-specific graphics
    fn interact(&mut self) -> MenuEvent<R>;
    fn title(&self) -> &str;
    fn details(&self) -> &str;
}

/// Menu-related extensions for object chain elements
pub trait MenuExt<R: Copy>: ChainElement {
    fn bounds_of(&self, nth: u32) -> Rectangle;
    fn title_of(&self, nth: u32) -> &str;
    fn details_of(&self, nth: u32) -> &str;
    fn interact_with(&mut self, nth: u32) -> MenuEvent<R>;
}

impl<I, R: Copy> MenuExt<R> for Chain<I>
where
    R: Copy,
    I: MenuItemTrait<R>,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        debug_assert!(nth == 0);
        self.object.bounds()
    }

    fn interact_with(&mut self, nth: u32) -> MenuEvent<R> {
        debug_assert!(nth == 0);
        self.object.interact()
    }

    fn title_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.object.title()
    }

    fn details_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.object.details()
    }
}

impl<I, LE, R> MenuExt<R> for Link<I, LE>
where
    R: Copy,
    I: MenuItemTrait<R>,
    LE: MenuExt<R>,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        if nth == 0 {
            self.object.bounds()
        } else {
            self.parent.bounds_of(nth - 1)
        }
    }

    fn interact_with(&mut self, nth: u32) -> MenuEvent<R> {
        if nth == 0 {
            self.object.interact()
        } else {
            self.parent.interact_with(nth - 1)
        }
    }

    fn title_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.title()
        } else {
            self.parent.title_of(nth - 1)
        }
    }

    fn details_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.details()
        } else {
            self.parent.details_of(nth - 1)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct MenuStyle<C: PixelColor> {
    pub(crate) color: C,
    pub(crate) indicator_color: C,
}

impl<C: PixelColor> MenuStyle<C> {
    pub fn new(color: C, indicator_color: C) -> Self {
        Self {
            color,
            indicator_color,
        }
    }
}

mod private {
    pub struct NoItems;
}

use private::NoItems;

use crate::adapters::{constrain::ConstrainedDrawTarget, invert::BinaryColorDrawTargetExt};

pub struct MenuBuilder<IT, LL, R, C>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    bounds: Rectangle,
    items: LL,
    interaction: IT,
    idle_timeout: Option<u16>,
    style: MenuStyle<C>,
}

impl<R: Copy> MenuBuilder<Programmed, NoItems, R, BinaryColor> {
    pub fn new(title: &'static str, bounds: Rectangle) -> Self {
        Self {
            _return_type: PhantomData,
            title,
            bounds,
            items: NoItems,
            interaction: Programmed::new(),
            idle_timeout: None,
            style: MenuStyle {
                color: BinaryColor::On,
                indicator_color: BinaryColor::On,
            },
        }
    }
}

impl<IT, LL, R, C> MenuBuilder<IT, LL, R, C>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
{
    pub fn show_details_after(self, timeout: u16) -> MenuBuilder<IT, LL, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction: self.interaction,
            idle_timeout: Some(timeout),
            style: self.style,
        }
    }

    pub fn with_interaction_controller<ITC: InteractionController>(
        self,
        interaction: ITC,
    ) -> MenuBuilder<ITC, LL, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }

    pub fn with_style<CC: PixelColor>(self, style: MenuStyle<CC>) -> MenuBuilder<IT, LL, R, CC> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style,
        }
    }
}

impl<IT, VG, R, C> MenuBuilder<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController,
    VG: ViewGroup + MenuExt<R>,
    C: PixelColor,
{
    pub fn build(self) -> Menu<IT, VG, R, C> {
        Menu {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: LinearLayout::vertical(self.items).arrange().into_inner(),
            interaction: self.interaction,
            selected: 0,
            recompute_targets: false,
            list_offset: Animated::new(0, 2),
            indicator_offset: Animated::new(0, 2),
            idle_timeout_threshold: self.idle_timeout,
            idle_timeout: self.idle_timeout.unwrap_or_default(),
            display_mode: MenuDisplayMode::List,
            style: self.style,
        }
    }
}

impl<IT, R, C> MenuBuilder<IT, NoItems, R, C>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
{
    pub fn add_item<I: MenuItemTrait<R>>(self, item: I) -> MenuBuilder<IT, Chain<I>, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: Chain::new(item),
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }
}

impl<IT, CE, R, C> MenuBuilder<IT, Chain<CE>, R, C>
where
    R: Copy,
    IT: InteractionController,
    CE: MenuItemTrait<R>,
    C: PixelColor,
{
    pub fn add_item<I: MenuItemTrait<R>>(
        self,
        item: I,
    ) -> MenuBuilder<IT, Link<I, Chain<CE>>, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items.append(item),
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }
}

impl<IT, P, CE, R, C> MenuBuilder<IT, Link<P, CE>, R, C>
where
    R: Copy,
    IT: InteractionController,
    P: MenuItemTrait<R>,
    CE: MenuExt<R>,
    C: PixelColor,
{
    pub fn add_item<I: MenuItemTrait<R>>(
        self,
        item: I,
    ) -> MenuBuilder<IT, Link<I, Link<P, CE>>, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items.append(item),
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }
}

pub struct Menu<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController,
    VG: ViewGroup + MenuExt<R>,
    C: PixelColor,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    bounds: Rectangle,
    items: VG,
    interaction: IT,
    selected: u32,
    recompute_targets: bool,
    list_offset: Animated,
    indicator_offset: Animated,
    idle_timeout_threshold: Option<u16>,
    idle_timeout: u16,
    display_mode: MenuDisplayMode,
    style: MenuStyle<C>,
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
                self.idle_timeout = self.idle_timeout - 1;
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

impl<'a, IT, VG, R, C> Menu<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController + Drawable<Color = C>,
    VG: ViewGroup + MenuExt<R> + Drawable<Color = C>,
    C: PixelColor + From<Rgb888>,
{
    fn header<'t, D>(
        &self,
        title: &'t str,
        display: &D,
    ) -> LinearLayout<
        Vertical<horizontal::Left>,
        Link<Styled<Line, PrimitiveStyle<C>>, Chain<TextBox<'t, MonoTextStyle<'static, C>>>>,
    >
    where
        D: DrawTarget<Color = C>,
    {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let text_style = MonoTextStyleBuilder::<C>::new()
            .font(&FONT_6X10)
            .text_color(self.style.color)
            .build();
        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);
        LinearLayout::vertical(
            Chain::new(TextBox::new(
                title,
                Rectangle::new(
                    Point::zero(),
                    Size::new(display_size.width, FONT_6X10.character_size.height),
                ),
                text_style,
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
    }

    pub fn update<D>(&mut self, display: &D)
    where
        D: DrawTarget<Color = C>,
    {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        match self.display_mode {
            MenuDisplayMode::List => {
                let menu_title = self.header(self.title, display);

                let menu_height = (display_size.height - menu_title.size().height) as i32;

                // Height of the first menu item
                let menuitem_height = self.items.bounds_of(0).size().height as i32;

                /* Reset positions */
                self.items
                    .align_to_mut(&display_area, horizontal::Left, vertical::Top);

                /* animations */
                if self.recompute_targets {
                    self.recompute_targets = false;

                    let current_target_top = self.list_offset.target();
                    let current_target_bottom = current_target_top + menu_height - 1;

                    let selected_top = self.items.bounds_of(self.selected).top_left.y;
                    let selected_bottom = selected_top + menuitem_height;

                    if selected_top < current_target_top {
                        self.list_offset.update_target(selected_top);
                    } else if selected_bottom > current_target_bottom {
                        self.list_offset
                            .update_target(selected_bottom - menu_height - 1);
                    } else {
                        // nothing to do
                    }

                    self.indicator_offset.update_target(selected_top);
                }

                self.list_offset.update();
                self.indicator_offset.update();

                self.items
                    .translate_mut(Point::new(1, -self.list_offset.current()));
                self.items
                    .translate_mut(Point::new(0, menu_title.size().height as i32));
            }
            _ => {}
        }
    }
}

impl<'a, IT, VG, R> Drawable for Menu<IT, VG, R, BinaryColor>
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
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let text_style = MonoTextStyleBuilder::<BinaryColor>::new()
            .font(&FONT_6X10)
            .text_color(self.style.color)
            .build();
        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);

        match self.display_mode {
            MenuDisplayMode::List => {
                let menu_title = self.header(self.title, display);
                menu_title.draw(display)?;

                let menu_height = display_size.height - menu_title.size().height;

                // Height of the first menu item
                let menuitem_height = self.items.bounds_of(0).size().height;
                let menuitem_inner_height = menuitem_height - 2;

                let scrollbar_area = Rectangle::new(Point::zero(), Size::new(2, menu_height))
                    .align_to(&menu_title, horizontal::Right, vertical::TopToBottom);

                let list_height = self.items.bounds().size().height;
                let draw_scrollbar = list_height > menu_height;

                let menu_list_width = if draw_scrollbar {
                    display_size.width - scrollbar_area.size().width
                } else {
                    display_size.width
                };

                let menu_display_area =
                    Rectangle::new(Point::zero(), Size::new(menu_list_width, menu_height))
                        .align_to(&menu_title, horizontal::Left, vertical::TopToBottom);

                // selection indicator
                let mut interaction_display = ConstrainedDrawTarget::new(
                    display,
                    Rectangle::new(
                        Point::zero(),
                        Size::new(menu_list_width, menuitem_inner_height),
                    )
                    .align_to(&menu_title, horizontal::Left, vertical::TopToBottom)
                    .translate(Point::new(
                        0,
                        self.indicator_offset.current() - self.list_offset.current() + 1,
                    )),
                );

                self.interaction.draw(&mut interaction_display)?;

                // FIXME: this is terrible
                let mut inverting_overlay = display.invert_area(&Rectangle::new(
                    Point::new(
                        0,
                        self.indicator_offset.current() - self.list_offset.current()
                            + 1
                            + menuitem_height as i32,
                    ),
                    Size::new(
                        self.interaction.fill_area_width(menu_list_width),
                        menuitem_inner_height,
                    ),
                ));

                self.items
                    .draw(&mut inverting_overlay.clipped(&menu_display_area))?;

                if draw_scrollbar {
                    let scale_factor = menu_height as f32 / list_height as f32;
                    let scale = |value| (value as f32 * scale_factor) as i32;

                    let scrollbar_height = scale(scrollbar_area.size().height as i32).max(1);
                    let mut scrollbar_display = display.cropped(&scrollbar_area);

                    // Start scrollbar from y=1, so we have a margin on top instead of bottom
                    Line::new(Point::new(1, 1), Point::new(1, scrollbar_height))
                        .into_styled(thin_stroke)
                        .translate(Point::new(0, scale(self.list_offset.current())))
                        .draw(&mut scrollbar_display)?;
                }
            }
            MenuDisplayMode::Details => {
                let header = self.header(self.items.title_of(self.selected), display);

                // TODO: embedded-layout should allow appending views at this point
                let size = header.size();
                let vg = header.into_inner();
                LinearLayout::vertical(
                    vg.append(
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
                .draw(display)?;
            }
        }
        Ok(())
    }
}
