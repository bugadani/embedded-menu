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
    current: i32,
}

impl AnimatedTriangle {
    pub fn new(period: i32) -> Self {
        Self { period, current: 0 }
    }
}

impl IndicatorStyle for AnimatedTriangle {
    type Shape = Arrow;

    fn on_target_changed(&mut self) {
        self.current = 0;
    }

    fn update(&mut self, fill_width: u32) {
        self.current = if fill_width == 0 {
            (self.current + 1) % self.period
        } else {
            0
        };
    }

    fn margin(&self, height: u32) -> Insets {
        Insets::new(height as i32 / 2 + 1, 0, 0, 0)
    }

    fn shape(&self, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        let max_offset = Arrow::tip_width(bounds);

        let half_move = self.period / 5;
        let rest = 3 * half_move;

        let offset = if self.current < rest {
            0
        } else if self.current < rest + half_move {
            self.current - rest
        } else {
            self.period - self.current
        };

        let offset = offset * max_offset / half_move;

        Arrow::new(bounds, fill_width).translate(Point::new(-offset, 0))
    }

    fn draw<D>(&self, fill_width: u32, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        self.shape(display_area, fill_width).draw(display)?;

        Ok(())
    }
}
