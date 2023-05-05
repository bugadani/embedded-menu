use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, DrawTargetExt, Point, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle, StyledDrawable},
    Drawable,
};

use crate::{
    adapters::invert::BinaryColorDrawTargetExt, selection_indicator::SelectionIndicator, Animated,
    MenuStyle,
};

pub struct AnimatedSelectionIndicator {
    y_offset: Animated,
}

impl AnimatedSelectionIndicator {
    pub fn new(anim_frames: i32) -> Self {
        Self {
            y_offset: Animated::new(0, anim_frames),
        }
    }
}

impl SelectionIndicator for AnimatedSelectionIndicator {
    type Color = BinaryColor;

    fn update_target(&mut self, y: i32) {
        self.y_offset.update_target(y);
    }

    fn offset(&self) -> i32 {
        self.y_offset.current()
    }

    fn update(&mut self) {
        self.y_offset.update();
    }

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
        D: DrawTarget<Color = Self::Color>,
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
        items.draw_styled(
            style,
            &mut inverting.cropped(&Rectangle::new(
                display_top_left + margin,
                display_size - margin,
            )),
        )
    }
}
