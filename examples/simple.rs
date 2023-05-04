//! Run using `cargo run --example simple --target x86_64-pc-windows-msvc`
//!
//! Navigate using up/down arrows, interact using the Enter key

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
use embedded_menu::{
    interaction::InteractionType,
    items::{select::SelectValue, NavigationItem, Select},
    MenuBuilder,
};

#[derive(Copy, Clone)]
pub enum TestEnum {
    A,
    B,
    C,
}

impl SelectValue for TestEnum {
    fn next(self) -> Self {
        match self {
            TestEnum::A => TestEnum::B,
            TestEnum::B => TestEnum::C,
            TestEnum::C => TestEnum::A,
        }
    }

    fn name(self) -> &'static str {
        match self {
            TestEnum::A => "A",
            TestEnum::B => "AB",
            TestEnum::C => "ABC",
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let display_area = Rectangle::new(Point::zero(), Size::new(128, 64));
    let mut menu = MenuBuilder::new("Menu", display_area)
        .show_details_after(300)
        .add_item(NavigationItem::new(
            ">",
            "Foo",
            "Lorem ipsum dolor sit amet, in per offendit assueverit adversarium, no sed clita adipisci nominati. Veritus placerat efficiantur mel ea. In splendide reformidans eos. In corpora inciderint duo, unum laudem constituto vis id, in iisque habemus quo. Pri nisl consul facilis te, percipitur deterruisset ne eum.",
            (),
            BinaryColor::On
        ))
        .add_item(Select::new("Check this", "Description", false, |_| (), BinaryColor::On))
        .add_item(Select::new("Check this", "Description", false, |_| (), BinaryColor::On))
        .add_item(Select::new(
            "Check this2",
            "Description",
            TestEnum::A,
            |_| (),
            BinaryColor::On
        ))
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
    }

    Ok(())
}
