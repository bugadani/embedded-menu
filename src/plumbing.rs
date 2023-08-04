use embedded_graphics::{prelude::Point, primitives::Rectangle};
use embedded_layout::{object_chain::ChainElement, prelude::*, view_group::ViewGroup};

use crate::MenuItem;

/// Menu-related extensions for object chain elements
pub trait MenuItemCollection<R, S> {
    fn set_style(&mut self, style: &S);
    fn bounds_of(&self, nth: u32) -> Rectangle;
    fn title_of(&self, nth: u32) -> &str;
    fn details_of(&self, nth: u32) -> &str;
    fn interact_with(&mut self, nth: u32) -> R;
    fn count(&self) -> usize;
}

// Treat any MenuItem impl as a 1-element collection
impl<I, R, S> MenuItemCollection<R, S> for I
where
    I: MenuItem<R, S> + View + crate::Marker,
{
    fn set_style(&mut self, style: &S) {
        MenuItem::set_style(self, style);
    }

    fn bounds_of(&self, nth: u32) -> Rectangle {
        debug_assert!(nth == 0);
        self.bounds()
    }

    fn interact_with(&mut self, nth: u32) -> R {
        debug_assert!(nth == 0);
        self.interact()
    }

    fn title_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.title()
    }

    fn details_of(&self, nth: u32) -> &str {
        debug_assert!(nth == 0);
        self.details()
    }

    fn count(&self) -> usize {
        1
    }
}

impl<R, S> MenuItemCollection<R, S> for &mut dyn MenuItemCollection<R, S> {
    fn set_style(&mut self, style: &S) {
        (**self).set_style(style)
    }

    fn bounds_of(&self, nth: u32) -> Rectangle {
        (**self).bounds_of(nth)
    }

    fn title_of(&self, nth: u32) -> &str {
        (**self).title_of(nth)
    }

    fn details_of(&self, nth: u32) -> &str {
        (**self).details_of(nth)
    }

    fn interact_with(&mut self, nth: u32) -> R {
        (**self).interact_with(nth)
    }

    fn count(&self) -> usize {
        (**self).count()
    }
}

pub struct MenuItems<'a, I> {
    items: &'a mut [I],
}

impl<'a, I> MenuItems<'a, I> {
    pub fn new(items: &'a mut [I]) -> Self {
        Self { items }
    }
}

impl<I> View for MenuItems<'_, I>
where
    I: View,
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

        for view in &self.items[1..] {
            rect = rect.enveloping(&view.bounds());
        }

        rect
    }
}

impl<I> ViewGroup for MenuItems<'_, I>
where
    I: View,
{
    fn len(&self) -> usize {
        self.items.len()
    }

    fn at(&self, idx: usize) -> &dyn View {
        &self.items[idx]
    }

    fn at_mut(&mut self, idx: usize) -> &mut dyn View {
        &mut self.items[idx]
    }
}

impl<I, R, S> MenuItemCollection<R, S> for MenuItems<'_, I>
where
    I: MenuItem<R, S> + View,
{
    fn set_style(&mut self, style: &S) {
        for item in self.items.iter_mut() {
            item.set_style(style);
        }
    }

    fn bounds_of(&self, nth: u32) -> Rectangle {
        self.items[nth as usize].bounds()
    }

    fn interact_with(&mut self, nth: u32) -> R {
        self.items[nth as usize].interact()
    }

    fn title_of(&self, nth: u32) -> &str {
        self.items[nth as usize].title()
    }

    fn details_of(&self, nth: u32) -> &str {
        self.items[nth as usize].details()
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

impl<I, R, S> MenuItemCollection<R, S> for Chain<I>
where
    I: MenuItemCollection<R, S> + View,
{
    fn set_style(&mut self, style: &S) {
        self.object.set_style(style);
    }

    fn bounds_of(&self, nth: u32) -> Rectangle {
        self.object.bounds_of(nth)
    }

    fn interact_with(&mut self, nth: u32) -> R {
        self.object.interact_with(nth)
    }

    fn title_of(&self, nth: u32) -> &str {
        self.object.title_of(nth)
    }

    fn details_of(&self, nth: u32) -> &str {
        self.object.details_of(nth)
    }

    fn count(&self) -> usize {
        self.object.count()
    }
}

impl<I, LE, R, S> MenuItemCollection<R, S> for Link<I, LE>
where
    I: MenuItemCollection<R, S> + View,
    LE: MenuItemCollection<R, S> + ChainElement,
{
    fn set_style(&mut self, style: &S) {
        self.object.set_style(style);
        self.parent.set_style(style);
    }

    fn bounds_of(&self, nth: u32) -> Rectangle {
        let count = self.object.count() as u32;
        if nth < count {
            self.object.bounds_of(nth)
        } else {
            self.parent.bounds_of(nth - count)
        }
    }

    fn interact_with(&mut self, nth: u32) -> R {
        let count = self.object.count() as u32;
        if nth < count {
            self.object.interact_with(nth)
        } else {
            self.parent.interact_with(nth - count)
        }
    }

    fn title_of(&self, nth: u32) -> &str {
        let count = self.object.count() as u32;
        if nth < count {
            self.object.title_of(nth)
        } else {
            self.parent.title_of(nth - count)
        }
    }

    fn details_of(&self, nth: u32) -> &str {
        let count = self.object.count() as u32;
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
