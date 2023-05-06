use crate::{
    adapters::invert::BinaryColorDrawTargetExt,
    selection_indicator::style::{line::Line, IndicatorStyle},
    Animated, MenuStyle,
};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, DrawTargetExt, Point, Size},
    primitives::{Rectangle, StyledDrawable},
};

pub mod style;

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

pub(crate) struct Indicator<P, S> {
    position: P,
    style: S,
}

impl Indicator<StaticPosition, Line> {
    pub fn new() -> Self {
        Self {
            position: StaticPosition::new(),
            style: Line,
        }
    }
}

impl<P, S> Indicator<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn with_indicator_style<S2: IndicatorStyle>(self, style: S2) -> Indicator<P, S2> {
        Indicator {
            position: self.position,
            style,
        }
    }

    pub fn with_animated_selection_indicator(self, frames: i32) -> Indicator<AnimatedPosition, S> {
        Indicator {
            position: AnimatedPosition::new(frames),
            style: self.style,
        }
    }

    pub fn offset(&self) -> i32 {
        self.position.offset()
    }

    pub fn change_selected_item(&mut self, pos: i32) {
        self.position.update_target(pos);
        self.style.on_target_changed();
    }

    pub fn update(&mut self, fill_width: u32) {
        self.position.update();
        self.style.update(fill_width);
    }

    pub fn item_height(&self, menuitem_height: u32) -> u32 {
        let indicator_insets = self.style.margin(menuitem_height);
        (menuitem_height as i32 + indicator_insets.top + indicator_insets.bottom) as u32
    }

    pub fn draw<D>(
        &self,
        selected_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &mut D,
        items: &impl StyledDrawable<MenuStyle<BinaryColor>, Color = BinaryColor, Output = ()>,
        style: &MenuStyle<BinaryColor>,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
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

        let mut inverting = display.invert_area(&self.style.shape(
            Rectangle::new(
                Point::new(0, screen_offset),
                Size::new(fill_width, selected_height),
            ),
            fill_width,
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
