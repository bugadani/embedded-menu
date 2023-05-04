use core::marker::PhantomData;

use embedded_graphics::{
    prelude::{Dimensions, DrawTarget, PixelColor},
    primitives::Rectangle,
    Pixel,
};

pub struct ConstrainedDrawTarget<'a, C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    clipping_rect: Rectangle,
    parent: &'a mut D,
    _color: PhantomData<C>,
}

impl<'a, C, D> ConstrainedDrawTarget<'a, C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    pub fn new(parent: &'a mut D, bounds: Rectangle) -> Self {
        Self {
            parent,
            clipping_rect: bounds,
            _color: PhantomData,
        }
    }
}

impl<'a, C, D> Dimensions for ConstrainedDrawTarget<'a, C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box()
    }
}

impl<'a, C, D> DrawTarget for ConstrainedDrawTarget<'a, C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    type Color = C;
    type Error = D::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        self.parent.draw_iter(pixels.into_iter().filter_map(|px| {
            let point = px.0 + self.clipping_rect.top_left;
            if self.clipping_rect.contains(point) {
                Some(Pixel(point, px.1))
            } else {
                None
            }
        }))
    }
}
