use crate::interaction::{InteractionController, InteractionType};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::Point,
    primitives::Line,
    primitives::{Primitive, PrimitiveStyle},
    Drawable,
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

impl Drawable for Programmed {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        Line::new(
            Point::zero(),
            Point::new(0, display.bounding_box().size.height as i32 - 1),
        )
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(display)
    }
}
