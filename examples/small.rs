//! Run using `cargo run --example small --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{
    prelude::{DrawTargetExt, Point, Size},
    primitives::Rectangle,
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
        MenuStyle::default().with_input_adapter(Simulator {
            page_size: 5,
            esc_value: (),
        }),
    )
    .add_item("Foo", ">", |_| ())
    .add_item("Check this", false, |_| ())
    .add_item("Check this", false, |_| ())
    .add_item("Check this too", false, |_| ())
    .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    'running: loop {
        let mut display = SimulatorDisplay::new(Size::new(128, 64));
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
