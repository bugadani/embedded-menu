//! Run using `cargo run --example scrolling_single_touch --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using only the spacebar. Short(ish) press moves on to the next item, long press activates.
//! Watch the animated selection indicator fill up. Long press is registered as the bar reaches full width.

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::single_touch::SingleTouch,
    items::{select::SelectValue, NavigationItem, Select},
    selection_indicator::style::animated_triangle::AnimatedTriangle,
    Menu, MenuStyle,
};

#[derive(Copy, Clone, PartialEq)]
pub enum TestEnum {
    A,
    B,
    C,
}

impl SelectValue for TestEnum {
    fn next(&self) -> Self {
        match self {
            TestEnum::A => TestEnum::B,
            TestEnum::B => TestEnum::C,
            TestEnum::C => TestEnum::A,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            TestEnum::A => "A",
            TestEnum::B => "AB",
            TestEnum::C => "ABC",
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let style = MenuStyle::default()
        .with_selection_indicator(AnimatedTriangle::new(160))
        .with_input_adapter(SingleTouch {
            ignore_time: 10,
            debounce_time: 1,
            max_time: 100,
        })
        .with_animated_selection_indicator(10);

    let mut menu = Menu::with_style("Menu with even longer title", style)
        .add_item(NavigationItem::new("Foo", ()).with_marker(">"))
        .add_item(Select::new("Check this", false))
        .add_item(Select::new("Check this", false))
        .add_item(Select::new("Check this too", TestEnum::A))
        .add_item(Select::new("Check this too", TestEnum::A))
        .add_item(Select::new("Check this", true))
        .add_item(Select::new("Check this too", true))
        .add_item(Select::new("Check this too", TestEnum::A))
        .add_item(Select::new("Check this", false))
        .add_item(Select::new("Check this too", true))
        .add_item(NavigationItem::new("Foo", ()).with_marker(">"))
        .add_item(Select::new("Check this", false))
        .add_item(Select::new("Check this too", TestEnum::A))
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
