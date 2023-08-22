use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};

use crate::{
    interaction::InputState,
    selection_indicator::{
        style::{interpolate, IndicatorStyle},
        Insets,
    },
};

#[derive(Clone, Copy)]
pub struct Line;

impl IndicatorStyle for Line {
    type Shape = Rectangle;
    type State = ();

    fn padding(&self, _state: &Self::State, _height: u32) -> Insets {
        Insets {
            left: 2,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        Rectangle::new(
            bounds.top_left,
            Size::new(fill_width.max(1), bounds.size.height),
        )
    }

    fn draw<D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        display: &mut D,
    ) -> Result<u32, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        self.shape(state, display_area, fill_width)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(display)?;

        Ok(fill_width)
    }
}
