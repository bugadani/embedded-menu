//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc`

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::InteractionType,
    items::{select::SelectValue, Select},
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
    let style = MenuStyle::new(BinaryColor::On).with_animated_selection_indicator(10);

    let mut menu = Menu::with_style("Menu", style)
        .add_item(Select::new(" 1 Check this", false).with_detail_text("Description"))
        .add_item(Select::new(" 2 Check this", false).with_detail_text("Description"))
        .add_item(Select::new(" 3 Check this", false).with_detail_text("Description"))
        .add_item(Select::new(" 4 Check this too", TestEnum::A).with_detail_text("Description"))
        .add_item(Select::new(" 5 Check this too", TestEnum::A).with_detail_text("Description"))
        .add_item(Select::new(" 6 Check this", true).with_detail_text("Description"))
        .add_item(Select::new(" 7 Check this too", true).with_detail_text("Description"))
        .add_item(Select::new(" 8 Check this too", TestEnum::A).with_detail_text("Description"))
        .add_item(Select::new(" 9 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("10 Check this too", true).with_detail_text("Description"))
        .add_item(Select::new("11 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("12 Check this too", TestEnum::A).with_detail_text("Description"))
        .add_item(Select::new("13 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("14 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("15 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("16 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("17 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("18 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("19 Check this", false).with_detail_text("Description"))
        .add_item(Select::new("20 Check this", false).with_detail_text("Description"))
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

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
                    Keycode::Return => menu.interact(InteractionType::Select),
                    Keycode::Up => menu.interact(InteractionType::Previous),
                    Keycode::Down => menu.interact(InteractionType::Next),
                    Keycode::PageDown => menu.interact(InteractionType::Forward(7)),
                    Keycode::PageUp => menu.interact(InteractionType::Backward(7)),
                    Keycode::Home => menu.interact(InteractionType::Beginning),
                    Keycode::End => menu.interact(InteractionType::End),
                    _ => None,
                },
                SimulatorEvent::Quit => break 'running,
                _ => None,
            };
        }
    }

    Ok(())
}
