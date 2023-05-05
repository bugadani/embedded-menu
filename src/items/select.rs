use crate::{items::MenuLine, margin::MarginExt, MenuEvent, MenuItem};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::{PixelColor, Point, Size},
    primitives::Rectangle,
};

pub trait SelectValue: Sized + Copy {
    fn next(&self) -> Self;
    fn name(&self) -> &'static str;
}

impl SelectValue for bool {
    fn next(&self) -> Self {
        !*self
    }

    fn name(&self) -> &'static str {
        match *self {
            // true => "O",
            // false => "0\r+\r#",
            true => "[ ]",
            false => "[X]",
        }
    }
}

pub struct Select<'a, R: Copy, S: SelectValue> {
    title_text: &'a str,
    details: &'a str,
    convert: fn(S) -> R,
    value: S,
}

impl<'a, S: SelectValue> Select<'a, (), S> {
    pub fn new(title: &'a str, value: S) -> Self {
        Self {
            title_text: title,
            value,
            convert: |_| (),
            details: "",
        }
    }
}

impl<'a, R: Copy, S: SelectValue> Select<'a, R, S> {
    pub fn with_value_converter<R2: Copy>(self, convert: fn(S) -> R2) -> Select<'a, R2, S> {
        Select {
            convert,
            title_text: self.title_text,
            value: self.value,
            details: self.details,
        }
    }

    pub fn with_detail_text(self, details: &'a str) -> Self {
        Self { details, ..self }
    }

    // TODO: temporary
    pub fn bind<C: PixelColor>(self, color: C) -> MenuLine<'a, C, Self> {
        let style = MonoTextStyle::<C>::new(&FONT_6X10, color);

        MenuLine {
            item: self,
            text_style: style,
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(1, style.font.character_size.height),
            )
            .with_margin(0, 0, -1, 1),
        }
    }
}

impl<'a, R: Copy, S: SelectValue> MenuItem for Select<'a, R, S> {
    type Data = R;

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

    fn value(&self) -> &str {
        self.value.name()
    }
}
