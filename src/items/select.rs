use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, PixelColor, Point},
    primitives::Rectangle,
};
use embedded_layout::View;

use crate::{
    interaction::InputAdapter,
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

pub struct Select<T, D, R, S>
where
    T: AsRef<str>,
    D: AsRef<str>,
    S: SelectValue,
{
    title_text: T,
    details: D,
    convert: fn(S) -> R,
    value: S,
    line: MenuLine,
}

impl<T, S> Select<T, &'static str, (), S>
where
    T: AsRef<str>,
    S: SelectValue,
{
    pub fn new(title: T, value: S) -> Self {
        Self {
            title_text: title,
            value,
            convert: |_| (),
            details: "",
            line: MenuLine::empty(),
        }
    }
}

impl<T, D, R, S> Select<T, D, R, S>
where
    T: AsRef<str>,
    D: AsRef<str>,
    S: SelectValue,
{
    pub fn with_value_converter<R2: Copy>(self, convert: fn(S) -> R2) -> Select<T, D, R2, S> {
        Select {
            convert,
            title_text: self.title_text,
            value: self.value,
            details: self.details,
            line: self.line,
        }
    }

    pub fn with_detail_text<D2: AsRef<str>>(self, details: D2) -> Select<T, D2, R, S> {
        Select {
            details,
            title_text: self.title_text,
            value: self.value,
            convert: self.convert,
            line: self.line,
        }
    }
}

impl<T, D, R, S> Marker for Select<T, D, R, S>
where
    T: AsRef<str>,
    D: AsRef<str>,
    S: SelectValue,
{
}

impl<T, D, R, S> MenuItem<R> for Select<T, D, R, S>
where
    T: AsRef<str>,
    D: AsRef<str>,
    S: SelectValue,
{
    fn interact(&mut self) -> R {
        self.value = self.value.next();
        (self.convert)(self.value)
    }

    fn title(&self) -> &str {
        self.title_text.as_ref()
    }

    fn details(&self) -> &str {
        self.details.as_ref()
    }

    fn value(&self) -> &str {
        self.value.name()
    }

    fn set_style<C, ST, IT, P>(&mut self, style: &MenuStyle<C, ST, IT, P>)
    where
        C: PixelColor,
        ST: IndicatorStyle,
        IT: InputAdapter,
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

    fn draw_styled<C, ST, IT, P, DIS>(
        &self,
        style: &MenuStyle<C, ST, IT, P>,
        display: &mut DIS,
    ) -> Result<(), DIS::Error>
    where
        C: PixelColor + From<Rgb888>,
        ST: IndicatorStyle,
        IT: InputAdapter,
        P: SelectionIndicatorController,
        DIS: DrawTarget<Color = C>,
    {
        self.line
            .draw_styled(self.title_text.as_ref(), self.value.name(), style, display)
    }
}

impl<T, D, R, S> View for Select<T, D, R, S>
where
    T: AsRef<str>,
    D: AsRef<str>,
    S: SelectValue,
{
    fn translate_impl(&mut self, by: Point) {
        self.line.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.line.bounds()
    }
}
