use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, PixelColor, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle, StyledDrawable},
    Drawable,
};

use crate::MenuStyle;

pub mod animated;
pub mod simple;

pub enum IndicatorStyle {
    Line,
    // Border,
}

impl IndicatorStyle {
    fn margin(&self) -> Size {
        match self {
            IndicatorStyle::Line => Size::new(2, 0),
            // IndicatorStyle::Border => Size::new(2, 1),
        }
    }
}

impl IndicatorStyle {
    fn draw<D>(&self, fill_width: u32, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();
        let fill = Rectangle::new(
            display_area.top_left,
            Size::new(fill_width, display_area.size.height),
        );
        match self {
            IndicatorStyle::Line => Rectangle::new(fill.top_left, fill.size + Size::new(1, 0))
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(display),
            // IndicatorStyle::Border => {
            //     display_area
            //         .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            //         .draw(display)?;
            //     fill.into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            //         .draw(display)
            // }
        }
    }
}

pub trait SelectionIndicator: Sized {
    type Color: PixelColor;

    fn update_target(&mut self, y: i32);

    fn offset(&self) -> i32;

    fn update(&mut self);

    fn draw<D>(
        &self,
        indicator_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &mut D,
        items: &impl StyledDrawable<MenuStyle<Self::Color>, Color = Self::Color, Output = ()>,
        style: &MenuStyle<Self::Color>,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}
