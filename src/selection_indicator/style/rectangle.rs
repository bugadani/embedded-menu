use embedded_graphics::{
    prelude::DrawTarget,
    primitives::{Primitive, PrimitiveStyle, Rectangle as RectangleShape},
    Drawable,
};

use crate::{
    interaction::InputState,
    selection_indicator::{
        style::{interpolate, IndicatorStyle},
        Insets,
    },
    theme::Theme,
};

#[derive(Clone, Copy)]
pub struct Rectangle;

impl IndicatorStyle for Rectangle {
    type Shape = RectangleShape;
    type State = ();

    fn padding(&self, _state: &Self::State, _height: i32) -> Insets {
        Insets {
            left: 2,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: RectangleShape, _fill_width: u32) -> Self::Shape {
        bounds
    }

    fn draw<T, D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        theme: &T,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        T: Theme,
        D: DrawTarget<Color = T::Color>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        let shape = self.shape(state, display_area, fill_width);

        shape
            .into_styled(PrimitiveStyle::with_fill(theme.selection_color()))
            .draw(display)?;

        Ok(shape)
    }
}
