#![cfg_attr(not(test), no_std)]

pub mod interaction;
pub mod items;

use core::marker::PhantomData;
use embedded_graphics::{
    fonts::Font6x8,
    pixelcolor::BinaryColor,
    primitives::{Line, Rectangle},
    style::PrimitiveStyle,
    DrawTarget,
    drawable::Pixel,
    geometry::Size,
    pixelcolor::PixelColor,
};
use embedded_layout::{
    layout::{
        linear::{LayoutElement, LinearLayout, Vertical},
        Guard, Link, ViewChainElement, ViewGroup,
    },
    prelude::*,
};
use embedded_text::prelude::*;
use interaction::{programmed::Programmed, InputType, InteractionController, InteractionType};

pub trait RectangleExt {
    fn expand(self, by: i32) -> Rectangle;
    fn contains(&self, p: Point) -> bool;
    fn intersects_with(&self, other: &Rectangle) -> bool;
}

impl RectangleExt for Rectangle {
    #[inline]
    fn expand(self, by: i32) -> Rectangle {
        Rectangle::new(
            Point::new(self.top_left.x - by, self.top_left.y - by),
            Point::new(self.bottom_right.x + by, self.bottom_right.y + by),
        )
    }

    #[inline]
    fn contains(&self, p: Point) -> bool {
        self.top_left.x <= p.x
            && p.x <= self.bottom_right.x
            && self.top_left.y <= p.y
            && p.y <= self.bottom_right.y
    }

    #[inline]
    fn intersects_with(&self, other: &Rectangle) -> bool {
        self.top_left.x <= other.bottom_right.x
            && other.top_left.x <= self.bottom_right.x
            && self.top_left.y <= other.bottom_right.y
            && other.top_left.y <= self.bottom_right.y
    }
}

pub struct ConstrainedDrawTarget<'a, C: PixelColor, D: DrawTarget<C>> {
    clipping_rect: Rectangle,
    display: &'a mut D,
    _color: PhantomData<C>,
}

impl<'a, C: PixelColor, D: DrawTarget<C>> ConstrainedDrawTarget<'a, C, D> {
    pub fn new(display: &'a mut D, bounds: Rectangle) -> Self {
        Self {
            display,
            clipping_rect: bounds,
            _color: PhantomData,
        }
    }
}

impl<'a, C: PixelColor, D: DrawTarget<C>> DrawTarget<C> for ConstrainedDrawTarget<'a, C, D> {
    type Error = D::Error;

    /// Returns the dimensions of the `DrawTarget` in pixels.
    fn size(&self) -> Size {
        self.clipping_rect.size()
    }

    fn draw_pixel(&mut self, item: Pixel<C>) -> Result<(), Self::Error> {
        let item = Pixel(item.0 + self.clipping_rect.top_left, item.1);
        if self.clipping_rect.contains(item.0) {
            self.display.draw_pixel(item)
        } else {
            Ok(())
        }
    }
}


pub enum MenuDisplayMode {
    List,
    Details,
}

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
    #[must_use]
    fn translate(mut self, by: Point) -> Self {
        self.translate_mut(by);
        self
    }

    /// Move the origin of an object by a given number of (x, y) pixels,
    /// mutating the object in place
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.view.translate_mut(by);
        self
    }

    /// Returns the bounding box of the `View` as a `Rectangle`
    fn bounds(&self) -> Rectangle {
        let bounds = self.view.bounds();
        Rectangle::new(
            Point::new(bounds.top_left.x - self.left, bounds.top_left.y - self.top),
            Point::new(
                bounds.bottom_right.x + self.right,
                bounds.bottom_right.y + self.bottom,
            ),
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

impl<'a, C, V> Drawable<C> for &'a Margin<V>
where
    C: PixelColor,
    V: View,
    &'a V: Drawable<C>,
{
    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
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
pub trait MenuExt<R: Copy> {
    fn bounds_of(&self, nth: u32) -> Rectangle;
    fn title_of(&self, nth: u32) -> &str;
    fn details_of(&self, nth: u32) -> &str;
    fn interact_with(&mut self, nth: u32) -> MenuEvent<R>;
}

impl<R: Copy> MenuExt<R> for Guard {
    fn bounds_of(&self, _nth: u32) -> Rectangle {
        Rectangle::new(Point::zero(), Point::zero())
    }

    fn interact_with(&mut self, _nth: u32) -> MenuEvent<R> {
        MenuEvent::Nothing
    }

    fn title_of(&self, _nth: u32) -> &str {
        ""
    }

    fn details_of(&self, _nth: u32) -> &str {
        ""
    }
}

impl<I, LE, R> MenuExt<R> for Link<I, LE>
where
    R: Copy,
    I: MenuItemTrait<R>,
    LE: LayoutElement<Vertical<horizontal::Left>> + MenuExt<R>,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        if nth == 0 {
            self.object.bounds()
        } else {
            self.next.bounds_of(nth - 1)
        }
    }

    fn interact_with(&mut self, nth: u32) -> MenuEvent<R> {
        if nth == 0 {
            self.object.interact()
        } else {
            self.next.interact_with(nth - 1)
        }
    }

    fn title_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.title()
        } else {
            self.next.title_of(nth - 1)
        }
    }

    fn details_of(&self, nth: u32) -> &str {
        if nth == 0 {
            self.object.details()
        } else {
            self.next.details_of(nth - 1)
        }
    }
}

impl<LE, R> MenuExt<R> for ViewGroup<LE>
where
    R: Copy,
    LE: ViewChainElement + MenuExt<R>,
{
    fn bounds_of(&self, nth: u32) -> Rectangle {
        self.views.bounds_of(self.view_count() - nth - 1)
    }

    fn interact_with(&mut self, nth: u32) -> MenuEvent<R> {
        self.views.interact_with(self.view_count() - nth - 1)
    }

    fn title_of(&self, nth: u32) -> &str {
        self.views.title_of(self.view_count() - nth - 1)
    }

    fn details_of(&self, nth: u32) -> &str {
        self.views.details_of(self.view_count() - nth - 1)
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

pub struct MenuBuilder<IT, LE, R, C>
where
    R: Copy,
    IT: InteractionController,
    LE: LayoutElement<Vertical<horizontal::Left>> + MenuExt<R>,
    C: PixelColor,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    bounds: Rectangle,
    items: LinearLayout<Vertical<horizontal::Left>, LE>,
    interaction: IT,
    idle_timeout: Option<u16>,
    style: MenuStyle<C>,
}

impl<R: Copy> MenuBuilder<Programmed, Guard, R, BinaryColor> {
    pub fn new(title: &'static str, bounds: Rectangle) -> Self {
        Self {
            _return_type: PhantomData,
            title,
            bounds,
            items: LinearLayout::vertical(),
            interaction: Programmed::new(),
            idle_timeout: None,
            style: MenuStyle {
                color: BinaryColor::On,
                indicator_color: BinaryColor::On,
            },
        }
    }
}

impl<IT, LE, R, C> MenuBuilder<IT, LE, R, C>
where
    R: Copy,
    IT: InteractionController,
    LE: LayoutElement<Vertical<horizontal::Left>> + MenuExt<R>,
    C: PixelColor,
{
    pub fn show_details_after(self, timeout: u16) -> MenuBuilder<IT, LE, R, C> {
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
    ) -> MenuBuilder<ITC, LE, R, C> {
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

    pub fn add_item<I: MenuItemTrait<R>>(self, item: I) -> MenuBuilder<IT, Link<I, LE>, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items.add_view(item),
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }

    pub fn with_style<CC: PixelColor>(self, style: MenuStyle<CC>) -> MenuBuilder<IT, LE, R, CC> {
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

    pub fn build(self) -> Menu<IT, LE, R, C> {
        Menu {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items.arrange(),
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

pub struct Menu<IT, LE, R, C>
where
    R: Copy,
    IT: InteractionController,
    LE: ViewChainElement + MenuExt<R>,
    C: PixelColor,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    bounds: Rectangle,
    items: ViewGroup<LE>,
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

impl<IT, LE, R, C> Menu<IT, LE, R, C>
where
    R: Copy,
    IT: InteractionController,
    LE: ViewChainElement + MenuExt<R>,
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

        match self.interaction.update(input) {
            InteractionType::Nothing => MenuEvent::Nothing,
            InteractionType::Previous => {
                let selected = if self.selected == 0 {
                    self.items.view_count() - 1
                } else {
                    self.selected - 1
                };

                self.change_selected_item(selected);
                MenuEvent::Nothing
            }
            InteractionType::Next => {
                let selected = if self.selected == self.items.view_count() - 1 {
                    0
                } else {
                    self.selected + 1
                };

                self.change_selected_item(selected);
                MenuEvent::Nothing
            }
            InteractionType::Select => self.items.interact_with(self.selected),
        }
    }
}

impl<IT, LE, R, C> View for Menu<IT, LE, R, C>
where
    R: Copy,
    IT: InteractionController,
    LE: ViewChainElement + MenuExt<R>,
    C: PixelColor,
{
    /// Move the origin of an object by a given number of (x, y) pixels,
    /// by returning a new object
    #[must_use]
    fn translate(mut self, by: Point) -> Self {
        self.bounds.translate_mut(by);
        self
    }

    /// Move the origin of an object by a given number of (x, y) pixels,
    /// mutating the object in place
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.bounds.translate_mut(by);
        self
    }

    /// Returns the bounding box of the `View` as a `Rectangle`
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

impl<'a, IT, LE, R, C> Drawable<C> for &'a mut Menu<IT, LE, R, C>
where
    R: Copy,
    IT: InteractionController,
    &'a IT: Drawable<C>,
    LE: ViewChainElement + MenuExt<R>,
    &'a LE: Drawable<C>,
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let display_area = DisplayArea::<C>::display_area(display);
        let display_size = display_area.size();

        let text_style = TextBoxStyleBuilder::new(Font6x8)
            .text_color(self.style.color)
            .build();
        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);

        match self.display_mode {
            MenuDisplayMode::List => {
                let menu_title = LinearLayout::vertical()
                    .add_view(
                        TextBox::new(
                            self.title,
                            Rectangle::with_size(
                                Point::zero(),
                                Size::new(display_size.width, Font6x8::CHARACTER_SIZE.height),
                            ),
                        )
                        .into_styled(text_style),
                    )
                    .add_view(
                        Line::new(Point::zero(), Point::new(display_area.bottom_right.x, 0))
                            .into_styled(thin_stroke),
                    )
                    .arrange();
                menu_title.draw(display)?;

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

                let scrollbar_area = Rectangle::with_size(
                    Point::zero(),
                    Size::new(2, menu_height as u32),
                )
                .align_to(&menu_title, horizontal::Right, vertical::TopToBottom);

                let list_height = self.items.bounds().size().height;
                let draw_scrollbar = list_height > scrollbar_area.size().height;

                let menu_list_width = if draw_scrollbar {
                    display_size.width - scrollbar_area.size().width
                } else {
                    display_size.width
                };

                let mut constrained_display = ConstrainedDrawTarget::new(
                    display,
                    Rectangle::with_size(
                        Point::zero(),
                        Size::new(menu_list_width, menu_height as u32),
                    )
                    .align_to(
                        &display_area,
                        horizontal::Left,
                        vertical::Bottom,
                    ),
                );

                self.items
                    .translate_mut(Point::new(2, -self.list_offset.current()))
                    .draw(&mut constrained_display)?;

                // selection indicator
                let mut interaction_display = ConstrainedDrawTarget::new(
                    display,
                    Rectangle::with_size(
                        Point::zero(),
                        Size::new(menu_list_width, menuitem_height as u32 - 2),
                    )
                    .align_to(&menu_title, horizontal::Left, vertical::TopToBottom)
                    .translate(Point::new(
                        0,
                        self.indicator_offset.current() - self.list_offset.current() + 1,
                    )),
                );

                self.interaction.draw(&mut interaction_display)?;

                if draw_scrollbar {
                    let scale =
                        |value| (value as f32 * menu_height as f32 / list_height as f32) as i32;

                    let scrollbar_height = scale(scrollbar_area.size().height as i32).max(1);
                    let mut scrollbar_display = ConstrainedDrawTarget::new(display, scrollbar_area);

                    // Start scrollbar from y=1, so we have a margin on top instead of bottom
                    Line::new(Point::new(1, 1), Point::new(1, scrollbar_height))
                        .into_styled(thin_stroke)
                        .translate(Point::new(0, scale(self.list_offset.current())))
                        .draw(&mut scrollbar_display)?;
                }
            }
            _ => {
                let layout = LinearLayout::vertical()
                    .add_view(
                        TextBox::new(
                            self.items.title_of(self.selected),
                            Rectangle::with_size(
                                Point::zero(),
                                Size::new(display_size.width, Font6x8::CHARACTER_SIZE.height),
                            ),
                        )
                        .into_styled(text_style),
                    )
                    .add_view(
                        Line::new(Point::zero(), Point::new(display_area.bottom_right.x, 0))
                            .into_styled(thin_stroke),
                    );

                let size = layout.size();

                layout
                    .add_view(
                        TextBox::new(
                            self.items.details_of(self.selected),
                            Rectangle::with_size(
                                Point::zero(),
                                Size::new(size.width, display_size.height - size.height - 2),
                            ),
                        )
                        .into_styled(text_style)
                        .with_margin(2, 0, 0, 0),
                    )
                    .arrange()
                    .draw(display)?;
            }
        }
        Ok(())
    }
}
