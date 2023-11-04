mod navigation_item;
mod section_title;
pub mod select;

pub use navigation_item::NavigationItem;
pub use section_title::SectionTitle;
pub use select::Select;

use crate::{
    interaction::InputAdapterSource,
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    theme::Theme,
    MenuStyle,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

pub struct MenuLine {
    bounds: Rectangle,
    value_width: u32,
}

impl MenuLine {
    pub fn new<T, S, IT, P, R>(longest_value: &str, style: &MenuStyle<S, IT, P, R, T>) -> Self
    where
        S: IndicatorStyle<Theme = T>,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        T: Theme,
    {
        let style = style.text_style();

        let value_width = style
            .measure_string(longest_value, Point::zero(), Baseline::Top)
            .bounding_box
            .size
            .width;

        MenuLine {
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(1, style.font.character_size.height - 1),
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

    pub fn draw_styled<T, D, S, IT, P, R>(
        &self,
        title: &str,
        value_text: &str,
        style: &MenuStyle<S, IT, P, R, T>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
        S: IndicatorStyle<Theme = T>,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        T: Theme,
    {
        let display_area = display.bounding_box();

        if self.bounds.intersection(&display_area).size.height == 0 {
            return Ok(());
        }

        let mut text_bounds = Rectangle::new(
            self.bounds.top_left,
            Size::new(display_area.size.width, self.bounds.size.height + 1),
        );

        let text_style = style.text_style();

        TextBox::with_textbox_style(
            value_text,
            text_bounds,
            text_style,
            TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Right)
                .build(),
        )
        .draw(display)?;

        text_bounds.size.width -= self.value_width;
        TextBox::new(title, text_bounds, text_style).draw(display)?;

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
