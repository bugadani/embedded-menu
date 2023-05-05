//! Run using `cargo run --example details_dev --target x86_64-pc-windows-msvc`

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

use embedded_menu::{interaction::InteractionType, items::NavigationItem, Menu};

fn main() -> Result<(), core::convert::Infallible> {
    let display_area = Rectangle::new(Point::zero(), Size::new(128, 64));
    let mut menu = Menu::builder("Menu", display_area)
        .show_details_after(100)
        .add_item(
            NavigationItem::new("Foo", ())
                .with_marker(">")
                .with_detail_text("Some longer     description text\nfoo\n  foo")
                .bind(BinaryColor::On),
        )
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

        let mut had_interaction = false;
        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::Return => {
                        menu.interact(InteractionType::Select);
                        had_interaction = true;
                    }
                    Keycode::Up => {
                        menu.interact(InteractionType::Previous);
                        had_interaction = true;
                    }
                    Keycode::Down => {
                        menu.interact(InteractionType::Next);
                        had_interaction = true;
                    }
                    _ => {}
                },
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        if !had_interaction {
            menu.interact(InteractionType::Nothing);
        }
    }

    Ok(())
}
