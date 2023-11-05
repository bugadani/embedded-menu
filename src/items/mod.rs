pub mod menu_item;

pub use menu_item::MenuItem;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

use crate::adapters::Canvas;

/// Marker trait necessary to avoid a "conflicting implementations" error.
pub trait Marker {}

pub trait MenuListItem<R>: Marker + View {
    /// Returns the value of the selected item, without interacting with it.
    fn value_of(&self) -> R;

    fn interact(&mut self) -> R;

    fn set_style(&mut self, text_style: &MonoTextStyle<'_, BinaryColor>);

    /// Returns whether the list item is selectable.
    ///
    /// If this returns false, the list item will not be interactable and user navigation will skip
    /// over it.
    fn selectable(&self) -> bool {
        true
    }

    fn draw_styled(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut dyn Canvas<BinaryColor>,
    ) -> Result<(), ()>;
}

impl<R> Marker for &mut dyn MenuListItem<R> {}

impl<R> View for &mut dyn MenuListItem<R> {
    fn translate_impl(&mut self, by: Point) {
        (**self).translate_impl(by)
    }

    fn bounds(&self) -> Rectangle {
        (**self).bounds()
    }
}

impl<R> MenuListItem<R> for &mut dyn MenuListItem<R> {
    fn value_of(&self) -> R {
        (**self).value_of()
    }

    fn interact(&mut self) -> R {
        (**self).interact()
    }

    fn set_style(&mut self, text_style: &MonoTextStyle<'_, BinaryColor>) {
        (**self).set_style(text_style)
    }

    fn selectable(&self) -> bool {
        (**self).selectable()
    }

    fn draw_styled(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut dyn Canvas<BinaryColor>,
    ) -> Result<(), ()> {
        (**self).draw_styled(text_style, display)
    }
}

/// Helper struct to draw a menu line that has a title and some additional marker.
pub struct MenuLine {
    bounds: Rectangle,
    value_width: u32,
}

impl MenuLine {
    pub fn new(longest_value: &str, text_style: &MonoTextStyle<'_, BinaryColor>) -> Self {
        let value_width = text_style
            .measure_string(longest_value, Point::zero(), Baseline::Top)
            .bounding_box
            .size
            .width;

        MenuLine {
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(1, text_style.font.character_size.height - 1),
            ),
            value_width,
        }
    }

    pub fn empty() -> Self {
        MenuLine {
            bounds: Rectangle::new(Point::zero(), Size::new(1, 0)),
            value_width: 0,
        }
    }

    pub fn draw_styled(
        &self,
        title: &str,
        value_text: &str,
        text_style: &MonoTextStyle<'static, BinaryColor>, // TODO: allow non-mono fonts
        mut display: &mut dyn Canvas<BinaryColor>,
    ) -> Result<(), ()> {
        let display_area = display.bounds();

        if self.bounds.intersection(&display_area).size.height == 0 {
            return Ok(());
        }

        let mut text_bounds = Rectangle::new(
            self.bounds.top_left,
            Size::new(display_area.size.width, self.bounds.size.height + 1),
        );

        TextBox::with_textbox_style(
            value_text,
            text_bounds,
            *text_style,
            TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Right)
                .build(),
        )
        .draw(&mut display)?;

        text_bounds.size.width -= self.value_width;
        TextBox::new(title, text_bounds, *text_style).draw(&mut display)?;

        Ok(())
    }
}

impl View for MenuLine {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}

#[cfg(test)]
mod test {
    #[allow(unused)]
    fn _is_object_safe(_: &dyn super::MenuListItem<u32>) {}
}
