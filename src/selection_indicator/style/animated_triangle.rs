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
    theme::Theme,
};

#[derive(Clone, Copy)]
pub struct AnimatedTriangle<T = BinaryColor> {
    period: i32,
    theme: T,
}

impl<T> AnimatedTriangle<T> {
    pub const fn new(period: i32, theme: T) -> Self {
        Self { period, theme }
    }
}

#[derive(Default, Clone, Copy)]
pub struct State {
    current: i32,
}

impl<T> IndicatorStyle for AnimatedTriangle<T>
where
    T: Theme,
{
    type Shape = Arrow<T::Color>;
    type State = State;
    type Theme = T;

    fn on_target_changed(&self, state: &mut Self::State) {
        state.current = 0;
    }

    fn update(&self, state: &mut Self::State, input_state: InputState) {
        state.current = if input_state == InputState::Idle {
            (state.current + 1) % self.period
        } else {
            0
        };
    }

    fn padding(&self, _state: &Self::State, height: i32) -> Insets {
        Insets {
            left: height / 2 + 1,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        let max_offset = Self::Shape::tip_width(bounds);

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

        Self::Shape::new(bounds, fill_width, self.color(state)).translate(Point::new(-offset, 0))
    }

    fn color(&self, _state: &Self::State) -> <Self::Theme as Theme>::Color {
        self.theme.selection_color()
    }

    fn draw<D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        D: DrawTarget<Color = <Self::Theme as Theme>::Color>,
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
