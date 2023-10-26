//! Run using `cargo run --example simple --target x86_64-pc-windows-msvc`
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::items::SectionTitle;
use embedded_menu::{
    interaction::{Action, Interaction, Navigation},
    items::{select::SelectValue, NavigationItem, Select},
    Menu,
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
    let mut menu = Menu::new("Menu")
        .add_item(NavigationItem::new("Foo", 1).with_marker(">"))
        .add_item(Select::new("Check this 1", false).with_value_converter(|b| 20 + b as i32))
        .add_item(SectionTitle::new("===== Section ====="))
        .add_item(Select::new("Check this 2", false).with_value_converter(|b| 30 + b as i32))
        .add_item(Select::new("Check this 3", TestEnum::A).with_value_converter(|b| 40 + b as i32))
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    let mut selected_value: i32 = 0;

    'running: loop {
        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
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
