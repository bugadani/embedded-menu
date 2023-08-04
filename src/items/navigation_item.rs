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

pub struct NavigationItem<'a, R: Copy> {
    title_text: &'a str,
    details: &'a str,
    return_value: R,
    marker: &'a str,
    line: MenuLine,
}

impl<R: Copy> Marker for NavigationItem<'_, R> {}

impl<'a, R: Copy> MenuItem<R> for NavigationItem<'a, R> {
    fn interact(&mut self) -> R {
        self.return_value
    }

    fn title(&self) -> &str {
        self.title_text
    }

    fn details(&self) -> &str {
        self.details
    }

    fn value(&self) -> &str {
        self.marker
    }

    fn set_style<C, S, IT, P>(&mut self, style: &MenuStyle<C, S, IT, P>)
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InteractionController,
        P: SelectionIndicatorController,
    {
        self.line = MenuLine::new(self.value(), style);
    }
}

impl<'a, R: Copy> NavigationItem<'a, R> {
    pub fn new(title: &'a str, value: R) -> Self {
        Self {
            title_text: title,
            return_value: value,
            details: "",
            marker: "",
            line: MenuLine::empty(),
        }
    }

    pub fn with_marker(self, marker: &'a str) -> Self {
        Self { marker, ..self }
    }

    pub fn with_detail_text(self, details: &'a str) -> Self {
        Self { details, ..self }
    }
}

impl<R: Copy> View for NavigationItem<'_, R> {
    fn translate_impl(&mut self, by: Point) {
        self.line.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.line.bounds()
    }
}

impl<C, S, IT, P, R> StyledDrawable<MenuStyle<C, S, IT, P>> for NavigationItem<'_, R>
where
    C: PixelColor + From<Rgb888>,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
    R: Copy,
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
        self.line
            .draw_styled(self.title(), self.value(), style, display)
    }
}
