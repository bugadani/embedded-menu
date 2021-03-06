//! Run using `cargo run --example details_dev --target x86_64-pc-windows-msvc`

use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::{thread, time::Duration};

use embedded_graphics::{pixelcolor::BinaryColor, primitives::Rectangle};
use embedded_layout::prelude::*;

use embedded_menu::{
    interaction::InteractionType,
    items::{select::SelectValue, NavigationItem, Select},
    MenuBuilder,
};

fn main() -> Result<(), core::convert::Infallible> {
    let display_area = Rectangle::with_size(Point::zero(), Size::new(128, 64));
    let mut menu = MenuBuilder::<_, _, _, _>::new("Menu", display_area)
        .show_details_after(100)
        .add_item(NavigationItem::new(
            "Foo",
            "Some longer     description text\nfoo\n  foo",
            (),
            BinaryColor::On,
        ))
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    'running: loop {
        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
        menu.draw(&mut display).unwrap();
        window.update(&display);

        let mut had_interaction = false;
        for event in window.events() {
            match event {
                SimulatorEvent::KeyUp { keycode, .. } => match keycode {
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

        thread::sleep(Duration::from_millis(10));
    }

    Ok(())
}
