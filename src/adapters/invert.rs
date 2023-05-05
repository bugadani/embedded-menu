use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget},
    primitives::{ContainsPoint, Rectangle},
    Pixel,
};

pub struct ColorInvertingOverlay<'a, T, S> {
    parent: &'a mut T,
    area: S,
}

impl<T, S> Dimensions for ColorInvertingOverlay<'_, T, S>
where
    T: Dimensions,
{
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box()
    }
}

impl<T, S> DrawTarget for ColorInvertingOverlay<'_, T, S>
where
    T: DrawTarget<Color = BinaryColor>,
    S: ContainsPoint,
{
    type Color = BinaryColor;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.parent
            .draw_iter(pixels.into_iter().map(|Pixel(pos, color)| {
                let color = if self.area.contains(pos) {
                    color.invert()
                } else {
                    color
                };
                Pixel(pos, color)
            }))
    }
}

pub trait BinaryColorDrawTargetExt: Sized {
    fn invert_area<S>(&mut self, area: &S) -> ColorInvertingOverlay<'_, Self, S>
    where
        S: Clone + ContainsPoint;
}

impl<T> BinaryColorDrawTargetExt for T
where
    T: DrawTarget<Color = BinaryColor>,
{
    fn invert_area<S>(&mut self, area: &S) -> ColorInvertingOverlay<'_, Self, S>
    where
        S: Clone + ContainsPoint,
    {
        ColorInvertingOverlay {
            parent: self,
            area: area.clone(),
        }
    }
}
