use crate::{MenuEvent, MenuItemTrait};

use crate::{Margin, MarginExt, RectangleExt};
use embedded_graphics::{fonts::Font6x8, primitives::Rectangle, DrawTarget};
use embedded_layout::prelude::*;
use embedded_text::{alignment::*, prelude::*, utils::font_ext::FontExt};

pub struct NavigationItem<'a, R: Copy, C: PixelColor> {
    style: TextBoxStyle<C, Font6x8, LeftAligned, TopAligned>,
    title_text: &'a str,
    details: &'a str,
    return_value: R,
    bounds: Margin<Rectangle>,
}

impl<'a, R: Copy, C: PixelColor> NavigationItem<'a, R, C> {
    pub fn new(title: &'a str, details: &'a str, value: R, color: C) -> Self {
        let style = TextBoxStyleBuilder::new(Font6x8).text_color(color).build();

        let width = Font6x8::str_width(title);
        let height = Font6x8::CHARACTER_SIZE.height;

        Self {
            style,
            title_text: title,
            details,
            return_value: value,
            bounds: Rectangle::with_size(Point::zero(), Size::new(width, height))
                .with_margin(2, 0, 1, 1),
        }
    }
}

impl<'a, R: Copy, C: PixelColor> MenuItemTrait<R> for NavigationItem<'a, R, C> {
    fn interact(&mut self) -> MenuEvent<R> {
        MenuEvent::NavigationEvent(self.return_value)
    }

    fn title(&self) -> &str {
        self.title_text
    }

    fn details(&self) -> &str {
        self.details
    }
}

impl<'a, R: Copy, C: PixelColor> View for NavigationItem<'a, R, C> {
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

impl<'a, R: Copy, C: PixelColor> Drawable<C> for &NavigationItem<'a, R, C> {
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        let text_bounds = self.bounds();
        let display_area = display.display_area();

        if !text_bounds.intersects_with(&display_area) {
            return Ok(());
        }

        let inner_bounds = self.bounds.inner().bounds();

        TextBox::new(self.title_text, inner_bounds)
            .into_styled(self.style)
            .draw(display)?;

        TextBox::new(
            "»",
            Rectangle::with_size(
                inner_bounds.top_left,
                Size::new(Font6x8::total_char_width('»'), inner_bounds.size().height),
            ),
        )
        .into_styled(self.style)
        .align_to(&display_area, horizontal::Right, vertical::NoAlignment)
        .draw(display)
    }
}
