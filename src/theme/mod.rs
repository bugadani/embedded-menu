use embedded_graphics::{pixelcolor::BinaryColor, prelude::PixelColor};

pub trait Theme: Copy {
    type Color: PixelColor + Default;

    fn text_color(&self) -> Self::Color;
    fn selected_text_color(&self) -> Self::Color;
    fn selection_color(&self) -> Self::Color;
}

impl Theme for BinaryColor {
    type Color = BinaryColor;

    fn text_color(&self) -> Self::Color {
        *self
    }

    fn selected_text_color(&self) -> Self::Color {
        self.invert()
    }

    fn selection_color(&self) -> Self::Color {
        *self
    }
}
