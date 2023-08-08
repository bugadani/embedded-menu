use alloc::borrow::Cow;
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
    title_text: Cow<'a, str>,
    details: Cow<'a, str>,
    return_value: R,
    marker: Cow<'a, str>,
    line: MenuLine,
}

impl<R: Copy> Marker for NavigationItem<'_, R> {}

impl<R> MenuItem<R> for NavigationItem<'_, R>
where
    R: Copy,
{
    fn interact(&mut self) -> R {
        self.return_value
    }

    fn title(&self) -> &str {
        self.title_text.as_ref()
    }

    fn details(&self) -> &str {
        self.details.as_ref()
    }

    fn value(&self) -> &str {
        self.marker.as_ref()
    }

    fn set_style<C, S, IT, P>(&mut self, style: &MenuStyle<C, S, IT, P>)
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InteractionController,
        P: SelectionIndicatorController,
    {
        self.line = MenuLine::new(self.marker.as_ref(), style);
    }
}

impl<'a, R: Copy> NavigationItem<'a, R> {
    pub fn new(title: impl Into<Cow<'a, str>>, value: R) -> Self {
        Self {
            title_text: title.into(),
            return_value: value,
            details: "".into(),
            marker: "".into(),
            line: MenuLine::empty(),
        }
    }

    pub fn with_marker(self, marker: impl Into<Cow<'a, str>>) -> Self {
        Self {
            marker: marker.into(),
            ..self
        }
    }

    pub fn with_detail_text(self, details: impl Into<Cow<'a, str>>) -> Self {
        Self {
            details: details.into(),
            ..self
        }
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
        self.line.draw_styled(
            self.title_text.as_ref(),
            self.marker.as_ref(),
            style,
            display,
        )
    }
}
