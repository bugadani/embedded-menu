use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, PixelColor, Point},
    primitives::Rectangle,
};
use embedded_layout::View;

use crate::{
    interaction::InputAdapterSource,
    items::MenuLine,
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    Marker, MenuItem, MenuStyle,
};

pub struct NavigationItem<T, D, M, R>
where
    T: AsRef<str>,
    D: AsRef<str>,
    M: AsRef<str>,
{
    title_text: T,
    details: D,
    return_value: R,
    marker: M,
    line: MenuLine,
}

impl<T, D, M, R> Marker for NavigationItem<T, D, M, R>
where
    T: AsRef<str>,
    D: AsRef<str>,
    M: AsRef<str>,
{
}

impl<T, D, M, R> MenuItem<R> for NavigationItem<T, D, M, R>
where
    T: AsRef<str>,
    D: AsRef<str>,
    M: AsRef<str>,
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

    fn set_style<C, S, IT, P>(&mut self, style: &MenuStyle<C, S, IT, P, R>)
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
    {
        self.line = MenuLine::new(self.marker.as_ref(), style);
    }

    fn draw_styled<C, S, IT, P, DIS>(
        &self,
        style: &MenuStyle<C, S, IT, P, R>,
        display: &mut DIS,
    ) -> Result<(), DIS::Error>
    where
        C: PixelColor + From<Rgb888>,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        DIS: DrawTarget<Color = C>,
    {
        self.line.draw_styled(
            self.title_text.as_ref(),
            self.marker.as_ref(),
            style,
            display,
        )
    }
}

impl<T, R> NavigationItem<T, &'static str, &'static str, R>
where
    T: AsRef<str>,
{
    pub fn new(title: T, value: R) -> Self {
        NavigationItem {
            title_text: title,
            return_value: value,
            details: "",
            marker: "",
            line: MenuLine::empty(),
        }
    }
}

impl<T, D, M, R> NavigationItem<T, D, M, R>
where
    T: AsRef<str>,
    D: AsRef<str>,
    M: AsRef<str>,
{
    pub fn with_marker<M2: AsRef<str>>(self, marker: M2) -> NavigationItem<T, D, M2, R> {
        NavigationItem {
            marker,
            title_text: self.title_text,
            return_value: self.return_value,
            details: self.details,
            line: self.line,
        }
    }

    pub fn with_detail_text<D2: AsRef<str>>(self, details: D2) -> NavigationItem<T, D2, M, R> {
        NavigationItem {
            details,
            title_text: self.title_text,
            return_value: self.return_value,
            marker: self.marker,
            line: self.line,
        }
    }
}

impl<T, D, M, R> View for NavigationItem<T, D, M, R>
where
    T: AsRef<str>,
    D: AsRef<str>,
    M: AsRef<str>,
{
    fn translate_impl(&mut self, by: Point) {
        self.line.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.line.bounds()
    }
}
