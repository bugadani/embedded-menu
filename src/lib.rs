#![cfg_attr(not(test), no_std)]

pub mod adapters;
pub mod builder;
pub mod collection;
pub mod interaction;
pub mod items;
pub mod selection_indicator;

mod margin;

use crate::{
    builder::MenuBuilder,
    collection::MenuItemCollection,
    interaction::{programmed::Programmed, InteractionController, InteractionType},
    margin::MarginExt,
    selection_indicator::{
        style::{line::Line as LineIndicator, IndicatorStyle},
        AnimatedPosition, Indicator, SelectionIndicatorController, State as IndicatorState,
        StaticPosition,
    },
};
use core::marker::PhantomData;
use embedded_graphics::{
    draw_target::DrawTarget,
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

pub use embedded_menu_macros::{Menu, SelectValue};

/// Marker trait necessary to avoid a "conflicting implementations" error.
pub trait Marker {}

pub trait MenuItem<D>: Marker + View {
    fn interact(&mut self) -> D;
    fn set_style<C, S, IT, P>(&mut self, style: &MenuStyle<C, S, IT, P>)
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InteractionController,
        P: SelectionIndicatorController;
    fn title(&self) -> &str;
    fn details(&self) -> &str;
    fn value(&self) -> &str;
    fn draw_styled<C, S, IT, P, DIS>(
        &self,
        style: &MenuStyle<C, S, IT, P>,
        display: &mut DIS,
    ) -> Result<(), DIS::Error>
    where
        C: PixelColor + From<Rgb888>,
        S: IndicatorStyle,
        IT: InteractionController,
        P: SelectionIndicatorController,

        DIS: DrawTarget<Color = C>;
}

#[derive(Clone, Copy)]
enum MenuDisplayMode {
    List(u16),
    Details,
}

impl Default for MenuDisplayMode {
    fn default() -> Self {
        Self::List(0)
    }
}

impl MenuDisplayMode {
    fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    fn is_details(&self) -> bool {
        matches!(self, Self::Details)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DisplayScrollbar {
    Display,
    Hide,
    Auto,
}

#[derive(Copy, Clone, Debug)]
pub struct MenuStyle<C, S, IT, P>
where
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
    pub(crate) color: C,
    pub(crate) scrollbar: DisplayScrollbar,
    pub(crate) font: &'static MonoFont<'static>,
    pub(crate) title_font: &'static MonoFont<'static>,
    pub(crate) details_delay: Option<u16>,
    pub(crate) interaction: IT,
    pub(crate) indicator: Indicator<P, S>,
}

impl Default for MenuStyle<BinaryColor, LineIndicator, Programmed, StaticPosition> {
    fn default() -> Self {
        Self::new(BinaryColor::On)
    }
}

impl<C> MenuStyle<C, LineIndicator, Programmed, StaticPosition>
where
    C: PixelColor,
{
    pub const fn new(color: C) -> Self {
        Self {
            color,
            scrollbar: DisplayScrollbar::Auto,
            font: &FONT_6X10,
            title_font: &FONT_6X10,
            details_delay: None,
            interaction: Programmed,
            indicator: Indicator {
                style: LineIndicator,
                controller: StaticPosition,
            },
        }
    }
}

impl<C, S, IT, P> MenuStyle<C, S, IT, P>
where
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
    pub const fn with_font(self, font: &'static MonoFont<'static>) -> Self {
        Self { font, ..self }
    }

    pub const fn with_title_font(self, title_font: &'static MonoFont<'static>) -> Self {
        Self { title_font, ..self }
    }

    pub const fn with_scrollbar_style(self, scrollbar: DisplayScrollbar) -> Self {
        Self { scrollbar, ..self }
    }

    pub const fn with_details_delay(self, frames: u16) -> Self {
        Self {
            details_delay: Some(frames),
            ..self
        }
    }

    pub const fn with_selection_indicator<S2>(self, indicator_style: S2) -> MenuStyle<C, S2, IT, P>
    where
        S2: IndicatorStyle,
    {
        MenuStyle {
            color: self.color,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            details_delay: self.details_delay,
            interaction: self.interaction,
            indicator: Indicator {
                style: indicator_style,
                controller: self.indicator.controller,
            },
        }
    }

    pub const fn with_interaction_controller<IT2>(self, interaction: IT2) -> MenuStyle<C, S, IT2, P>
    where
        IT2: InteractionController,
    {
        MenuStyle {
            interaction,
            color: self.color,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            details_delay: self.details_delay,
            indicator: self.indicator,
        }
    }

    pub const fn with_animated_selection_indicator(
        &self,
        frames: i32,
    ) -> MenuStyle<C, S, IT, AnimatedPosition> {
        MenuStyle {
            interaction: self.interaction,
            color: self.color,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            details_delay: self.details_delay,
            indicator: Indicator {
                style: self.indicator.style,
                controller: AnimatedPosition::new(frames),
            },
        }
    }

    pub fn text_style(&self) -> MonoTextStyle<'static, C> {
        MonoTextStyle::new(self.font, self.color)
    }

    pub fn title_style(&self) -> MonoTextStyle<'static, C> {
        MonoTextStyle::new(self.title_font, self.color)
    }
}

pub struct NoItems;

pub struct MenuState<IT, P, S>
where
    IT: InteractionController,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    selected: usize,
    recompute_targets: bool,
    list_offset: i32,
    display_mode: MenuDisplayMode,
    interaction_state: IT::State,
    indicator_state: IndicatorState<P, S>,
}

impl<IT, P, S> Default for MenuState<IT, P, S>
where
    IT: InteractionController,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn default() -> Self {
        Self {
            selected: 0,
            recompute_targets: Default::default(),
            list_offset: Default::default(),
            display_mode: Default::default(),
            interaction_state: Default::default(),
            indicator_state: Default::default(),
        }
    }
}

impl<IT, P, S> Clone for MenuState<IT, P, S>
where
    IT: InteractionController,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<IT, P, S> Copy for MenuState<IT, P, S>
where
    IT: InteractionController,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
}

impl<IT, P, S> MenuState<IT, P, S>
where
    IT: InteractionController,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn change_selected_item(&mut self, new_selected: usize) {
        if new_selected != self.selected {
            self.selected = new_selected;
            self.recompute_targets = true;
        }
    }

    pub fn reset_interaction(&mut self) {
        self.interaction_state = Default::default();
    }
}

pub struct Menu<T, IT, VG, R, C, P, S>
where
    T: AsRef<str>,
    IT: InteractionController,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    _return_type: PhantomData<R>,
    title: T,
    items: VG,
    style: MenuStyle<C, S, IT, P>,
    state: MenuState<IT, P, S>,
}

impl<T, R, C, S> Menu<T, Programmed, NoItems, R, C, StaticPosition, S>
where
    T: AsRef<str>,
    C: PixelColor,
    S: IndicatorStyle,
{
    pub fn new(title: T) -> MenuBuilder<T, Programmed, NoItems, R, C, StaticPosition, S>
    where
        MenuStyle<C, S, Programmed, StaticPosition>: Default,
    {
        Self::with_style(title, MenuStyle::default())
    }
}

impl<T, IT, R, C, P, S> Menu<T, IT, NoItems, R, C, P, S>
where
    T: AsRef<str>,
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
    pub fn with_style(
        title: T,
        style: MenuStyle<C, S, IT, P>,
    ) -> MenuBuilder<T, IT, NoItems, R, C, P, S> {
        MenuBuilder::new(title, style)
    }
}

impl<T, IT, VG, R, C, P, S> Menu<T, IT, VG, R, C, P, S>
where
    T: AsRef<str>,
    IT: InteractionController,
    VG: MenuItemCollection<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn selected_has_details(&self) -> bool {
        !self.items.details_of(self.state.selected).is_empty()
    }

    pub fn interact(&mut self, input: IT::Input) -> Option<R> {
        if let Some(threshold) = self.style.details_delay {
            self.state.display_mode = MenuDisplayMode::List(threshold);
        }

        let count = self.items.count();
        match self
            .style
            .interaction
            .update(&mut self.state.interaction_state, input)
        {
            Some(InteractionType::Next) => {
                let selected = (self.state.selected + 1) % count;

                self.state.change_selected_item(selected);
                None
            }
            Some(InteractionType::Previous) => {
                let selected = self.state.selected.checked_sub(1).unwrap_or(count - 1);

                self.state.change_selected_item(selected);
                None
            }
            Some(InteractionType::Select) => Some(self.items.interact_with(self.state.selected)),
            _ => None,
        }
    }

    pub fn state(&self) -> MenuState<IT, P, S> {
        self.state
    }
}

impl<T, IT, VG, R, C, P, S> Menu<T, IT, VG, R, C, P, S>
where
    T: AsRef<str>,
    IT: InteractionController,
    VG: ViewGroup + MenuItemCollection<R>,
    C: PixelColor + From<Rgb888>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
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
        if self.style.details_delay.is_some() && self.selected_has_details() {
            if let MenuDisplayMode::List(ref mut timeout) = self.state.display_mode {
                *timeout = timeout.saturating_sub(1);
                if *timeout == 0 {
                    self.state.display_mode = MenuDisplayMode::Details;
                }
            }
        }

        if !self.state.display_mode.is_list() {
            return;
        }

        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let menu_title = self.header(self.title.as_ref(), display);
        let title_height = menu_title.size().height as i32;

        let menu_height = display_size.height as i32 - title_height;

        // Reset positions
        self.items
            .align_to_mut(&display_area, horizontal::Left, vertical::Top);

        // Height of the selection indicator
        let selected_item_bounds = self.items.bounds_of(self.state.selected);

        // animations
        if self.state.recompute_targets {
            self.state.recompute_targets = false;
            self.style.indicator.change_selected_item(
                selected_item_bounds.top_left.y,
                &mut self.state.indicator_state,
            );
        }

        self.style.indicator.update(
            self.style
                .interaction
                .fill_area_width(&self.state.interaction_state, display_size.width),
            &mut self.state.indicator_state,
        );

        // Ensure selection indicator is always visible
        let top_distance =
            self.style.indicator.offset(&self.state.indicator_state) - self.state.list_offset;
        self.state.list_offset += if top_distance > 0 {
            let indicator_height = self.style.indicator.item_height(
                selected_item_bounds.size().height,
                &self.state.indicator_state,
            ) as i32;

            // Indicator is below display top. We only have to
            // move if indicator bottom is below display bottom.
            (top_distance + indicator_height - menu_height).max(0)
        } else {
            // We need to move up
            top_distance
        };

        self.items
            .translate_mut(Point::new(0, -self.state.list_offset));
    }

    fn display_details<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let header = self.header(self.items.title_of(self.state.selected), display);

        let size = header.size();
        LinearLayout::vertical(
            Chain::new(header).append(
                TextBox::new(
                    self.items.details_of(self.state.selected),
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

impl<T, IT, VG, R, P, S> Menu<T, IT, VG, R, BinaryColor, P, S>
where
    T: AsRef<str>,
    IT: InteractionController,
    VG: ViewGroup + MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn display_list<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();
        let display_size = display_area.size();

        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);

        let menu_title = self.header(self.title.as_ref(), display);
        menu_title.draw(display)?;

        let menu_height = display_size.height - menu_title.size().height;

        // Height of the selected menu item
        let menuitem_height = self.items.bounds_of(self.state.selected).size().height;

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

        self.style.indicator.draw(
            menuitem_height,
            self.style.indicator.offset(&self.state.indicator_state) - self.state.list_offset
                + menu_title.size().height as i32,
            self.style
                .interaction
                .fill_area_width(&self.state.interaction_state, menu_list_width),
            &mut display.clipped(&menu_display_area),
            &self.items,
            &self.style,
            &self.state.indicator_state,
        )?;

        if draw_scrollbar {
            let scale = |value| (value * menu_height / list_height) as i32;

            let scrollbar_height = scale(menu_height).max(1);
            let mut scrollbar_display = display.cropped(&scrollbar_area);

            // Start scrollbar from y=1, so we have a margin on top instead of bottom
            Line::new(Point::new(0, 1), Point::new(0, scrollbar_height))
                .into_styled(thin_stroke)
                .translate(Point::new(1, scale(self.state.list_offset as u32)))
                .draw(&mut scrollbar_display)?;
        }

        Ok(())
    }
}

impl<T, IT, VG, R, P, S> Drawable for Menu<T, IT, VG, R, BinaryColor, P, S>
where
    T: AsRef<str>,
    IT: InteractionController,
    VG: ViewGroup + MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        if self.state.display_mode.is_details() && self.selected_has_details() {
            self.display_details(display)
        } else {
            self.display_list(display)
        }
    }
}
