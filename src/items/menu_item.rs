use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
};
use embedded_layout::View;

use crate::items::{Marker, MenuLine, MenuListItem};

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

impl SelectValue for () {
    fn next(&self) -> Self {
        ()
    }

    fn marker(&self) -> &'static str {
        ""
    }
}

pub struct MenuItem<T, R, S, const SELECTABLE: bool>
where
    T: AsRef<str>,
    S: SelectValue,
{
    title_text: T,
    convert: fn(S) -> R,
    value: S,
    line: MenuLine,
}

impl<T, S> MenuItem<T, (), S, true>
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

impl<T, R, S, const SELECTABLE: bool> MenuItem<T, R, S, SELECTABLE>
where
    T: AsRef<str>,
    S: SelectValue,
{
    pub fn with_value_converter<R2>(self, convert: fn(S) -> R2) -> MenuItem<T, R2, S, SELECTABLE> {
        MenuItem {
            convert,
            title_text: self.title_text,
            value: self.value,
            line: self.line,
        }
    }

    /// Make the item selectable or not
    pub fn selectable<const SELECTABLE2: bool>(self) -> MenuItem<T, R, S, SELECTABLE2> {
        MenuItem {
            convert: self.convert,
            title_text: self.title_text,
            value: self.value,
            line: self.line,
        }
    }
}

impl<T, R, S, const SELECTABLE: bool> Marker for MenuItem<T, R, S, SELECTABLE>
where
    T: AsRef<str>,
    S: SelectValue,
{
}

impl<T, R, S, const SELECTABLE: bool> MenuListItem<R> for MenuItem<T, R, S, SELECTABLE>
where
    T: AsRef<str>,
    S: SelectValue,
{
    fn value_of(&self) -> R {
        (self.convert)(self.value)
    }

    fn interact(&mut self) -> R {
        self.value = self.value.next();
        self.value_of()
    }

    fn selectable(&self) -> bool {
        SELECTABLE
    }

    fn set_style(&mut self, text_style: &MonoTextStyle<'_, BinaryColor>) {
        let initial = self.value;
        let mut longest_str = initial.marker();

        let mut current = initial.next();
        while current != initial {
            if current.marker().len() > longest_str.len() {
                longest_str = current.marker();
            }
            current = current.next();
        }

        self.line = MenuLine::new(longest_str, text_style);
    }

    fn draw_styled<D>(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.line.draw_styled(
            self.title_text.as_ref(),
            self.value.marker(),
            text_style,
            display,
        )
    }
}

impl<T, R, S, const SELECTABLE: bool> View for MenuItem<T, R, S, SELECTABLE>
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

#[cfg(test)]
mod test {
    #[test]
    fn interaction_selects_next_value_and_returns_converted() {
        use super::*;
        use crate::items::MenuListItem;

        let mut item = MenuItem::new("title", false).with_value_converter(|b| b as u8);

        assert_eq!(item.value_of(), 0);

        assert_eq!(item.interact(), 1);
        assert_eq!(item.value_of(), 1);

        assert_eq!(item.interact(), 0);
        assert_eq!(item.value_of(), 0);
    }
}
