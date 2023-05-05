use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, DrawTargetExt, Point, Size},
    primitives::{Rectangle, StyledDrawable},
};

use crate::{
    adapters::invert::BinaryColorDrawTargetExt,
    selection_indicator::{IndicatorStyle, SelectionIndicator},
    MenuStyle,
};

pub struct SimpleSelectionIndicator {
    y_offset: i32,
    style: IndicatorStyle,
}

impl SimpleSelectionIndicator {
    pub fn new() -> Self {
        Self {
            y_offset: 0,
            style: IndicatorStyle::Line,
        }
    }

    pub fn with_indicator_style(self, style: IndicatorStyle) -> Self {
        Self { style, ..self }
    }
}

impl SelectionIndicator for SimpleSelectionIndicator {
    type Color = BinaryColor;

    fn update_target(&mut self, y: i32) {
        self.y_offset = y;
    }

    fn offset(&self) -> i32 {
        self.y_offset
    }

    fn update(&mut self) {}

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
        let margin = self.style.margin();
        self.style.draw(
            fill_width,
            &mut display.cropped(&Rectangle::new(
                Point::new(0, screen_offset),
                Size::new(display.bounding_box().size.width, indicator_height),
            )),
        )?;

        let display_top_left = display.bounding_box().top_left;
        let display_size = display.bounding_box().size;

        let mut inverting = display.invert_area(&Rectangle::new(
            Point::new(0, screen_offset),
            Size::new(fill_width, indicator_height),
        ));
        items.draw_styled(
            style,
            &mut inverting.cropped(&Rectangle::new(display_top_left + margin, display_size)),
        )
    }
}
