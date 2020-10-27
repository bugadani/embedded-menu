use crate::{MenuEvent, MenuItemTrait};

use crate::{Margin, MarginExt, RectangleExt};
use embedded_graphics::{fonts::Font6x8, primitives::Rectangle, DrawTarget};
use embedded_layout::prelude::*;
use embedded_text::{alignment::*, prelude::*, utils::font_ext::FontExt};

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
    style: TextBoxStyle<C, Font6x8, LeftAligned, TopAligned>,
    bounds: Margin<Rectangle>,
    details: &'a str,
    convert: fn(S) -> R,
    value: S,
}

impl<'a, R: Copy, S: SelectValue, C: PixelColor> Select<'a, R, S, C> {
    pub fn new(
        title: &'a str,
        details: &'a str,
        initial: S,
        convert: fn(S) -> R,
        color: C,
    ) -> Self {
        let style = TextBoxStyleBuilder::new(Font6x8).text_color(color).build();

        let width = Font6x8::str_width(title);
        let height = Font6x8::CHARACTER_SIZE.height;

        Self {
            title_text: title,
            details,
            convert,
            value: initial,
            style,
            bounds: Rectangle::with_size(Point::zero(), Size::new(width, height))
                .with_margin(2, 0, 1, 1),
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
    #[must_use]
    fn translate(mut self, by: Point) -> Self {
        self.bounds.translate_mut(by);
        self
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.bounds.translate_mut(by);
        self
    }

    fn bounds(&self) -> Rectangle {
        self.bounds.bounds()
    }
}

impl<'a, R: Copy, S: SelectValue, C: PixelColor> Drawable<C> for &Select<'a, R, S, C> {
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let text_bounds = self.bounds();
        let display_area = display.display_area();

        if !text_bounds.intersects_with(&display_area) {
            return Ok(());
        }

        let value_text = self.value.name();
        let inner_bounds = self.bounds.inner().bounds();

        TextBox::new(self.title_text, inner_bounds)
            .into_styled(self.style)
            .draw(display)?;

        TextBox::new(
            value_text,
            Rectangle::with_size(
                inner_bounds.top_left,
                Size::new(
                    Font6x8::str_width(value_text),
                    Font6x8::CHARACTER_SIZE.height,
                ),
            ),
        )
        .into_styled(self.style)
        .align_to(&display_area, horizontal::Right, vertical::NoAlignment)
        .draw(display)
    }
}
