use crate::{items::MenuLine, margin::MarginExt, MenuEvent, MenuItem};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::{PixelColor, Point, Size},
    primitives::Rectangle,
};

pub struct NavigationItem<'a, R: Copy> {
    title_text: &'a str,
    details: &'a str,
    return_value: R,
    marker: &'a str,
}

impl<'a, R: Copy> MenuItem for NavigationItem<'a, R> {
    type Data = R;

    fn interact(&mut self) -> MenuEvent<R> {
        MenuEvent::NavigationEvent(self.return_value)
    }

    fn title(&self) -> &str {
        self.title_text
    }

    fn details(&self) -> &str {
        self.details
    }

    fn value(&self) -> &str {
        self.marker
    }
}

impl<'a, R: Copy> NavigationItem<'a, R> {
    pub fn new(title: &'a str, value: R) -> Self {
        Self {
            title_text: title,
            return_value: value,
            details: "",
            marker: "",
        }
    }

    pub fn with_marker(self, marker: &'a str) -> Self {
        Self { marker, ..self }
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
