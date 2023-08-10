use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point},
    primitives::Rectangle,
    Drawable,
};
use embedded_layout::View;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Insets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Insets {
    pub fn grow(self, rect: Rectangle) -> Rectangle {
        let bottom_right = rect.bottom_right().unwrap_or(rect.top_left);
        Rectangle::with_corners(
            Point::new(rect.top_left.x - self.left, rect.top_left.y - self.top),
            Point::new(bottom_right.x + self.right, bottom_right.y + self.bottom),
        )
    }
}

#[derive(Clone, Copy)]
pub struct Margin<V: View> {
    pub inner: V,
    pub insets: Insets,
}

pub trait MarginExt: View + Sized {
    fn with_margin(self, top: i32, right: i32, bottom: i32, left: i32) -> Margin<Self>;
}

impl<T> MarginExt for T
where
    T: View,
{
    fn with_margin(self, top: i32, right: i32, bottom: i32, left: i32) -> Margin<Self> {
        Margin::new(
            self,
            Insets {
                top,
                right,
                bottom,
                left,
            },
        )
    }
}

impl<V: View> Margin<V> {
    pub fn new(inner: V, insets: Insets) -> Self {
        Self { inner, insets }
    }
}

impl<V: View> View for Margin<V> {
    /// Move the origin of an object by a given number of (x, y) pixels,
    /// by returning a new object
    fn translate_impl(&mut self, by: Point) {
        self.inner.translate_mut(by);
    }

    /// Returns the bounding box of the `View` as a `Rectangle`
    fn bounds(&self) -> Rectangle {
        let bounds = self.inner.bounds();
        self.insets.grow(bounds)
    }
}

impl<C, V> Drawable for Margin<V>
where
    C: PixelColor,
    V: Drawable<Color = C> + View,
{
    type Color = C;
    type Output = V::Output;

    /// Draw the graphics object using the supplied DrawTarget.
    fn draw<D>(&self, display: &mut D) -> Result<V::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        self.inner.draw(display)
    }
}
