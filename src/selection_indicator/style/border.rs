use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, PixelColor, Size},
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
pub struct Border<C = BinaryColor> {
    color: C,
}

impl<C> IndicatorStyle for Border<C>
where
    C: Copy + PixelColor,
{
    type Shape = Rectangle;
    type State = ();
    type Color = C;

    fn padding(&self, _state: &Self::State, _height: i32) -> Insets {
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

    fn color(&self, _state: &Self::State) -> Self::Color {
        self.color
    }

    fn draw<D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        display_area
            .into_styled(PrimitiveStyle::with_stroke(self.color(state), 1))
            .draw(display)?;

        Rectangle::new(
            display_area.top_left,
            Size::new(fill_width, display_area.size.height),
        )
        .into_styled(PrimitiveStyle::with_fill(self.color(state)))
        .draw(display)?;

        Ok(Rectangle::new(
            display_area.top_left,
            Size::new(fill_width, display_area.size.height),
        ))
    }
}
