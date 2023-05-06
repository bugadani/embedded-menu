mod navigation_item;
pub mod select;

pub use navigation_item::NavigationItem;
pub use select::Select;

use crate::{
    interaction::InteractionController,
    margin::{Margin, MarginExt},
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    MenuItem, MenuStyle,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{PixelColor, Point, Size},
    primitives::{Rectangle, StyledDrawable},
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

pub struct MenuLine<I> {
    bounds: Margin<Rectangle>,
    value_width: u32,
    item: I,
}

impl<I> MenuLine<I>
where
    I: MenuItem,
{
    pub fn new<C, S, IT, P>(item: I, style: MenuStyle<C, S, IT, P>) -> MenuLine<I>
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InteractionController,
        P: SelectionIndicatorController,
    {
        let style = style.text_style();

        let longest_value = item.longest_value_str();

        let value_width = style
            .measure_string(longest_value, Point::zero(), Baseline::Top)
            .bounding_box
            .size
            .width;

        MenuLine {
            item,
            value_width,
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(1, style.font.character_size.height),
            )
            .with_margin(0, 0, -1, 0),
        }
    }

    pub fn as_item(&self) -> &I {
        &self.item
    }

    pub fn as_item_mut(&mut self) -> &mut I {
        &mut self.item
    }
}

impl<I> View for MenuLine<I> {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds.bounds()
    }
}

impl<C, S, I, IT, P> StyledDrawable<MenuStyle<C, S, IT, P>> for MenuLine<I>
where
    C: PixelColor + From<Rgb888>,
    I: MenuItem,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &MenuStyle<C, S, IT, P>,
        display: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let text_bounds = self.bounds.bounds();
        let display_area = display.bounding_box();

        if text_bounds.intersection(&display_area).is_zero_sized() {
            return Ok(());
        }

        let text_style = style.text_style();
        let value_text = self.item.value();

        let mut inner_bounds = self.bounds.inner.bounds();
        inner_bounds.size.width = display_area.size.width - self.bounds.left as u32;
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
        TextBox::new(self.item.title(), inner_bounds, text_style).draw(display)?;

        Ok(())
    }
}
