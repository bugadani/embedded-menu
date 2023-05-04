//! Run using `cargo run --example scrolling_single_touch --target x86_64-pc-windows-msvc`
//!
//! Navigate using only the spacebar. Short(ish) press moves on to the next item, long press activates.
//! Watch the animated selection indicator fill up. Long press is registered as the bar reaches full width.

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::Rectangle,
    Drawable,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::single_touch::SingleTouch,
    items::{select::SelectValue, NavigationItem, Select},
    MenuBuilder,
};

#[derive(Copy, Clone)]
pub enum TestEnum {
    A,
    B,
    C,
}

impl SelectValue for TestEnum {
    fn next(self) -> Self {
        match self {
            TestEnum::A => TestEnum::B,
            TestEnum::B => TestEnum::C,
            TestEnum::C => TestEnum::A,
        }
    }

    fn name(self) -> &'static str {
        match self {
            TestEnum::A => "A",
            TestEnum::B => "AB",
            TestEnum::C => "ABC",
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let display_area = Rectangle::new(Point::zero(), Size::new(128, 64));
    let mut menu = MenuBuilder::new("Menu", display_area)
        .show_details_after(300)
        .with_interaction_controller(SingleTouch::new(5, 100))
        .add_item(NavigationItem::new(
            ">",
            "Foo",
            "Some longer description text",
            (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this",
            "Description",
            false,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this",
            "Description",
            false,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this2",
            "Description",
            TestEnum::A,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this2",
            "Description",
            TestEnum::A,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this",
            "Description",
            false,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this2",
            "Description",
            true,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this2",
            "Description",
            TestEnum::A,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this",
            "Description",
            false,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this2",
            "Description",
            true,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(NavigationItem::new(
            ">",
            "Foo",
            "Description",
            (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this",
            "Description",
            false,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this2",
            "Description",
            true,
            |_| (),
            BinaryColor::On,
        ))
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    let mut space_pressed = false;
    'running: loop {
        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
        menu.update(&display);
        menu.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown {
                    keycode: Keycode::Space,
                    ..
                } => space_pressed = true,
                SimulatorEvent::KeyUp {
                    keycode: Keycode::Space,
                    ..
                } => space_pressed = false,
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }

        menu.interact(space_pressed);
    }

    Ok(())
}
