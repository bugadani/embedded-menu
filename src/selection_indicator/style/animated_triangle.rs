use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
    Drawable,
};
use embedded_layout::View;

use crate::selection_indicator::{
    style::{triangle::Arrow, IndicatorStyle},
    Insets,
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

    fn update(&self, state: &mut Self::State, fill_width: u32) {
        state.current = if fill_width == 0 {
            (state.current + 1) % self.period
        } else {
            0
        };
    }

    fn margin(&self, _state: &Self::State, height: u32) -> Insets {
        Insets::new(height as i32 / 2 + 1, 0, 0, 0)
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

    fn draw<D>(&self, state: &Self::State, fill_width: u32, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        self.shape(state, display_area, fill_width).draw(display)?;

        Ok(())
    }
}
