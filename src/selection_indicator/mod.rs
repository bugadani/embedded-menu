use embedded_graphics::{
    draw_target::Cropped,
    prelude::{DrawTarget, PixelColor},
};

pub mod simple;

pub trait SelectionIndicator: Sized {
    type Color: PixelColor;
    type Display<'a, D: DrawTarget<Color = Self::Color> + 'a>;

    fn new(anim_frames: i32) -> Self;

    fn update_target(&mut self, y: i32);

    fn offset(&self) -> i32;

    fn update(&mut self);

    fn draw<'d, D, R>(
        &self,
        indicator_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &'d mut D,
        op: impl Fn(&mut Cropped<'_, Self::Display<'d, D>>) -> Result<R, D::Error>,
    ) -> Result<R, D::Error>
    where
        D: DrawTarget<Color = Self::Color> + 'd;
}
