use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
    transform::Transform,
    Drawable,
};

use crate::{
    interaction::InputState,
    selection_indicator::{
        style::{interpolate, triangle::Arrow, IndicatorStyle},
        Insets,
    },
};

#[derive(Clone, Copy)]
pub struct AnimatedTriangle {
    period: i32,
}

impl AnimatedTriangle {
    pub const fn new(period: i32) -> Self {
        Self { period }
    }
}

#[derive(Default, Clone, Copy)]
pub struct State {
    current: i32,
}

impl IndicatorStyle for AnimatedTriangle {
    type Shape = Arrow;
    type State = State;

    fn on_target_changed(&self, state: &mut Self::State) {
        state.current = 0;
    }

    fn update<R>(&self, state: &mut Self::State, input_state: InputState<R>) {
        state.current = if let InputState::Idle = input_state {
            (state.current + 1) % self.period
        } else {
            0
        };
    }

    fn padding(&self, _state: &Self::State, height: u32) -> Insets {
        Insets {
            left: height as i32 / 2 + 1,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        let max_offset = Arrow::tip_width(bounds);

        let half_move = self.period / 5;
        let rest = 3 * half_move;

        let offset = if state.current < rest {
            0
        } else if state.current < rest + half_move {
            state.current - rest
        } else {
            self.period - state.current
        };

        let offset = offset * max_offset / half_move;

        Arrow::new(bounds, fill_width).translate(Point::new(-offset, 0))
    }

    fn draw<D, R>(
        &self,
        state: &Self::State,
        input_state: InputState<R>,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        let shape = self.shape(state, display_area, fill_width);

        shape.draw(display)?;

        Ok(shape)
    }
}
