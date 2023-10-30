//! Run using `cargo run --example color --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{
    pixelcolor::{Rgb888, Rgb555},
    prelude::{DrawTargetExt, Point, Size},
    primitives::Rectangle,
    Drawable,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_menu::{
    interaction::simulator::Simulator,
    items::{select::SelectValue, NavigationItem, Select},
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
    let mut menu = Menu::with_style(
        "Color Menu",
        MenuStyle::new(Rgb888::new(51, 255, 51), Rgb555::new(255, 0, 0)).with_input_adapter(
            Simulator {
                page_size: 5,
                esc_value: (),
            },
        ),
    )
    .add_item(NavigationItem::new("Foo", ()).with_marker(">"))
    .add_item(Select::new("Check this", false))
    .add_item(Select::new("Check this", false))
    .add_item(Select::new("Check this too", false))
    .build();

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("Menu demonstration w/color", &output_settings);

    'running: loop {
        let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(128, 64));
        let mut sub = display.cropped(&Rectangle::new(Point::new(16, 16), Size::new(96, 34)));
        menu.update(&sub);
        menu.draw(&mut sub).unwrap();
        window.update(&display);

        for event in window.events() {
            menu.interact(event);

            match event {
                SimulatorEvent::Quit => break 'running,
                _ => continue,
            }
        }
    }

    Ok(())
}
