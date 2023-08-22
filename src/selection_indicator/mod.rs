use crate::{
    adapters::invert::BinaryColorDrawTargetExt,
    collection::MenuItemCollection,
    interaction::{InputAdapter, InputState},
    margin::Insets,
    selection_indicator::style::IndicatorStyle,
    MenuStyle,
};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, DrawTargetExt, Point, Size},
    primitives::Rectangle,
};

pub mod style;

pub trait SelectionIndicatorController: Copy {
    type State: Default + Copy;

    fn update_target(&self, state: &mut Self::State, y: i32);
    fn jump_to_target(&self, state: &mut Self::State);
    fn offset(&self, state: &Self::State) -> i32;
    fn update(&self, state: &mut Self::State);
}

#[derive(Clone, Copy, Default)]
pub struct StaticState {
    y_offset: i32,
}

#[derive(Clone, Copy)]
pub struct StaticPosition;

impl SelectionIndicatorController for StaticPosition {
    type State = StaticState;

    fn update_target(&self, state: &mut Self::State, y: i32) {
        state.y_offset = y;
    }

    fn jump_to_target(&self, _state: &mut Self::State) {}

    fn offset(&self, state: &Self::State) -> i32 {
        state.y_offset
    }

    fn update(&self, _state: &mut Self::State) {}
}

#[derive(Clone, Copy)]
pub struct AnimatedPosition {
    frames: i32,
}

#[derive(Clone, Copy, Default)]
pub struct AnimatedState {
    current: i32,
    target: i32,
}

impl AnimatedPosition {
    pub const fn new(frames: i32) -> Self {
        Self { frames }
    }
}

impl SelectionIndicatorController for AnimatedPosition {
    type State = AnimatedState;

    fn update_target(&self, state: &mut Self::State, y: i32) {
        state.target = y;
    }

    fn jump_to_target(&self, state: &mut Self::State) {
        state.current = state.target;
    }

    fn offset(&self, state: &Self::State) -> i32 {
        state.current
    }

    fn update(&self, state: &mut Self::State) {
        let rounding = if state.current < state.target {
            self.frames - 1
        } else {
            1 - self.frames
        };

        let distance = state.target - state.current;
        state.current += (distance + rounding) / self.frames;
    }
}

pub struct State<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    position: P::State,
    state: S::State,
}

impl<P, S> Default for State<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn default() -> Self {
        Self {
            position: Default::default(),
            state: Default::default(),
        }
    }
}

impl<P, S> Clone for State<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<P, S> Copy for State<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Indicator<P, S> {
    pub controller: P,
    pub style: S,
}

impl<P, S> Indicator<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn offset(&self, state: &State<P, S>) -> i32 {
        self.controller.offset(&state.position)
    }

    pub fn change_selected_item(&self, pos: i32, state: &mut State<P, S>) {
        self.controller.update_target(&mut state.position, pos);
        self.style.on_target_changed(&mut state.state);
    }

    pub fn jump_to_target(&self, state: &mut State<P, S>) {
        self.controller.jump_to_target(&mut state.position);
    }

    pub fn update(&self, input_state: InputState, state: &mut State<P, S>) {
        self.controller.update(&mut state.position);
        self.style.update(&mut state.state, input_state);
    }

    pub fn item_height(&self, menuitem_height: u32, state: &State<P, S>) -> u32 {
        let indicator_insets = self.style.margin(&state.state, menuitem_height);
        (menuitem_height as i32 + indicator_insets.top + indicator_insets.bottom) as u32
    }

    pub fn draw<R, D, IT>(
        &self,
        selected_height: u32,
        screen_offset: i32,
        input_state: InputState,
        display: &mut D,
        items: &impl MenuItemCollection<R>,
        style: &MenuStyle<BinaryColor, S, IT, P>,
        state: &State<P, S>,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        IT: InputAdapter,
    {
        let Insets {
            left: margin_left,
            top: margin_top,
            right: margin_right,
            bottom: margin_bottom,
        } = self.style.margin(&state.state, selected_height);

        let fill_width = self.style.draw(
            &state.state,
            input_state,
            &mut display.cropped(&Rectangle::new(
                Point::new(0, screen_offset),
                Size::new(
                    display.bounding_box().size.width,
                    (selected_height as i32 + margin_top + margin_bottom) as u32,
                ),
            )),
        )?;

        let display_top_left = display.bounding_box().top_left;
        let display_size = display.bounding_box().size;

        let mut inverting = display.invert_area(&self.style.shape(
            &state.state,
            Rectangle::new(
                Point::new(0, screen_offset),
                Size::new(
                    fill_width,
                    (selected_height as i32 + margin_top + margin_bottom) as u32,
                ),
            ),
            fill_width,
        ));

        items.draw_styled(
            style,
            &mut inverting.cropped(&Rectangle::new(
                display_top_left + Point::new(margin_left, margin_top),
                Size::new(
                    (display_size.width as i32 - margin_left - margin_right) as u32,
                    display_size.height,
                ),
            )),
        )
    }
}
