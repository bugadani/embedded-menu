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
pub struct Border;

impl IndicatorStyle for Border {
    type Shape = Rectangle;
    type State = ();

    fn padding(&self, _state: &Self::State, _height: u32) -> Insets {
        Insets {
            left: 2,
            top: 1,
            right: 1,
            bottom: 1,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: Rectangle, _fill_width: u32) -> Self::Shape {
        bounds
    }

    fn draw<D, R>(
        &self,
        _state: &Self::State,
        input_state: InputState<R>,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        display_area
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)?;

        Rectangle::new(
            display_area.top_left,
            Size::new(fill_width, display_area.size.height),
        )
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)?;

        Ok(Rectangle::new(
            display_area.top_left,
            Size::new(fill_width, display_area.size.height),
        ))
    }
}
