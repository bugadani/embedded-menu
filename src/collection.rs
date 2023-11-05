use core::marker::PhantomData;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};
use embedded_layout::{object_chain::ChainElement, prelude::*, view_group::ViewGroup};

use crate::items::{Marker, MenuListItem};

/// Menu-related extensions for object chain elements
pub trait MenuItemCollection<R> {
    fn bounds_of(&self, nth: usize) -> Rectangle;
    fn value_of(&self, nth: usize) -> R;
    fn interact_with(&mut self, nth: usize) -> R;
    /// Whether an item is selectable. If not, the item will be skipped.
    fn selectable(&self, nth: usize) -> bool;
    fn count(&self) -> usize;
    fn draw_styled<D>(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>;
}

// Treat any MenuItem impl as a 1-element collection
impl<I, R> MenuItemCollection<R> for I
where
    I: MenuListItem<R> + Marker,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        debug_assert!(nth == 0);
        self.bounds()
    }

    fn value_of(&self, nth: usize) -> R {
        debug_assert!(nth == 0);
        self.value_of()
    }

    fn interact_with(&mut self, nth: usize) -> R {
        debug_assert!(nth == 0);
        self.interact()
    }

    fn selectable(&self, nth: usize) -> bool {
        debug_assert!(nth == 0);
        self.selectable()
    }

    fn count(&self) -> usize {
        1
    }

    fn draw_styled<D>(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        MenuListItem::draw_styled(self, text_style, display)
    }
}

pub struct MenuItems<C, I, R>
where
    C: AsRef<[I]> + AsMut<[I]>,
    I: MenuListItem<R>,
{
    items: C,
    /// Used to keep track of the whole collection's position in case it's empty.
    position: Point,
    _marker: PhantomData<(I, R)>,
}

impl<C, I, R> MenuItems<C, I, R>
where
    C: AsRef<[I]> + AsMut<[I]>,
    I: MenuListItem<R>,
{
    pub fn new(mut items: C) -> Self {
        let mut offset = 0;

        for item in items.as_mut().iter_mut() {
            item.translate_mut(Point::new(0, offset));
            offset += item.bounds().size.height as i32;
        }

        Self {
            items,
            position: Point::zero(),
            _marker: PhantomData,
        }
    }
}

impl<C, I, R> MenuItemCollection<R> for MenuItems<C, I, R>
where
    C: AsRef<[I]> + AsMut<[I]>,
    I: MenuListItem<R>,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        self.items.as_ref()[nth].bounds()
    }

    fn value_of(&self, nth: usize) -> R {
        self.items.as_ref()[nth].value_of()
    }

    fn interact_with(&mut self, nth: usize) -> R {
        self.items.as_mut()[nth].interact()
    }

    fn selectable(&self, nth: usize) -> bool {
        self.items.as_ref()[nth].selectable()
    }

    fn count(&self) -> usize {
        self.items.as_ref().len()
    }

    fn draw_styled<D>(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        for item in self.items.as_ref() {
            item.draw_styled(text_style, display)?;
        }

        Ok(())
    }
}

impl<C, I, R> View for MenuItems<C, I, R>
where
    C: AsRef<[I]> + AsMut<[I]>,
    I: MenuListItem<R>,
{
    fn translate_impl(&mut self, by: Point) {
        self.position += by;
        for view in self.items.as_mut().iter_mut() {
            view.translate_impl(by);
        }
    }

    fn bounds(&self) -> Rectangle {
        let mut size = Size::zero();

        for view in self.items.as_ref().iter() {
            let view_size = view.bounds().size;
            size = Size::new(
                size.width.max(view_size.width),
                size.height + view_size.height,
            );
        }

        Rectangle::new(self.position, size)
    }
}

impl<C, I, R> ViewGroup for MenuItems<C, I, R>
where
    C: AsRef<[I]> + AsMut<[I]>,
    I: MenuListItem<R>,
{
    fn len(&self) -> usize {
        self.count()
    }

    fn at(&self, idx: usize) -> &dyn View {
        &self.items.as_ref()[idx]
    }

    fn at_mut(&mut self, idx: usize) -> &mut dyn View {
        &mut self.items.as_mut()[idx]
    }
}

impl<I, R> MenuItemCollection<R> for Chain<I>
where
    I: MenuItemCollection<R>,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        self.object.bounds_of(nth)
    }

    fn value_of(&self, nth: usize) -> R {
        self.object.value_of(nth)
    }

    fn interact_with(&mut self, nth: usize) -> R {
        self.object.interact_with(nth)
    }

    fn selectable(&self, nth: usize) -> bool {
        self.object.selectable(nth)
    }

    fn count(&self) -> usize {
        self.object.count()
    }

    fn draw_styled<D>(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.object.draw_styled(text_style, display)
    }
}

impl<I, LE, R> MenuItemCollection<R> for Link<I, LE>
where
    I: MenuItemCollection<R>,
    LE: MenuItemCollection<R> + ChainElement,
{
    fn bounds_of(&self, nth: usize) -> Rectangle {
        let count = self.parent.count();
        if nth < count {
            self.parent.bounds_of(nth)
        } else {
            self.object.bounds_of(nth - count)
        }
    }

    fn value_of(&self, nth: usize) -> R {
        let count = self.parent.count();
        if nth < count {
            self.parent.value_of(nth)
        } else {
            self.object.value_of(nth - count)
        }
    }

    fn interact_with(&mut self, nth: usize) -> R {
        let count = self.parent.count();
        if nth < count {
            self.parent.interact_with(nth)
        } else {
            self.object.interact_with(nth - count)
        }
    }

    fn selectable(&self, nth: usize) -> bool {
        let count = self.parent.count();
        if nth < count {
            self.parent.selectable(nth)
        } else {
            self.object.selectable(nth - count)
        }
    }

    fn count(&self) -> usize {
        self.object.count() + self.parent.count()
    }

    fn draw_styled<D>(
        &self,
        text_style: &MonoTextStyle<'static, BinaryColor>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.parent.draw_styled(text_style, display)?;
        self.object.draw_styled(text_style, display)?;

        Ok(())
    }
}
