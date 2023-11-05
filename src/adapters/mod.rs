use embedded_graphics::{
    prelude::{Dimensions, DrawTarget, PixelColor},
    primitives::Rectangle,
    Pixel,
};

pub mod color_map;

/// An object-safe abstraction over a display.
pub trait Canvas<C: PixelColor> {
    fn draw_pixel(&mut self, px: Pixel<C>) -> Result<(), ()>;
    fn bounds(&self) -> Rectangle;
}

impl<C: PixelColor> DrawTarget for &mut dyn Canvas<C> {
    type Color = C;
    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            self.draw_pixel(pixel)?;
        }

        Ok(())
    }
}

impl<C: PixelColor> Dimensions for &mut dyn Canvas<C> {
    fn bounding_box(&self) -> Rectangle {
        self.bounds()
    }
}

/// Turns a [`DrawTarget`]` into something object safe through [`Canvas`].
pub struct DrawTargetWrapper<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    display: D,
    last_result: Result<(), D::Error>,
    _marker: core::marker::PhantomData<C>,
}

impl<'a, C, D> DrawTargetWrapper<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    pub fn new(display: D) -> Self {
        Self {
            display,
            last_result: Ok(()),
            _marker: core::marker::PhantomData,
        }
    }

    pub fn into_result(self) -> Result<(), D::Error> {
        self.last_result
    }

    fn update_result(&mut self, result: Result<(), D::Error>) -> Result<(), ()> {
        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                self.last_result = Err(err);
                Err(())
            }
        }
    }
}

impl<C, D> Canvas<C> for DrawTargetWrapper<C, D>
where
    C: PixelColor,
    D: DrawTarget<Color = C>,
{
    fn draw_pixel(&mut self, px: Pixel<C>) -> Result<(), ()> {
        let result = self.display.draw_iter(core::iter::once(px));
        self.update_result(result)
    }

    fn bounds(&self) -> Rectangle {
        self.display.bounding_box()
    }
}
