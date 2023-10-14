mod navigation_item;
pub mod select;

pub use navigation_item::NavigationItem;
pub use select::Select;

use crate::{
    interaction::InputAdapterSource,
    margin::{Margin, MarginExt},
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    MenuStyle,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{PixelColor, Point, Size},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

pub struct MenuLine {
    bounds: Margin<Rectangle>,
    value_width: u32,
}

impl MenuLine {
    pub fn new<C, S, IT, P, R>(longest_value: &str, style: &MenuStyle<C, S, IT, P, R>) -> Self
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
    {
        let style = style.text_style();

        let value_width = style
            .measure_string(longest_value, Point::zero(), Baseline::Top)
            .bounding_box
            .size
            .width;

        MenuLine {
            value_width,
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(1, style.font.character_size.height),
            )
            .with_margin(0, 0, -1, 0),
        }
    }

    pub fn empty() -> Self {
        MenuLine {
            bounds: Rectangle::new(Point::zero(), Size::new(1, 1)).with_margin(0, 0, -1, 0),
            value_width: 0,
        }
    }

    pub fn draw_styled<D, C, S, IT, P, R>(
        &self,
        title: &str,
        value_text: &str,
        style: &MenuStyle<C, S, IT, P, R>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
        C: PixelColor + From<Rgb888>,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
    {
        let text_bounds = self.bounds.bounds();
        let display_area = display.bounding_box();

        if text_bounds.intersection(&display_area).size.height == 0 {
            return Ok(());
        }

        let text_style = style.text_style();

        let mut inner_bounds = self.bounds.inner.bounds();
        inner_bounds.size.width = display_area.size.width - self.bounds.insets.left as u32;
        TextBox::with_textbox_style(
            value_text,
            inner_bounds,
            text_style,
            TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Right)
                .build(),
        )
        .draw(display)?;

        inner_bounds.size.width -= self.value_width;
        TextBox::new(title, inner_bounds, text_style).draw(display)?;

        Ok(())
    }
}

impl View for MenuLine {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds.bounds()
    }
}
