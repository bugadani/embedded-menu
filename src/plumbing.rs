use embedded_graphics::primitives::Rectangle;
use embedded_layout::{object_chain::ChainElement, prelude::*};

use crate::MenuItem;

/// Menu-related extensions for object chain elements
pub trait MenuItemCollection<R, S> {
    fn bounds_of(&self, nth: usize) -> Rectangle;
    fn title_of(&self, nth: usize) -> &str;
    fn details_of(&self, nth: usize) -> &str;
    fn interact_with(&mut self, nth: usize) -> R;
    fn count(&self) -> usize;
}

// Treat any MenuItem impl as a 1-element collection
impl<I, R, S> MenuItemCollection<R, S> for I
where
    I: MenuItem<R, S> + View + crate::Marker,
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

impl<I, R, S> MenuItemCollection<R, S> for Chain<I>
where
    I: MenuItemCollection<R, S>,
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

impl<I, LE, R, S> MenuItemCollection<R, S> for Link<I, LE>
where
    I: MenuItemCollection<R, S>,
    LE: MenuItemCollection<R, S> + ChainElement,
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
