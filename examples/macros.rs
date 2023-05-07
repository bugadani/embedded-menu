//! Run using `cargo run --example simple --target x86_64-pc-windows-msvc`
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::InteractionType, items::select::SelectValue,
    selection_indicator::style::animated_triangle::AnimatedTriangle, Menu, MenuStyle, SelectValue,
};

#[derive(Copy, Clone, Debug, PartialEq, SelectValue)]
pub enum TestEnum {
    A,
    #[display_as("AB")]
    B,
    #[display_as("ABC")]
    C,
}

#[derive(Clone, Copy, PartialEq)]
pub enum NavEvents {
    Quit,
}

#[derive(Clone, Copy, Debug, Menu)]
#[menu(
    title = "Menu title",
    navigation(events = NavEvents, marker = ">"),
    items = [
        data(label = "Multiple select", field = test_field, details = "Some description"),
        data(label = "Checkbox", field = checkbox),
        navigation(label = "Quit", details = "Exits the demo", event = NavEvents::Quit)
    ]
)]
pub struct DemoMenu {
    test_field: TestEnum,
    checkbox: bool,
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut menu = DemoMenu {
        test_field: TestEnum::B,
        checkbox: false,
    }
    .create_menu_with_style(
        MenuStyle::new(BinaryColor::On)
            .with_details_delay(250)
            .with_animated_selection_indicator(10)
            .with_selection_indicator(AnimatedTriangle::new(200)),
    );

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
            let interaction = match event {
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Return => Some(InteractionType::Select),
                    Keycode::Up => Some(InteractionType::Previous),
                    Keycode::Down => Some(InteractionType::Next),
                    _ => None,
                },
                SimulatorEvent::Quit => break 'running,
                _ => None,
            };

            let Some(interaction) = interaction else { continue; };
            let output = menu.interact(interaction);
            let Some(output) = output else { continue; };

            match output {
                NavEvents::Quit => break 'running,
            }
        }
    }

    let final_data = menu.data();

    println!("{final_data:?}");

    Ok(())
}
