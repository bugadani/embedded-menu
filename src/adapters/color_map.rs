use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Dimensions, DrawTarget, PixelColor},
    primitives::{ContainsPoint, Rectangle},
    Pixel,
};

pub struct ColorMappingOverlay<'a, T, S, C> {
    parent: &'a mut T,
    area: S,
    on_color: C,
    off_color: C,
}

impl<T, S, C> Dimensions for ColorMappingOverlay<'_, T, S, C>
where
    T: Dimensions,
{
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box()
    }
}

impl<T, S, C> DrawTarget for ColorMappingOverlay<'_, T, S, C>
where
    T: DrawTarget<Color = C>,
    S: ContainsPoint,
    C: PixelColor,
{
    type Color = BinaryColor;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<BinaryColor>>,
    {
        self.parent
            .draw_iter(pixels.into_iter().map(|Pixel(pos, color)| {
                let color = if self.area.contains(pos) ^ (color == BinaryColor::On) {
                    self.on_color
                } else {
                    self.off_color
                };

                Pixel(pos, color)
            }))
    }
}

pub trait BinaryColorDrawTargetExt: Sized {
    fn map_colors<S, C>(&mut self, area: &S, on: C, off: C) -> ColorMappingOverlay<'_, Self, S, C>
    where
        S: Clone + ContainsPoint,
        C: PixelColor;
}

impl<T> BinaryColorDrawTargetExt for T
where
    T: DrawTarget,
{
    fn map_colors<S, C>(&mut self, area: &S, on: C, off: C) -> ColorMappingOverlay<'_, Self, S, C>
    where
        S: Clone + ContainsPoint,
        C: PixelColor,
    {
        ColorMappingOverlay {
            parent: self,
            area: area.clone(),
            on_color: on,
            off_color: off,
        }
    }
}
