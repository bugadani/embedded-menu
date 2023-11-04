use embedded_graphics::{
    pixelcolor::BinaryColor,
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
pub struct Rectangle<T = BinaryColor> {
    theme: T,
}

impl<T> Rectangle<T> {
    pub fn new(theme: T) -> Self {
        Self { theme }
    }
}

impl<T> IndicatorStyle for Rectangle<T>
where
    T: Theme,
{
    type Theme = T;
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

        shape
            .into_styled(PrimitiveStyle::with_fill(self.theme.selection_color()))
            .draw(display)?;

        Ok(shape)
    }
}
