//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc`

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
    interaction::InteractionType,
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
    let display_area = Rectangle::new(Point::zero(), Size::new(128, 64));
    let mut menu = Menu::builder("Menu", display_area)
        .show_details_after(300)
        .with_animated_selection_indicator(10)
        .add_item(
            NavigationItem::new("Foo", ())
                .with_marker(">")
                .with_detail_text("Some longer description text"),
        )
        .add_item(Select::new("Check this", false).with_detail_text("Description"))
        .add_item(Select::new("Check this", false).with_detail_text("Description"))
        .add_item(Select::new("Check this too", TestEnum::A).with_detail_text("Description"))
        .add_item(Select::new("Check this too", TestEnum::A).with_detail_text("Description"))
        .add_item(Select::new("Check this", true).with_detail_text("Description"))
        .add_item(Select::new("Check this too", true).with_detail_text("Description"))
        .add_item(Select::new("Check this too", TestEnum::A).with_detail_text("Description"))
        .add_item(Select::new("Check this", false).with_detail_text("Description"))
        .add_item(Select::new("Check this too", true).with_detail_text("Description"))
        .add_item(
            NavigationItem::new("Foo", ())
                .with_marker(">")
                .with_detail_text("Some longer description text"),
        )
        .add_item(Select::new("Check this", false).with_detail_text("Description"))
        .add_item(Select::new("Check this too", TestEnum::A).with_detail_text("Description"))
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
                    _ => None,
                },
                SimulatorEvent::Quit => break 'running,
                _ => None,
            };
        }
    }

    Ok(())
}
