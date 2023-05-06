use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, PixelColor, Point, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle, StyledDrawable, Triangle},
    transform::Transform,
    Drawable,
};
use embedded_layout::prelude::{horizontal::LeftToRight, vertical::Center, Align};

use crate::{Animated, MenuStyle};

pub mod simple;

pub struct Insets {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Insets {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

#[derive(Clone, Copy)]
pub enum IndicatorStyle {
    Line,
    Border,
    Triangle,
}

impl IndicatorStyle {
    pub fn margin(&self, height: u32) -> Insets {
        match self {
            IndicatorStyle::Line => Insets::new(2, 0, 0, 0),
            IndicatorStyle::Border => Insets::new(2, 1, 1, 1),
            IndicatorStyle::Triangle => Insets::new(height as i32 / 2 + 1, 0, 0, 0),
        }
    }
}

impl IndicatorStyle {
    fn draw<D>(&self, fill_width: u32, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();
        let fill = Rectangle::new(
            display_area.top_left,
            Size::new(fill_width, display_area.size.height),
        );
        match self {
            IndicatorStyle::Line => {
                Rectangle::new(fill.top_left, fill.size.component_max(Size::new(1, 0)))
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(display)
            }
            IndicatorStyle::Border => {
                display_area
                    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                    .draw(display)?;
                fill.into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(display)
            }
            IndicatorStyle::Triangle => {
                // TODO: describe as a single shape, use this shape to invert area
                fill.into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                    .draw(display)?;
                const SHRINK: i32 = 1;
                Triangle::new(
                    Point::new(0, SHRINK),
                    Point::new(0, display_area.size.height as i32 - 1 - SHRINK),
                    Point::new(
                        display_area.size.height as i32 / 2 - SHRINK,
                        display_area.size.height as i32 / 2,
                    ),
                )
                .align_to(&fill, LeftToRight, Center)
                // e-layout doesn't align well to 0 area rectangles
                // TODO: animate
                .translate(Point::new(if fill.is_zero_sized() { -1 } else { 0 }, 0))
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(display)
            }
        }
    }
}

pub trait SelectionIndicatorController {
    fn update_target(&mut self, y: i32);
    fn offset(&self) -> i32;
    fn update(&mut self);
}

pub struct StaticPosition {
    y_offset: i32,
}

impl StaticPosition {
    pub fn new() -> Self {
        Self { y_offset: 0 }
    }
}

impl SelectionIndicatorController for StaticPosition {
    fn update_target(&mut self, y: i32) {
        self.y_offset = y;
    }

    fn offset(&self) -> i32 {
        self.y_offset
    }

    fn update(&mut self) {}
}

pub struct AnimatedPosition {
    y_offset: Animated,
}

impl AnimatedPosition {
    pub fn new(frames: i32) -> Self {
        Self {
            y_offset: Animated::new(0, frames),
        }
    }
}

impl SelectionIndicatorController for AnimatedPosition {
    fn update_target(&mut self, y: i32) {
        self.y_offset.update_target(y);
    }

    fn offset(&self) -> i32 {
        self.y_offset.current()
    }

    fn update(&mut self) {
        self.y_offset.update()
    }
}

pub trait SelectionIndicator: Sized {
    type Color: PixelColor;
    type Controller: SelectionIndicatorController;

    fn position(&self) -> &Self::Controller;
    fn position_mut(&mut self) -> &mut Self::Controller;

    fn item_height(&self, menuitem_height: u32) -> u32;

    fn draw<D>(
        &self,
        indicator_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &mut D,
        items: &impl StyledDrawable<MenuStyle<Self::Color>, Color = Self::Color, Output = ()>,
        style: &MenuStyle<Self::Color>,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>;
}
