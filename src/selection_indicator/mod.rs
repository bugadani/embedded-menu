use embedded_graphics::{
    prelude::{DrawTarget, PixelColor},
    primitives::StyledDrawable,
};

use crate::MenuStyle;

pub mod animated;
pub mod simple;

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
