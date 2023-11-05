//! Run using `cargo run --example scrolling_single_touch --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using only the spacebar. Short(ish) press moves on to the next item, long press activates.
//! Watch the animated selection indicator fill up. Long press is registered as the bar reaches full width.

use embedded_graphics::{prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::single_touch::SingleTouch,
    selection_indicator::style::animated_triangle::AnimatedTriangle, Menu, MenuStyle, SelectValue,
};

#[derive(Copy, Clone, PartialEq, SelectValue)]
pub enum TestEnum {
    A,
    B,
    C,
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
        .add_item("Foo", ">", |_| ())
        .add_item("Check this", false, |_| ())
        .add_item("Check this", false, |_| ())
        .add_item("Check this too", TestEnum::A, |_| ())
        .add_item("Check this too", TestEnum::A, |_| ())
        .add_item("Check this", true, |_| ())
        .add_item("Check this too", true, |_| ())
        .add_item("Check this too", TestEnum::A, |_| ())
        .add_item("Check this", false, |_| ())
        .add_item("Check this too", true, |_| ())
        .add_item("Foo", "<-", |_| ())
        .add_item("Check this", false, |_| ())
        .add_item("Check this too", TestEnum::A, |_| ())
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    let mut space_pressed = false;
    'running: loop {
        let mut display = SimulatorDisplay::new(Size::new(128, 64));
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
