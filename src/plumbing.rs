use embedded_graphics::primitives::Rectangle;
use embedded_layout::{object_chain::ChainElement, prelude::*};

use crate::MenuItem;

/// Menu-related extensions for object chain elements
pub trait MenuItemCollection<R> {
    fn bounds_of(&self, nth: u32) -> Rectangle;
    fn title_of(&self, nth: u32) -> &str;
    fn details_of(&self, nth: u32) -> &str;
    fn interact_with(&mut self, nth: u32) -> R;
    fn count(&self) -> usize;
}

// Treat any MenuItem impl as a 1-element collection
impl<I, R> MenuItemCollection<R> for I
where
    I: MenuItem<Data = R> + View,
{
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

impl<I, R> MenuItemCollection<R> for Chain<I>
where
    I: MenuItemCollection<R>,
{
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

impl<I, LE, R> MenuItemCollection<R> for Link<I, LE>
where
    I: MenuItem<Data = R> + View,
    LE: MenuItemCollection<R> + ChainElement,
{
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
