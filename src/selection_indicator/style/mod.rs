use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::DrawTarget,
    primitives::{ContainsPoint, Rectangle},
};

use crate::selection_indicator::Insets;

pub mod animated_triangle;
pub mod border;
pub mod line;
pub mod triangle;

pub trait IndicatorStyle {
    type Shape: ContainsPoint + Clone;
    type State: Default;

    fn on_target_changed(&self, _state: &mut Self::State) {}
    fn update(&self, _state: &mut Self::State, _fill_width: u32) {}
    fn margin(&self, state: &Self::State, height: u32) -> Insets;
    fn shape(&self, state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape;
    fn draw<D>(
        &self,
        state: &Self::State,
        fill_width: u32,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;
}
