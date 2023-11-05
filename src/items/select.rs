use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
};
use embedded_layout::View;

use crate::{
    interaction::InputAdapterSource,
    items::{Marker, MenuLine, MenuListItem},
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    theme::Theme,
    MenuStyle,
};

pub trait SelectValue: Sized + Copy + PartialEq {
    /// Transforms the value on interaction
    fn next(&self) -> Self {
        *self
    }

    /// Returns a displayable marker for the value
    fn marker(&self) -> &'static str;
}

impl SelectValue for bool {
    fn next(&self) -> Self {
        !*self
    }

    fn marker(&self) -> &'static str {
        match *self {
            // true => "O",
            // false => "O\r+\r#", // this only works for certain small fonts, unfortunately
            false => "[ ]",
            true => "[X]",
        }
    }
}

impl SelectValue for &'static str {
    fn next(&self) -> Self {
        *self
    }

    fn marker(&self) -> &'static str {
        *self
    }
}

pub struct Select<T, R, S>
where
    T: AsRef<str>,
    S: SelectValue,
{
    title_text: T,
    convert: fn(S) -> R,
    value: S,
    line: MenuLine,
}

impl<T, S> Select<T, (), S>
where
    T: AsRef<str>,
    S: SelectValue,
{
    pub fn new(title_text: T, value: S) -> Self {
        Self {
            title_text,
            value,
            convert: |_| (),
            line: MenuLine::empty(),
        }
    }
}

impl<T, R, S> Select<T, R, S>
where
    T: AsRef<str>,
    S: SelectValue,
{
    pub fn with_value_converter<R2: Copy>(self, convert: fn(S) -> R2) -> Select<T, R2, S> {
        Select {
            convert,
            title_text: self.title_text,
            value: self.value,
            line: self.line,
        }
    }
}

impl<T, R, S> Marker for Select<T, R, S>
where
    T: AsRef<str>,
    S: SelectValue,
{
}

impl<T, R, S> MenuListItem<R> for Select<T, R, S>
where
    T: AsRef<str>,
    S: SelectValue,
{
    fn value_of(&self) -> R {
        (self.convert)(self.value)
    }

    fn interact(&mut self) -> R {
        self.value = self.value.next();
        (self.convert)(self.value)
    }

    fn set_style<IS, IT, P, C>(&mut self, style: &MenuStyle<IS, IT, P, R, C>)
    where
        IS: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        C: Theme,
    {
        let initial = self.value;
        let mut longest_str = initial.marker();

        let mut current = initial.next();
        while current != initial {
            if current.marker().len() > longest_str.len() {
                longest_str = current.marker();
            }
            current = current.next();
        }

        self.line = MenuLine::new(longest_str, style);
    }

    fn draw_styled<IS, IT, P, DIS, C>(
        &self,
        style: &MenuStyle<IS, IT, P, R, C>,
        display: &mut DIS,
    ) -> Result<(), DIS::Error>
    where
        IS: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        DIS: DrawTarget<Color = BinaryColor>,
        C: Theme,
    {
        self.line.draw_styled(
            self.title_text.as_ref(),
            self.value.marker(),
            style,
            display,
        )
    }
}

impl<T, R, S> View for Select<T, R, S>
where
    T: AsRef<str>,
    S: SelectValue,
{
    fn translate_impl(&mut self, by: Point) {
        self.line.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.line.bounds()
    }
}
