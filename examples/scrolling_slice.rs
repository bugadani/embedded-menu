//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc` --features=simulator

use embedded_graphics::{prelude::Size, Drawable};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_menu::{
    interaction::simulator::Simulator, items::MenuItem, Menu, MenuStyle, SelectValue,
};

#[derive(Copy, Clone, PartialEq, SelectValue)]
pub enum TestEnum {
    A,
    B,
    C,
}

fn main() -> Result<(), core::convert::Infallible> {
    let style = MenuStyle::default()
        .with_input_adapter(Simulator {
            page_size: 5,
            esc_value: (),
        })
        .with_animated_selection_indicator(10);

    let selects1 = [
        MenuItem::new("Check this 1", false),
        MenuItem::new("Check this 2", false),
    ];
    let selects2 = [
        MenuItem::new("Check this 3", true),
        MenuItem::new("Check this 4", true),
    ];

    let mut menu = Menu::with_style("Menu", style)
        .add_item("Foo", ">", |_| ())
        .add_menu_items(selects1)
        .add_menu_items(selects2)
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
