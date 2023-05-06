use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, DrawTargetExt, Point, Size},
    primitives::{Rectangle, StyledDrawable},
};

use crate::{
    adapters::invert::BinaryColorDrawTargetExt,
    selection_indicator::{IndicatorStyle, Insets, SelectionIndicator},
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

    fn style(&self) -> IndicatorStyle {
        self.style
    }

    fn update(&mut self) {}

    fn draw<D>(
        &self,
        selected_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &mut D,
        items: &impl StyledDrawable<MenuStyle<Self::Color>, Color = Self::Color, Output = ()>,
        style: &MenuStyle<Self::Color>,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let Insets {
            left: margin_left,
            top: margin_top,
            right: margin_right,
            bottom: margin_bottom,
        } = self.style.margin(selected_height);

        self.style.draw(
            fill_width,
            &mut display.cropped(&Rectangle::new(
                Point::new(0, screen_offset),
                Size::new(
                    display.bounding_box().size.width,
                    (selected_height as i32 + margin_top + margin_bottom) as u32,
                ),
            )),
        )?;

        let display_top_left = display.bounding_box().top_left;
        let display_size = display.bounding_box().size;

        let mut inverting = display.invert_area(&Rectangle::new(
            Point::new(0, screen_offset),
            Size::new(fill_width, selected_height),
        ));
        items.draw_styled(
            style,
            &mut inverting.cropped(&Rectangle::new(
                display_top_left + Point::new(margin_left, margin_top),
                Size::new(
                    (display_size.width as i32 - margin_left - margin_right) as u32,
                    display_size.height,
                ),
            )),
        )
    }
}
