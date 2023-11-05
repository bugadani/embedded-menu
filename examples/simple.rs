//! Run using `cargo run --example simple --target x86_64-pc-windows-msvc`
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::{Action, Interaction, Navigation},
    Menu, SelectValue,
};

#[derive(Copy, Clone, PartialEq, SelectValue)]
pub enum TestEnum {
    A,
    B,
    C,
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut menu = Menu::build("Menu")
        .add_item("Foo", ">", |_| 1)
        .add_item("Check this 1", false, |b| 20 + b as i32)
        .add_section_title("===== Section =====")
        .add_item("Check this 2", false, |b| 30 + b as i32)
        .add_item("Check this 3", TestEnum::A, |b| 40 + b as i32)
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    let mut selected_value: i32 = 0;

    'running: loop {
        let mut display = SimulatorDisplay::new(Size::new(128, 64));
        menu.update(&display);
        menu.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::Return => menu.interact(Interaction::Action(Action::Select)),
                    Keycode::Up => menu.interact(Interaction::Navigation(Navigation::Previous)),
                    Keycode::Down => menu.interact(Interaction::Navigation(Navigation::Next)),
                    _ => None,
                },
                SimulatorEvent::Quit => break 'running,
                _ => None,
            };
        }

        let selected = menu.selected_value();
        if selected != selected_value {
            println!("Selected value: {}", selected);
            selected_value = selected;
        }
    }

    Ok(())
}
