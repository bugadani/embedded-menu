use crate::interaction::{InteractionController, InteractionType};
use embedded_layout::prelude::*;

use embedded_graphics::{
    pixelcolor::BinaryColor, primitives::Line, style::PrimitiveStyle, DrawTarget,
};

pub struct Programmed {}

impl Programmed {
    pub fn new() -> Self {
        Self {}
    }
}

impl InteractionController for Programmed {
    type Input = InteractionType;

    fn reset(&mut self) {}
    fn update(&mut self, action: Self::Input) -> InteractionType {
        action
    }
}

impl Drawable<BinaryColor> for &Programmed {
    fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
        Line::new(
            Point::new(0, 0),
            Point::new(0, display.size().height as i32 - 1),
        )
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
    }
}
