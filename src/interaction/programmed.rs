use crate::interaction::{InteractionController, InteractionType};

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor, Drawable};

pub struct Programmed;

impl InteractionController for Programmed {
    type Input = InteractionType;

    fn reset(&mut self) {}
    fn fill_area_width(&self, _max: u32) -> u32 {
        0
    }
    fn update(&mut self, action: Self::Input) -> Option<InteractionType> {
        Some(action)
    }
}

impl Drawable for Programmed {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, _display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        Ok(())
    }
}
