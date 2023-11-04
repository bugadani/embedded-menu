use crate::interaction::InputState;
use crate::margin::Insets;
use crate::selection_indicator::style::{interpolate, IndicatorStyle};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::Drawable;

pub struct Invert;

impl IndicatorStyle for Invert {
    type Shape = Rectangle;
    type State = ();

    fn padding(&self, _state: &Self::State, _height: i32) -> Insets {
        Insets {
            left: 2,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        Rectangle::new(bounds.top_left, Size::new(fill_width, bounds.size.height))
    }

    fn color(&self, _state: &Self::State) -> <Self::Color as Theme>::Color {
        unimplemented!()
    }

    fn draw<D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        let fill_width = display_area.size.width
            - if let InputState::InProgress(progress) = input_state {
                interpolate(progress as u32, 0, 255, 0, display_area.size.width)
            } else {
                0
            };

        let shape = self.shape(state, display_area, fill_width);

        shape
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(display)?;

        Ok(shape)
    }
}
