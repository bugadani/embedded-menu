use crate::{adapters::invert::BinaryColorDrawTargetExt, Animated, MenuStyle};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, DrawTargetExt, PixelColor, Point, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle, StyledDrawable, Triangle},
    transform::Transform,
    Drawable,
};
use embedded_layout::prelude::{horizontal::LeftToRight, vertical::Center, Align};

#[derive(Clone, Copy, PartialEq, Eq)]
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

pub struct Indicator<P> {
    position: P,
    style: IndicatorStyle,
}

impl Indicator<StaticPosition> {
    pub fn new() -> Self {
        Self {
            position: StaticPosition::new(),
            style: IndicatorStyle::Line,
        }
    }

    pub fn animated(frames: i32) -> Indicator<AnimatedPosition> {
        Indicator {
            position: AnimatedPosition::new(frames),
            style: IndicatorStyle::Line,
        }
    }
}

impl<P> Indicator<P> {
    pub fn with_indicator_style(self, style: IndicatorStyle) -> Self {
        Self { style, ..self }
    }
}

impl<P: SelectionIndicatorController> SelectionIndicator for Indicator<P> {
    type Color = BinaryColor;
    type Controller = P;

    fn position(&self) -> &Self::Controller {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Self::Controller {
        &mut self.position
    }

    fn item_height(&self, menuitem_height: u32) -> u32 {
        let indicator_insets = self.style.margin(menuitem_height);
        (menuitem_height as i32 + indicator_insets.top + indicator_insets.bottom) as u32
    }

    fn draw<D>(
        &self,
        selected_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &mut D,
        items: &impl StyledDrawable<MenuStyle<Self::Color>, Color = Self::Color, Output = ()>,
        style: &MenuStyle<Self::Color>,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let Insets {
            left: margin_left,
            top: margin_top,
            right: margin_right,
            bottom: margin_bottom,
        } = self.style.margin(selected_height);

        self.style.draw(
            fill_width,
            &mut display.cropped(&Rectangle::new(
                Point::new(0, screen_offset),
                Size::new(
                    display.bounding_box().size.width,
                    (selected_height as i32 + margin_top + margin_bottom) as u32,
                ),
            )),
        )?;

        let display_top_left = display.bounding_box().top_left;
        let display_size = display.bounding_box().size;

        let mut inverting = display.invert_area(&Rectangle::new(
            Point::new(0, screen_offset),
            Size::new(fill_width, selected_height),
        ));
        items.draw_styled(
            style,
            &mut inverting.cropped(&Rectangle::new(
                display_top_left + Point::new(margin_left, margin_top),
                Size::new(
                    (display_size.width as i32 - margin_left - margin_right) as u32,
                    display_size.height,
                ),
            )),
        )
    }
}
