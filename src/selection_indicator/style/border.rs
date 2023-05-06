use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};

use crate::selection_indicator::{style::IndicatorStyle, Insets};

#[derive(Clone, Copy)]
pub struct Border;

impl IndicatorStyle for Border {
    type Shape = Rectangle;
    type State = ();

    fn margin(&self, _state: &Self::State, _height: u32) -> Insets {
        Insets::new(2, 1, 1, 1)
    }

    fn shape(&self, _state: &Self::State, bounds: Rectangle, _fill_width: u32) -> Self::Shape {
        bounds
    }

    fn draw<D>(
        &self,
        _state: &Self::State,
        fill_width: u32,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        display_area
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(display)?;

        Rectangle::new(
            display_area.top_left,
            Size::new(fill_width, display_area.size.height),
        )
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)
    }
}
