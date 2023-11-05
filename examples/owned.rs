//! Run using `cargo run --example simple --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using up/down arrows, interact using the Enter key

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
    // Use a generator to create owned strings that we give to the menu.
    let items: Vec<MenuItem<_, _, _, true>> = (1..10)
        .map(|i| format!("Item {}", i))
        .map(|i| MenuItem::new(i, false))
        .collect();

    let mut menu = Menu::with_style(
        format!("Items: {}", items.len()),
        MenuStyle::default().with_input_adapter(Simulator {
            page_size: 5,
            esc_value: (),
        }),
    )
    .add_item("Foo", ">", |_| ())
    .add_menu_items(items)
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
