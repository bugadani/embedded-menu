use embedded_graphics::{
    prelude::{DrawTarget, Point, Size},
    primitives::{ContainsPoint, Primitive, PrimitiveStyle, Rectangle as RectangleShape},
    transform::Transform,
    Drawable,
};
use embedded_layout::prelude::{horizontal::LeftToRight, vertical::Center, Align};

use crate::{
    interaction::InputState,
    selection_indicator::{
        style::{interpolate, IndicatorStyle},
        Insets,
    },
};

#[derive(Clone, Copy)]
pub struct Rectangle<C = BinaryColor> {
    color: C,
}

impl<C> IndicatorStyle for Rectangle<C>
where
    C: Copy,
{
    type Shape = Arrow<C>;
    type State = ();

    fn padding(&self, _state: &Self::State, height: i32) -> Insets {
        Insets {
            left: height / 2 + 1,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: RectangleShape, fill_width: u32) -> Self::Shape {
        Arrow::new(bounds, fill_width, self.color)
    }

    fn draw<D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        D: DrawTarget<Color = <Self::Color as Theme>::Color>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        let shape = self.shape(state, display_area, fill_width);

        shape.draw(display)?;

        Ok(shape)
    }
}
