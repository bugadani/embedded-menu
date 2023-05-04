use crate::{Margin, MarginExt, MenuEvent, MenuItemTrait, RectangleExt};

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{PixelColor, Point},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::TextBox;

pub trait SelectValue: Sized + Copy {
    fn next(self) -> Self;
    fn name(self) -> &'static str;
}

impl SelectValue for bool {
    fn next(self) -> Self {
        !self
    }

    fn name(self) -> &'static str {
        match self {
            // true => "O",
            // false => "0\r+\r#",
            true => "[ ]",
            false => "[X]",
        }
    }
}

pub struct Select<'a, R: Copy, S: SelectValue, C: PixelColor> {
    title_text: &'a str,
    style: MonoTextStyle<'a, C>,
    bounds: Margin<Rectangle>,
    details: &'a str,
    convert: fn(S) -> R,
    value: S,
}

impl<'a, R, S, C> Select<'a, R, S, C>
where
    R: Copy,
    S: SelectValue,
    C: PixelColor,
{
    pub fn new(
        title: &'a str,
        details: &'a str,
        initial: S,
        convert: fn(S) -> R,
        color: C,
    ) -> Self {
        let style = MonoTextStyle::new(&FONT_6X10, color);

        Self {
            title_text: title,
            details,
            convert,
            value: initial,
            style,
            bounds: style
                .measure_string(title, Point::zero(), Baseline::Top)
                .bounding_box
                .with_margin(1, 0, 0, 1),
        }
    }
}

impl<'a, R: Copy, S: SelectValue, C: PixelColor> MenuItemTrait<R> for Select<'a, R, S, C> {
    fn interact(&mut self) -> MenuEvent<R> {
        self.value = self.value.next();
        MenuEvent::DataEvent((self.convert)(self.value))
    }

    fn title(&self) -> &str {
        self.title_text
    }

    fn details(&self) -> &str {
        self.details
    }
}

impl<'a, R: Copy, S: SelectValue, C: PixelColor> View for Select<'a, R, S, C> {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds.bounds()
    }
}

impl<'a, R, S, C> Drawable for Select<'a, R, S, C>
where
    R: Copy,
    S: SelectValue,
    C: PixelColor + From<Rgb888>,
{
    type Color = C;
    type Output = ();

    fn draw<D: DrawTarget<Color = C>>(&self, display: &mut D) -> Result<(), D::Error> {
        let text_bounds = self.bounds();
        let display_area = display.bounding_box();

        if !text_bounds.intersects_with(&display_area) {
            return Ok(());
        }

        let value_text = self.value.name();
        let inner_bounds = self.bounds.inner().bounds();

        TextBox::new(self.title_text, inner_bounds, self.style).draw(display)?;

        TextBox::new(
            value_text,
            self.style
                .measure_string(value_text, inner_bounds.top_left, Baseline::Top)
                .bounding_box,
            self.style,
        )
        .align_to(&display_area, horizontal::Right, vertical::NoAlignment)
        .draw(display)?;

        Ok(())
    }
}
