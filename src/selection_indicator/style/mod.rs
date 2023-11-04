use embedded_graphics::{
    prelude::DrawTarget,
    primitives::{ContainsPoint, Rectangle},
    transform::Transform,
};

use crate::{interaction::InputState, selection_indicator::Insets, theme::Theme};

pub mod animated_triangle;
pub mod border;
pub mod line;
pub mod rectangle;
pub mod triangle;

// Re-export the styles themselves to make them easier to use.
pub use animated_triangle::AnimatedTriangle;
pub use border::Border;
pub use line::Line;
pub use triangle::Triangle;

pub fn interpolate(value: u32, x_min: u32, x_max: u32, y_min: u32, y_max: u32) -> u32 {
    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    if x_range == 0 {
        y_min
    } else {
        let x = value - x_min;
        let y = x * y_range / x_range;

        y + y_min
    }
}

pub trait IndicatorStyle: Copy {
    type Shape: ContainsPoint + Transform + Clone;
    type State: Default + Copy;

    fn on_target_changed(&self, _state: &mut Self::State) {}
    fn update(&self, _state: &mut Self::State, _input_state: InputState) {}
    fn padding(&self, state: &Self::State, height: i32) -> Insets;
    fn shape(&self, state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape;
    fn draw<T, D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        theme: &T,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        T: Theme,
        D: DrawTarget<Color = T::Color>;
}

#[test]
fn interpolate_basic() {
    assert_eq!(interpolate(0, 0, 100, 0, 100), 0);
    assert_eq!(interpolate(50, 0, 100, 0, 100), 50);
    assert_eq!(interpolate(100, 0, 100, 0, 100), 100);
    assert_eq!(interpolate(100, 0, 10, 0, 100), 1000);
}
