use embedded_graphics::{prelude::Point, primitives::Rectangle};

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
