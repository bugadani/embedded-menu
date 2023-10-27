//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc` --features=simulator

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
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
    let style = MenuStyle::new(BinaryColor::On)
        .with_input_adapter(Simulator {
            page_size: 5,
            esc_value: (),
        })
        .with_animated_selection_indicator(10);

    let mut selects1 = [
        Select::new("Check this 1", false),
        Select::new("Check this 2", false),
    ];
    let mut selects2 = [
        Select::new("Check this 3", true),
        Select::new("Check this 4", true),
    ];

    let mut menu = Menu::with_style("Menu", style)
        .add_item(NavigationItem::new("Foo", ()).with_marker(">"))
        .add_items(&mut selects1)
        .add_items(&mut selects2)
        .add_item(NavigationItem::new("Foo", ()).with_marker(">"))
        .add_item(Select::new("Check this", false))
        .add_item(Select::new("Check this too", TestEnum::A))
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
            menu.interact(event);

            match event {
                SimulatorEvent::Quit => break 'running,
                _ => continue,
            }
        }
    }

    Ok(())
}
