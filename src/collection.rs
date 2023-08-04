use core::marker::PhantomData;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, PixelColor, Point},
    primitives::{Rectangle, StyledDrawable},
};
use embedded_layout::{object_chain::ChainElement, prelude::*, view_group::ViewGroup};

use crate::{
    interaction::InteractionController,
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    Marker, MenuItem, MenuStyle,
};

/// Menu-related extensions for object chain elements
pub trait MenuItemCollection<R> {
    fn bounds_of(&self, nth: usize) -> Rectangle;
    fn title_of(&self, nth: usize) -> &str;
    fn details_of(&self, nth: usize) -> &str;
    fn interact_with(&mut self, nth: usize) -> R;
    fn count(&self) -> usize;
}

// Treat any MenuItem impl as a 1-element collection
impl<I, R> MenuItemCollection<R> for I
where
    I: MenuItem<R> + View + Marker,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        debug_assert!(nth == 0);
        self.bounds()
    }

    fn interact_with(&mut self, nth: usize) -> R {
        debug_assert!(nth == 0);
        self.interact()
    }

    fn title_of(&self, nth: usize) -> &str {
        debug_assert!(nth == 0);
        self.title()
    }

    fn details_of(&self, nth: usize) -> &str {
        debug_assert!(nth == 0);
        self.details()
    }

    fn count(&self) -> usize {
        1
    }
}

pub struct MenuItems<'a, I, R>
where
    I: MenuItem<R> + Marker,
{
    items: &'a mut [I],
    _marker: PhantomData<R>,
}

impl<'a, I, R> MenuItems<'a, I, R>
where
    I: MenuItem<R> + Marker,
{
    pub fn new(items: &'a mut [I]) -> Self {
        Self {
            items,
            _marker: PhantomData,
        }
    }
}

impl<I, R> MenuItemCollection<R> for MenuItems<'_, I, R>
where
    I: MenuItem<R> + View + Marker,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        self.items[nth].bounds()
    }

    fn interact_with(&mut self, nth: usize) -> R {
        self.items[nth].interact()
    }

    fn title_of(&self, nth: usize) -> &str {
        self.items[nth].title()
    }

    fn details_of(&self, nth: usize) -> &str {
        self.items[nth].details()
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

impl<I, R> View for MenuItems<'_, I, R>
where
    I: MenuItem<R> + View + Marker,
{
    fn translate_impl(&mut self, by: Point) {
        for view in self.items.iter_mut() {
            view.translate_impl(by);
        }
    }

    fn bounds(&self) -> Rectangle {
        if self.items.is_empty() {
            return Rectangle::zero();
        }

        let mut rect = self.items[0].bounds();

        for view in self.items[1..].iter() {
            rect = rect.enveloping(&view.bounds());
        }

        rect
    }
}

impl<I, R> ViewGroup for MenuItems<'_, I, R>
where
    I: MenuItem<R> + View + Marker,
{
    fn len(&self) -> usize {
        self.count()
    }

    fn at(&self, idx: usize) -> &dyn View {
        &self.items[idx]
    }

    fn at_mut(&mut self, idx: usize) -> &mut dyn View {
        &mut self.items[idx]
    }
}

impl<I, C, S, IT, P, R> StyledDrawable<MenuStyle<C, S, IT, P>> for MenuItems<'_, I, R>
where
    I: MenuItem<R> + View + Marker + StyledDrawable<MenuStyle<C, S, IT, P>, Color = C, Output = ()>,
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
        for view in self.items[1..].iter() {
            view.draw_styled(style, display)?;
        }

        Ok(())
    }
}

impl<I, R> MenuItemCollection<R> for Chain<I>
where
    I: MenuItemCollection<R>,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        self.object.bounds_of(nth)
    }

    fn interact_with(&mut self, nth: usize) -> R {
        self.object.interact_with(nth)
    }

    fn title_of(&self, nth: usize) -> &str {
        self.object.title_of(nth)
    }

    fn details_of(&self, nth: usize) -> &str {
        self.object.details_of(nth)
    }

    fn count(&self) -> usize {
        self.object.count()
    }
}

impl<I, LE, R> MenuItemCollection<R> for Link<I, LE>
where
    I: MenuItemCollection<R>,
    LE: MenuItemCollection<R> + ChainElement,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        let count = self.object.count();
        if nth < count {
            self.object.bounds_of(nth)
        } else {
            self.parent.bounds_of(nth - count)
        }
    }

    fn interact_with(&mut self, nth: usize) -> R {
        let count = self.object.count();
        if nth < count {
            self.object.interact_with(nth)
        } else {
            self.parent.interact_with(nth - count)
        }
    }

    fn title_of(&self, nth: usize) -> &str {
        let count = self.object.count();
        if nth < count {
            self.object.title_of(nth)
        } else {
            self.parent.title_of(nth - count)
        }
    }

    fn details_of(&self, nth: usize) -> &str {
        let count = self.object.count();
        if nth < count {
            self.object.details_of(nth)
        } else {
            self.parent.details_of(nth - count)
        }
    }

    fn count(&self) -> usize {
        self.object.count() + self.parent.count()
    }
}
