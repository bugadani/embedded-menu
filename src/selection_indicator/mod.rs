use crate::{
    adapters::invert::BinaryColorDrawTargetExt, selection_indicator::style::IndicatorStyle,
    MenuStyle,
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
    current: i32,
    target: i32,
    frames: i32,
}

impl AnimatedPosition {
    pub fn new(frames: i32) -> Self {
        Self {
            current: 0,
            target: 0,
            frames,
        }
    }
}

impl SelectionIndicatorController for AnimatedPosition {
    fn update_target(&mut self, y: i32) {
        self.target = y;
    }

    fn offset(&self) -> i32 {
        self.current
    }

    fn update(&mut self) {
        let rounding = if self.current < self.target {
            self.frames - 1
        } else {
            1 - self.frames
        };

        let distance = self.target - self.current;
        self.current += (distance + rounding) / self.frames;
    }
}

pub(crate) struct Indicator<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    position: P,
    style: S,
    state: S::State,
}

impl<P, S> Indicator<P, S>
where
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn new(position: P, style: S) -> Self {
        Self {
            position,
            style,
            state: Default::default(),
        }
    }

    pub fn offset(&self) -> i32 {
        self.position.offset()
    }

    pub fn change_selected_item(&mut self, pos: i32) {
        self.position.update_target(pos);
        self.style.on_target_changed(&mut self.state);
    }

    pub fn update(&mut self, fill_width: u32) {
        self.position.update();
        self.style.update(&mut self.state, fill_width);
    }

    pub fn item_height(&self, menuitem_height: u32) -> u32 {
        let indicator_insets = self.style.margin(&self.state, menuitem_height);
        (menuitem_height as i32 + indicator_insets.top + indicator_insets.bottom) as u32
    }

    pub fn draw<D>(
        &self,
        selected_height: u32,
        screen_offset: i32,
        fill_width: u32,
        display: &mut D,
        items: &impl StyledDrawable<MenuStyle<BinaryColor, S>, Color = BinaryColor, Output = ()>,
        style: &MenuStyle<BinaryColor, S>,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let Insets {
            left: margin_left,
            top: margin_top,
            right: margin_right,
            bottom: margin_bottom,
        } = self.style.margin(&self.state, selected_height);

        self.style.draw(
            &self.state,
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
            &self.state,
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
