pub mod menu_item;

pub use menu_item::MenuItem;

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

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

    fn draw_styled<D>(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;
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

    pub fn draw_styled<D>(
        &self,
        title: &str,
        value_text: &str,
        text_style: &MonoTextStyle<'static, BinaryColor>, // TODO: allow non-mono fonts
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

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
        .draw(display)?;

        text_bounds.size.width -= self.value_width;
        TextBox::new(title, text_bounds, *text_style).draw(display)?;

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
