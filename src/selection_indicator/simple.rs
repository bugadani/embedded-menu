use embedded_graphics::{
    draw_target::Cropped,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, DrawTargetExt, Point, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};

use crate::{
    adapters::invert::{BinaryColorDrawTargetExt, ColorInvertingOverlay},
    selection_indicator::SelectionIndicator,
    Animated,
};

pub struct SimpleSelectionIndicator {
    y_offset: Animated,
}

impl SelectionIndicator for SimpleSelectionIndicator {
    type Color = BinaryColor;
    type Display<'a, D: DrawTarget<Color = Self::Color> + 'a> = ColorInvertingOverlay<'a, D>;

    fn new(anim_frames: i32) -> Self {
        Self {
            y_offset: Animated::new(0, anim_frames),
        }
    }

    fn update_target(&mut self, y: i32) {
        self.y_offset.update_target(y);
    }

    fn offset(&self) -> i32 {
        self.y_offset.current()
    }

    fn update(&mut self) {
        self.y_offset.update();
    }

    fn draw<'d, D, R>(
        &self,
        indicator_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &'d mut D,
        op: impl Fn(&mut Cropped<'_, Self::Display<'d, D>>) -> Result<R, D::Error>,
    ) -> Result<R, D::Error>
    where
        D: DrawTarget<Color = Self::Color> + 'd,
    {
        Rectangle::new(
            Point::new(0, screen_offset),
            Size::new(fill_width.max(1), indicator_height),
        )
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)?;

        let display_top_left = display.bounding_box().top_left;
        let display_size = display.bounding_box().size;

        let margin = Size::new(2, 0);

        let mut inverting = display.invert_area(&Rectangle::new(
            Point::new(0, screen_offset),
            Size::new(fill_width, indicator_height),
        ));
        op(&mut inverting.cropped(&Rectangle::new(
            display_top_left + margin,
            display_size - margin,
        )))
    }
}
