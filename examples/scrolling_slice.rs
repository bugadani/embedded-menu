//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc` --features=simulator

use embedded_graphics::{prelude::Size, Drawable};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_menu::{
    interaction::simulator::Simulator,
    items::{menu_item::SelectValue, MenuItem},
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

    fn marker(&self) -> &'static str {
        match self {
            TestEnum::A => "A",
            TestEnum::B => "AB",
            TestEnum::C => "ABC",
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let style = MenuStyle::default()
        .with_input_adapter(Simulator {
            page_size: 5,
            esc_value: (),
        })
        .with_animated_selection_indicator(10);

    let mut selects1 = [
        MenuItem::new("Check this 1", false),
        MenuItem::new("Check this 2", false),
    ];
    let mut selects2 = [
        MenuItem::new("Check this 3", true),
        MenuItem::new("Check this 4", true),
    ];

    let mut menu = Menu::with_style("Menu", style)
        .add_item("Foo", ">", |_| ())
        .add_menu_items(&mut selects1)
        .add_menu_items(&mut selects2)
        .add_item("Foo", "<-", |_| ())
        .add_item("Check this", false, |_| ())
        .add_item("Check this too", TestEnum::A, |_| ())
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    'running: loop {
        let mut display = SimulatorDisplay::new(Size::new(128, 64));
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
