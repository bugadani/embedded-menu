use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, PixelColor, Point},
    primitives::{Rectangle, StyledDrawable},
};
use embedded_layout::View;

use crate::{
    interaction::InteractionController,
    items::MenuLine,
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    Marker, MenuItem, MenuStyle,
};

pub trait SelectValue: Sized + Copy + PartialEq {
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
            // false => "O\r+\r#", // this only works for certain small fonts, unfortunately
            false => "[ ]",
            true => "[X]",
        }
    }
}

pub struct Select<'a, R, S: SelectValue> {
    title_text: &'a str,
    details: &'a str,
    convert: fn(S) -> R,
    value: S,
    line: MenuLine,
}

impl<'a, S: SelectValue> Select<'a, (), S> {
    pub fn new(title: &'a str, value: S) -> Self {
        Self {
            title_text: title,
            value,
            convert: |_| (),
            details: "",
            line: MenuLine::empty(),
        }
    }
}

impl<'a, R, S: SelectValue> Select<'a, R, S> {
    pub fn with_value_converter<R2: Copy>(self, convert: fn(S) -> R2) -> Select<'a, R2, S> {
        Select {
            convert,
            title_text: self.title_text,
            value: self.value,
            details: self.details,
            line: self.line,
        }
    }

    pub fn with_detail_text(self, details: &'a str) -> Self {
        Self { details, ..self }
    }
}

impl<R, S: SelectValue> Marker for Select<'_, R, S> {}

impl<'a, R, S: SelectValue> MenuItem<R> for Select<'a, R, S> {
    fn interact(&mut self) -> R {
        self.value = self.value.next();
        (self.convert)(self.value)
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

    fn set_style<C, ST, IT, P>(&mut self, style: &MenuStyle<C, ST, IT, P>)
    where
        C: PixelColor,
        ST: IndicatorStyle,
        IT: InteractionController,
        P: SelectionIndicatorController,
    {
        let initial = self.value;
        let mut longest_str = initial.name();

        let mut current = initial.next();
        while current != initial {
            if current.name().len() > longest_str.len() {
                longest_str = current.name();
            }
            current = current.next();
        }

        self.line = MenuLine::new(longest_str, style);
    }
}

impl<R, S: SelectValue> View for Select<'_, R, S> {
    fn translate_impl(&mut self, by: Point) {
        self.line.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.line.bounds()
    }
}

impl<C, ST, IT, P, R, S> StyledDrawable<MenuStyle<C, ST, IT, P>> for Select<'_, R, S>
where
    C: PixelColor + From<Rgb888>,
    ST: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
    S: SelectValue,
{
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &MenuStyle<C, ST, IT, P>,
        display: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.line
            .draw_styled(self.title(), self.value(), style, display)
    }
}
