use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};

use crate::selection_indicator::{style::IndicatorStyle, Insets};

#[derive(Clone, Copy)]
pub struct Line;

impl IndicatorStyle for Line {
    type Shape = Rectangle;

    fn margin(&self, _height: u32) -> Insets {
        Insets::new(2, 0, 0, 0)
    }

    fn shape(&self, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        Rectangle::new(
            bounds.top_left,
            Size::new(fill_width.max(1), bounds.size.height),
        )
    }

    fn draw<D>(&self, fill_width: u32, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        self.shape(display_area, fill_width)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(display)
    }
}
