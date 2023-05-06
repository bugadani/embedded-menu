use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::DrawTarget,
    primitives::{ContainsPoint, Rectangle},
};

use crate::selection_indicator::Insets;

pub mod border;
pub mod line;
pub mod triangle;

pub trait IndicatorStyle {
    type Shape: ContainsPoint + Clone;

    fn margin(&self, height: u32) -> Insets;
    fn shape(&self, bounds: Rectangle, fill_width: u32) -> Self::Shape;
    fn draw<D>(&self, fill_width: u32, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;
}
