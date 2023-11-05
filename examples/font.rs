//! Run using `cargo run --example simple --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13_BOLD, iso_8859_1::FONT_6X10},
    prelude::Size,
    Drawable,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_menu::{interaction::simulator::Simulator, Menu, MenuStyle, SelectValue};

#[derive(Copy, Clone, PartialEq, SelectValue)]
pub enum TestEnum {
    A,
    B,
    C,
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut menu = Menu::with_style(
        "Menu",
        MenuStyle::default()
            .with_font(&FONT_6X10)
            .with_title_font(&FONT_8X13_BOLD)
            .with_input_adapter(Simulator {
                page_size: 5,
                esc_value: (),
            }),
    )
    .add_item("Nav item", "»", |_| ()) // » is not part of the ASCII font
    .add_item("Checkbox", true, |_| ())
    .add_item("Other checkbox", false, |_| ())
    .add_item("Multiple options long", TestEnum::A, |_| ())
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
