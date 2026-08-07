#![cfg_attr(not(test), no_std)]
#![deny(clippy::missing_const_for_fn)]
pub mod adapters;
pub mod builder;
pub mod collection;
pub mod interaction;
pub mod items;
pub mod margin;
pub mod selection_indicator;
pub mod theme;

use crate::{
    builder::MenuBuilder,
    collection::MenuItemCollection,
    interaction::{
        programmed::Programmed, Action, InputAdapter, InputAdapterSource, InputResult, InputState,
        Interaction, Navigation,
    },
    selection_indicator::{
        style::{line::Line as LineIndicator, IndicatorStyle},
        AnimatedPosition, Indicator, SelectionIndicatorController, State as IndicatorState,
        StaticPosition,
    },
    theme::Theme,
};
use core::marker::PhantomData;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{AnchorPoint, AnchorX, AnchorY},
    mono_font::{ascii::FONT_6X10, MonoFont, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTargetExt, Point},
    primitives::{Line, Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*, view_group::ViewGroup};
use embedded_text::{
    style::{HeightMode, TextBoxStyle},
    TextBox,
};

pub use embedded_menu_macros::SelectValue;

#[derive(Copy, Clone, Debug)]
pub enum DisplayScrollbar {
    Display,
    Hide,
    Auto,
}

#[derive(Copy, Clone, Debug)]
pub struct MenuStyle<S, IT, P, R, T> {
    pub(crate) theme: T,
    pub(crate) scrollbar: DisplayScrollbar,
    pub(crate) font: &'static MonoFont<'static>,
    pub(crate) title_font: &'static MonoFont<'static>,
    pub(crate) input_adapter: IT,
    pub(crate) indicator: Indicator<P, S>,
    _marker: PhantomData<R>,
}

impl<R> Default for MenuStyle<LineIndicator, Programmed, StaticPosition, R, BinaryColor> {
    fn default() -> Self {
        Self::new(BinaryColor::On)
    }
}

impl<T, R> MenuStyle<LineIndicator, Programmed, StaticPosition, R, T>
where
    T: Theme,
{
    pub const fn new(theme: T) -> Self {
        Self {
            theme,
            scrollbar: DisplayScrollbar::Auto,
            font: &FONT_6X10,
            title_font: &FONT_6X10,
            input_adapter: Programmed,
            indicator: Indicator {
                style: LineIndicator,
                controller: StaticPosition,
            },
            _marker: PhantomData,
        }
    }
}

impl<S, IT, P, R, T> MenuStyle<S, IT, P, R, T>
where
    S: IndicatorStyle,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
    T: Theme,
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

    pub const fn with_selection_indicator<S2>(
        self,
        indicator_style: S2,
    ) -> MenuStyle<S2, IT, P, R, T>
    where
        S2: IndicatorStyle,
    {
        MenuStyle {
            theme: self.theme,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            input_adapter: self.input_adapter,
            indicator: Indicator {
                style: indicator_style,
                controller: self.indicator.controller,
            },
            _marker: PhantomData,
        }
    }

    pub const fn with_input_adapter<IT2>(self, input_adapter: IT2) -> MenuStyle<S, IT2, P, R, T>
    where
        IT2: InputAdapterSource<R>,
    {
        MenuStyle {
            theme: self.theme,
            input_adapter,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            indicator: self.indicator,
            _marker: PhantomData,
        }
    }

    pub const fn with_animated_selection_indicator(
        self,
        frames: i32,
    ) -> MenuStyle<S, IT, AnimatedPosition, R, T> {
        MenuStyle {
            theme: self.theme,
            input_adapter: self.input_adapter,
            scrollbar: self.scrollbar,
            font: self.font,
            title_font: self.title_font,
            indicator: Indicator {
                style: self.indicator.style,
                controller: AnimatedPosition::new(frames),
            },
            _marker: PhantomData,
        }
    }

    pub const fn text_style(&self) -> MonoTextStyle<'static, BinaryColor> {
        MonoTextStyle::new(self.font, BinaryColor::On)
    }

    pub fn title_style(&self) -> MonoTextStyle<'static, T::Color> {
        MonoTextStyle::new(self.title_font, self.theme.text_color())
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

    /// Points the selection indicator at `selected`, without restarting the indicator's animation.
    ///
    /// Use this when the selection itself doesn't change, but the items may have been laid out
    /// anew, e.g. when the menu is rebuilt from an existing state.
    fn relayout<ITS, R, T>(
        &mut self,
        selected: usize,
        items: &impl MenuItemCollection<R>,
        style: &MenuStyle<S, ITS, P, R, T>,
    ) where
        ITS: InputAdapterSource<R, InputAdapter = IT>,
        T: Theme,
    {
        let selected =
            Navigation::JumpTo(selected)
                .calculate_selection(self.selected, items.count(), |i| items.selectable(i));
        self.selected = selected;

        let selected_offset = if items.count() == 0 {
            0
        } else {
            items.bounds_of(selected).top_left.y
        };

        style
            .indicator
            .update_target(selected_offset, &mut self.indicator_state);
    }

    fn set_selected_item<ITS, R, T>(
        &mut self,
        selected: usize,
        items: &impl MenuItemCollection<R>,
        style: &MenuStyle<S, ITS, P, R, T>,
    ) where
        ITS: InputAdapterSource<R, InputAdapter = IT>,
        T: Theme,
    {
        self.relayout(selected, items, style);
        style.indicator.on_target_changed(&mut self.indicator_state);
    }
}

pub struct Menu<T, IT, VG, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: Theme,
{
    _return_type: PhantomData<R>,
    title: T,
    items: VG,
    style: MenuStyle<S, IT, P, R, C>,
    state: MenuState<IT::InputAdapter, P, S>,
    dirty: bool,
}

impl<T, R, S, C> Menu<T, Programmed, NoItems, R, StaticPosition, S, C>
where
    T: AsRef<str>,
    S: IndicatorStyle,
    C: Theme,
{
    /// Creates a new menu builder with the given title.
    pub fn build(title: T) -> MenuBuilder<T, Programmed, NoItems, R, StaticPosition, S, C>
    where
        MenuStyle<S, Programmed, StaticPosition, R, C>: Default,
    {
        Self::with_style(title, MenuStyle::default())
    }
}

impl<T, IT, R, P, S, C> Menu<T, IT, NoItems, R, P, S, C>
where
    T: AsRef<str>,
    S: IndicatorStyle,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
    C: Theme,
{
    /// Creates a new menu builder with the given title and style.
    pub const fn with_style(
        title: T,
        style: MenuStyle<S, IT, P, R, C>,
    ) -> MenuBuilder<T, IT, NoItems, R, P, S, C> {
        MenuBuilder::new(title, style)
    }
}

impl<T, IT, VG, R, P, S, C> Menu<T, IT, VG, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: Theme,
{
    pub fn interact(&mut self, input: <IT::InputAdapter as InputAdapter>::Input) -> Option<R> {
        let input = self
            .style
            .input_adapter
            .adapter()
            .handle_input(&mut self.state.interaction_state, input);

        let last_input_state = match input {
            InputResult::None => self.state.last_input_state, // No change
            InputResult::Interaction(_) => InputState::Idle,
            InputResult::StateUpdate(state) => state,
        };

        // The input state drives how the selection indicator is filled in, so a change to it is
        // visible.
        if last_input_state != self.state.last_input_state {
            self.dirty = true;
        }
        self.state.last_input_state = last_input_state;

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
                        self.dirty = true;
                    }
                    None
                }
                Interaction::Action(Action::Select) => {
                    if self.items.count() == 0 {
                        return None;
                    }

                    // We can't tell whether the item's appearance changed, so assume it did.
                    self.dirty = true;
                    let value = self.items.interact_with(self.state.selected);
                    Some(value)
                }
                Interaction::Action(Action::Return(value)) => Some(value),
            },
            _ => None,
        }
    }

    pub const fn state(&self) -> MenuState<IT::InputAdapter, P, S> {
        self.state
    }
}

impl<T, IT, VG, R, P, S, C> Menu<T, IT, VG, R, P, S, C>
where
    T: AsRef<str>,
    R: Copy,
    IT: InputAdapterSource<R>,
    VG: MenuItemCollection<R>,
    C: Theme,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    /// Returns the value of the selected item, without interacting with it.
    ///
    /// # Panics
    ///
    /// Panics if the menu has no items, as there is no value to return.
    pub fn selected_value(&self) -> R {
        assert!(self.items.count() > 0, "the menu has no items");
        self.items.value_of(self.state.selected)
    }
}

impl<T, IT, VG, R, C, P, S> Menu<T, IT, VG, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: ViewGroup + MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: Theme,
{
    fn header<'t>(
        &self,
        title: &'t str,
        display_area: Rectangle,
    ) -> Option<impl View + 't + Drawable<Color = C::Color>>
    where
        C: Theme + 't,
    {
        if title.is_empty() {
            return None;
        }

        let text_style = self.style.title_style();
        let thin_stroke = PrimitiveStyle::with_stroke(self.style.theme.text_color(), 1);
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

    /// The height of the selected menu item, or 0 if the menu has no items.
    fn selected_item_height(&self) -> i32 {
        if self.items.count() == 0 {
            0
        } else {
            MenuItemCollection::bounds_of(&self.items, self.state.selected)
                .size()
                .height as i32
        }
    }

    /// Advances animations by a single frame, and returns whether the menu needs to be redrawn.
    ///
    /// Call this once per frame, and only draw the menu if it returns `true`:
    ///
    /// ```ignore
    /// if menu.update(&display) {
    ///     menu.draw(&mut display)?;
    ///     display.flush()?;
    /// }
    /// ```
    ///
    /// Each call reports the changes since the previous one, so ignoring a `true` means losing
    /// that frame.
    ///
    /// The returned value only accounts for state the menu owns. If you change anything else
    /// that the menu displays - the value of a menu item, or the contents of an item collection
    /// you own - draw the menu yourself; it has no way to observe those changes.
    pub fn update(&mut self, display: &impl Dimensions) -> bool {
        // animations
        let animation_changed = self
            .style
            .indicator
            .update(self.state.last_input_state, &mut self.state.indicator_state);

        // Ensure selection indicator is always visible by moving the menu list.
        let top_distance = self.top_offset();

        let list_offset_change = if top_distance > 0 {
            let display_area = display.bounding_box();
            let display_height = display_area.size().height as i32;

            let header_height = if let Some(header) = self.header(self.title.as_ref(), display_area)
            {
                header.size().height as i32
            } else {
                0
            };

            let selected_height = self.selected_item_height();
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

        if animation_changed || list_offset_change != 0 {
            self.dirty = true;
        }

        core::mem::take(&mut self.dirty)
    }
}

impl<T, IT, VG, R, C, P, S> Drawable for Menu<T, IT, VG, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: ViewGroup + MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: Theme,
{
    type Color = C::Color;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C::Color>,
    {
        let display_area = display.bounding_box();

        let header = self.header(self.title.as_ref(), display_area);
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

        // An empty list has nothing to scroll, and scaling by its height would divide by zero.
        let draw_scrollbar = list_height > 0
            && match self.style.scrollbar {
                DisplayScrollbar::Display => true,
                DisplayScrollbar::Hide => false,
                DisplayScrollbar::Auto => list_height > menu_height,
            };

        let menu_display_area = if draw_scrollbar {
            let scrollbar_area = content_area.resized_width(2, AnchorX::Right);
            let thin_stroke = PrimitiveStyle::with_stroke(self.style.theme.text_color(), 1);

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

        self.style.indicator.draw(
            self.selected_item_height(),
            self.top_offset(),
            self.state.last_input_state,
            display.cropped(&menu_display_area),
            &self.items,
            &self.style,
            &self.state,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor, Drawable};

    use crate::{
        interaction::{Action, Interaction, Navigation},
        items::MenuItem,
        selection_indicator::style::AnimatedTriangle,
        DisplayScrollbar, Menu, MenuStyle,
    };

    /// The menu is wider and taller than a `MockDisplay`, and freely overdraws.
    fn test_display() -> MockDisplay<BinaryColor> {
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);
        display
    }

    #[test]
    fn a_menu_without_items_can_be_used() {
        let mut items: [MenuItem<&str, (), &str, true>; 0] = [];
        let mut menu = Menu::with_style(
            "Title",
            MenuStyle::new(BinaryColor::On).with_scrollbar_style(DisplayScrollbar::Display),
        )
        .add_menu_items(&mut items)
        .build();

        let mut display = test_display();
        menu.update(&display);
        menu.draw(&mut display).unwrap();

        assert_eq!(
            menu.interact(Interaction::Navigation(Navigation::Next)),
            None
        );
        assert_eq!(menu.interact(Interaction::Action(Action::Select)), None);
    }

    #[test]
    fn a_value_wider_than_the_display_can_be_drawn() {
        let mut menu = Menu::with_style("Title", MenuStyle::new(BinaryColor::On))
            .add_item("Title", "a marker far wider than the display", |_| ())
            .build();

        let mut display = test_display();
        menu.update(&display);
        menu.draw(&mut display).unwrap();
    }

    #[test]
    fn animated_position_settles_and_input_wakes_it_up() {
        let style = MenuStyle::new(BinaryColor::On).with_animated_selection_indicator(10);
        let mut menu = Menu::with_style("Title", style)
            .add_item("Item 1", "", |_| ())
            .add_item("Item 2", "", |_| ())
            .add_item("Item 3", "", |_| ())
            .build();

        let mut display = test_display();

        // A menu that has never been drawn must be drawn.
        assert!(menu.update(&display));
        menu.draw(&mut display).unwrap();
        assert!(!menu.update(&display));

        menu.interact(Interaction::Navigation(Navigation::Next));

        let mut frames = 0;
        while menu.update(&display) {
            menu.draw(&mut display).unwrap();
            frames += 1;
            assert!(frames < 100, "the indicator animation did not settle");
        }
        assert!(frames > 1, "the indicator did not animate");
    }

    #[test]
    fn selecting_an_item_requires_a_redraw() {
        let style = MenuStyle::new(BinaryColor::On);
        let mut menu = Menu::with_style("Title", style)
            .add_item("Item 1", false, |_| ())
            .add_item("Item 2", false, |_| ())
            .build();

        let mut display = test_display();
        assert!(menu.update(&display));
        menu.draw(&mut display).unwrap();
        assert!(!menu.update(&display));

        menu.interact(Interaction::Action(Action::Select));
        assert!(menu.update(&display));
    }

    #[test]
    fn animated_triangle_only_redraws_while_the_arrow_moves() {
        let period = 10;
        let style =
            MenuStyle::new(BinaryColor::On).with_selection_indicator(AnimatedTriangle::new(period));
        let mut menu = Menu::with_style("Title", style)
            .add_item("Item 1", "", |_| ())
            .add_item("Item 2", "", |_| ())
            .build();

        let mut display = test_display();
        while menu.update(&display) {
            menu.draw(&mut display).unwrap();
        }

        // The arrow rests at its resting position for 3/5 of every period, and only the frames
        // where it actually moves need to be drawn.
        let redraws = (0..period)
            .filter(|_| {
                let redraw = menu.update(&display);
                if redraw {
                    menu.draw(&mut display).unwrap();
                }
                redraw
            })
            .count();

        assert_eq!(redraws, 4);
    }

    #[test]
    fn rebuilding_a_menu_does_not_restart_the_indicator_animation() {
        let period = 10;
        let new_menu = || {
            let style = MenuStyle::new(BinaryColor::On)
                .with_selection_indicator(AnimatedTriangle::new(period));
            Menu::with_style("Title", style)
                .add_item("Item 1", "", |_| ())
                .add_item("Item 2", "", |_| ())
        };

        let mut menu = new_menu().build();
        let mut display = test_display();

        // Rebuild the menu more often than the arrow's resting phase (3/5 of the period) lasts.
        // The animation must still reach the moving phase.
        let rebuild_period = 3 * period / 5 - 1;

        let mut moved = 0;
        for frame in 0..3 * period {
            let rebuilt = frame % rebuild_period == 0;
            if rebuilt {
                menu = new_menu().build_with_state(menu.state());
            }

            if menu.update(&display) {
                menu.draw(&mut display).unwrap();
                // A rebuilt menu is always redrawn, so those frames say nothing about the arrow.
                if !rebuilt {
                    moved += 1;
                }
            }
        }

        assert!(moved > 0, "the arrow did not animate");
    }
}
