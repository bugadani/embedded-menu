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
    interaction::{
        programmed::Programmed, Action, InputAdapter, InputAdapterSource, InputResult, InputState,
        Interaction,
    },
    selection_indicator::{
        style::{line::Line as LineIndicator, IndicatorStyle},
        AnimatedPosition, Indicator, SelectionIndicatorController, State as IndicatorState,
        StaticPosition,
    },
};
use core::marker::PhantomData;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{AnchorPoint, AnchorX, AnchorY},
    mono_font::{ascii::FONT_6X10, MonoFont, MonoTextStyle},
    pixelcolor::{BinaryColor, PixelColor, Rgb888},
    prelude::{Dimensions, DrawTargetExt, Point},
    primitives::{Line, Primitive, PrimitiveStyle},
    Drawable,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*, view_group::ViewGroup};
use embedded_text::{
    style::{HeightMode, TextBoxStyle},
    TextBox,
};

use crate::interaction::Navigation;
pub use embedded_menu_macros::{Menu, SelectValue};

/// Marker trait necessary to avoid a "conflicting implementations" error.
pub trait Marker {}

pub trait MenuItem<R>: Marker + View {
    fn interact(&mut self) -> R;
    fn set_style<C, S, IT, P>(&mut self, style: &MenuStyle<C, S, IT, P, R>)
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController;
    fn title(&self) -> &str;
    fn details(&self) -> &str;
    fn value(&self) -> &str;
    fn selectable(&self) -> bool {
        true
    }
    fn draw_styled<C, S, IT, P, DIS>(
        &self,
        style: &MenuStyle<C, S, IT, P, R>,
        display: &mut DIS,
    ) -> Result<(), DIS::Error>
    where
        C: PixelColor + From<Rgb888>,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
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
pub struct MenuStyle<C, S, IT, P, R>
where
    C: PixelColor,
    S: IndicatorStyle,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
{
    pub(crate) color: C,
    pub(crate) scrollbar: DisplayScrollbar,
    pub(crate) font: &'static MonoFont<'static>,
    pub(crate) title_font: &'static MonoFont<'static>,
    pub(crate) details_delay: Option<u16>,
    pub(crate) input_adapter: IT,
    pub(crate) indicator: Indicator<P, S>,
    _marker: PhantomData<R>,
}

impl<R> Default for MenuStyle<BinaryColor, LineIndicator, Programmed, StaticPosition, R> {
    fn default() -> Self {
        Self::new(BinaryColor::On)
    }
}

impl<C, R> MenuStyle<C, LineIndicator, Programmed, StaticPosition, R>
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
            input_adapter: Programmed,
            indicator: Indicator {
                style: LineIndicator,
                controller: StaticPosition,
            },
            _marker: PhantomData,
        }
    }
}

impl<C, S, IT, P, R> MenuStyle<C, S, IT, P, R>
where
    C: PixelColor,
    S: IndicatorStyle,
    IT: InputAdapterSource<R>,
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

    pub const fn with_selection_indicator<S2>(
        self,
        indicator_style: S2,
    ) -> MenuStyle<C, S2, IT, P, R>
    where
        S2: IndicatorStyle,
    {
        MenuStyle {
            color: self.color,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            details_delay: self.details_delay,
            input_adapter: self.input_adapter,
            indicator: Indicator {
                style: indicator_style,
                controller: self.indicator.controller,
            },
            _marker: PhantomData,
        }
    }

    pub const fn with_input_adapter<IT2>(self, input_adapter: IT2) -> MenuStyle<C, S, IT2, P, R>
    where
        IT2: InputAdapterSource<R>,
    {
        MenuStyle {
            input_adapter,
            color: self.color,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            details_delay: self.details_delay,
            indicator: self.indicator,
            _marker: PhantomData,
        }
    }

    pub const fn with_animated_selection_indicator(
        self,
        frames: i32,
    ) -> MenuStyle<C, S, IT, AnimatedPosition, R> {
        MenuStyle {
            input_adapter: self.input_adapter,
            color: self.color,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            details_delay: self.details_delay,
            indicator: Indicator {
                style: self.indicator.style,
                controller: AnimatedPosition::new(frames),
            },
            _marker: PhantomData,
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
    IT: InputAdapter,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    selected: usize,
    list_offset: i32,
    display_mode: MenuDisplayMode,
    interaction_state: IT::State,
    indicator_state: IndicatorState<P, S>,
    last_input_state: InputState,
}

impl<IT, P, S> Default for MenuState<IT, P, S>
where
    IT: InputAdapter,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn default() -> Self {
        Self {
            selected: 0,
            list_offset: Default::default(),
            display_mode: Default::default(),
            interaction_state: Default::default(),
            indicator_state: Default::default(),
            last_input_state: InputState::Idle,
        }
    }
}

impl<IT, P, S> Clone for MenuState<IT, P, S>
where
    IT: InputAdapter,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<IT, P, S> Copy for MenuState<IT, P, S>
where
    IT: InputAdapter,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
}

impl<IT, P, S> MenuState<IT, P, S>
where
    IT: InputAdapter,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn reset_interaction(&mut self) {
        self.interaction_state = Default::default();
    }

    fn set_selected_item<ITS, R, C>(
        &mut self,
        selected: usize,
        items: &impl MenuItemCollection<R>,
        style: &MenuStyle<C, S, ITS, P, R>,
    ) where
        ITS: InputAdapterSource<R, InputAdapter = IT>,
        C: PixelColor,
    {
        let selected =
            Navigation::JumpTo(selected)
                .calculate_selection(self.selected, items.count(), |i| items.selectable(i));
        self.selected = selected;

        let selected_offset = items.bounds_of(selected).top_left.y;

        style
            .indicator
            .change_selected_item(selected_offset, &mut self.indicator_state);
    }
}

pub struct Menu<T, IT, VG, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    _return_type: PhantomData<R>,
    title: T,
    items: VG,
    style: MenuStyle<C, S, IT, P, R>,
    state: MenuState<IT::InputAdapter, P, S>,
}

impl<T, R, C, S> Menu<T, Programmed, NoItems, R, C, StaticPosition, S>
where
    T: AsRef<str>,
    C: PixelColor,
    S: IndicatorStyle,
{
    pub fn new(title: T) -> MenuBuilder<T, Programmed, NoItems, R, C, StaticPosition, S>
    where
        MenuStyle<C, S, Programmed, StaticPosition, R>: Default,
    {
        Self::with_style(title, MenuStyle::default())
    }
}

impl<T, IT, R, C, P, S> Menu<T, IT, NoItems, R, C, P, S>
where
    T: AsRef<str>,
    C: PixelColor,
    S: IndicatorStyle,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
{
    pub fn with_style(
        title: T,
        style: MenuStyle<C, S, IT, P, R>,
    ) -> MenuBuilder<T, IT, NoItems, R, C, P, S> {
        MenuBuilder::new(title, style)
    }
}

impl<T, IT, VG, R, C, P, S> Menu<T, IT, VG, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: MenuItemCollection<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn selected_has_details(&self) -> bool {
        !self.items.details_of(self.state.selected).is_empty()
    }

    fn reset_display_state(&mut self) {
        if let Some(threshold) = self.style.details_delay {
            self.state.display_mode = MenuDisplayMode::List(threshold);
        }
    }

    pub fn interact(&mut self, input: <IT::InputAdapter as InputAdapter>::Input) -> Option<R> {
        let input = self
            .style
            .input_adapter
            .adapter()
            .handle_input(&mut self.state.interaction_state, input);

        if !matches!(input, InputResult::StateUpdate(InputState::Idle)) {
            // If anything happens, exit Details view
            self.reset_display_state();
        }

        self.state.last_input_state = match input {
            InputResult::Interaction(_) => InputState::Idle,
            InputResult::StateUpdate(state) => state,
        };

        match input {
            InputResult::Interaction(interaction) => match interaction {
                Interaction::Navigation(navigation) => {
                    let count = self.items.count();
                    let new_selected =
                        navigation.calculate_selection(self.state.selected, count, |i| {
                            self.items.selectable(i)
                        });
                    if new_selected != self.state.selected {
                        self.state
                            .set_selected_item(new_selected, &self.items, &self.style);
                    }
                    None
                }
                Interaction::Action(Action::Select) => {
                    let value = self.items.interact_with(self.state.selected);
                    Some(value)
                }
                Interaction::Action(Action::Return(value)) => Some(value),
            },
            _ => None,
        }
    }

    pub fn state(&self) -> MenuState<IT::InputAdapter, P, S> {
        self.state
    }
}

impl<T, IT, VG, R, C, P, S> Menu<T, IT, VG, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: ViewGroup + MenuItemCollection<R>,
    C: PixelColor + From<Rgb888>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn header<'t>(
        &self,
        title: &'t str,
        display: &impl Dimensions,
    ) -> Option<impl View + 't + Drawable<Color = C>>
    where
        C: 't,
    {
        if title.is_empty() {
            return None;
        }

        let display_area = display.bounding_box();

        let text_style = self.style.title_style();
        let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);
        let header = LinearLayout::vertical(
            Chain::new(TextBox::with_textbox_style(
                title,
                display_area,
                text_style,
                TextBoxStyle::with_height_mode(HeightMode::FitToText),
            ))
            .append(
                // Bottom border
                Line::new(
                    display_area.top_left,
                    display_area.anchor_point(AnchorPoint::TopRight),
                )
                .into_styled(thin_stroke),
            ),
        )
        .arrange();

        Some(header)
    }

    fn top_offset(&self) -> i32 {
        self.style.indicator.offset(&self.state.indicator_state) - self.state.list_offset
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

        // animations
        self.style
            .indicator
            .update(self.state.last_input_state, &mut self.state.indicator_state);

        // Ensure selection indicator is always visible by moving the menu list.
        let top_distance = self.top_offset();

        let list_offset_change = if top_distance > 0 {
            let display_height = display.bounding_box().size().height as i32;

            let header_height = if let Some(header) = self.header(self.title.as_ref(), display) {
                header.size().height as i32
            } else {
                0
            };

            let selected_height = MenuItemCollection::bounds_of(&self.items, self.state.selected)
                .size()
                .height as i32;
            let indicator_height = self
                .style
                .indicator
                .item_height(selected_height, &self.state.indicator_state);

            // Indicator is below display top. We only have to
            // move if indicator bottom is below display bottom.
            (top_distance + indicator_height + header_height - display_height).max(0)
        } else {
            // We need to move up
            top_distance
        };

        // Move menu list.
        self.state.list_offset += list_offset_change;
    }

    fn display_details<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let display_area = display.bounding_box();

        let title = self.items.title_of(self.state.selected);
        let details = self.items.details_of(self.state.selected);
        let header = self.header(title, display);

        let content_area = if let Some(header) = header {
            header.draw(display)?;
            display_area.resized_height(
                display_area.size().height - header.size().height,
                AnchorY::Bottom,
            )
        } else {
            display_area
        };

        let character_style = self.style.text_style();
        TextBox::new(details, content_area, character_style).draw(display)?;

        Ok(())
    }
}

impl<T, IT, VG, R, P, S> Menu<T, IT, VG, R, BinaryColor, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: ViewGroup + MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn display_list<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        let header = self.header(self.title.as_ref(), display);
        let content_area = if let Some(header) = header {
            header.draw(display)?;
            display_area.resized_height(
                display_area.size().height - header.size().height,
                AnchorY::Bottom,
            )
        } else {
            display_area
        };

        let menu_height = content_area.size().height as i32;
        let list_height = self.items.bounds().size().height as i32;

        let draw_scrollbar = match self.style.scrollbar {
            DisplayScrollbar::Display => true,
            DisplayScrollbar::Hide => false,
            DisplayScrollbar::Auto => list_height > menu_height,
        };

        let menu_display_area = if draw_scrollbar {
            let scrollbar_area = content_area.resized_width(2, AnchorX::Right);
            let thin_stroke = PrimitiveStyle::with_stroke(self.style.color, 1);

            let scale = |value| value * menu_height / list_height;

            let scrollbar_height = scale(menu_height).max(1);
            let mut scrollbar_display = display.cropped(&scrollbar_area);

            // Start scrollbar from y=1, so we have a margin on top instead of bottom
            Line::new(Point::new(0, 1), Point::new(0, scrollbar_height))
                .into_styled(thin_stroke)
                .translate(Point::new(1, scale(self.state.list_offset)))
                .draw(&mut scrollbar_display)?;

            content_area.resized_width(
                content_area.size().width - scrollbar_area.size().width,
                AnchorX::Left,
            )
        } else {
            content_area
        };

        let selected_menuitem_height =
            MenuItemCollection::bounds_of(&self.items, self.state.selected)
                .size()
                .height as i32;

        self.style.indicator.draw(
            selected_menuitem_height,
            self.top_offset(),
            self.state.last_input_state,
            &mut display
                .clipped(&menu_display_area)
                .cropped(&menu_display_area),
            &self.items,
            &self.style,
            &self.state,
        )?;

        Ok(())
    }
}

impl<T, IT, VG, R, P, S> Drawable for Menu<T, IT, VG, R, BinaryColor, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
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
