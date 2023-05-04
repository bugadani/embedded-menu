//! Run using `cargo run --example small --target x86_64-pc-windows-msvc`
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
use std::{thread, time::Duration};

use embedded_menu::{
    interaction::InteractionType,
    items::{select::SelectValue, NavigationItem, Select},
    ConstrainedDrawTarget, MenuBuilder,
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
    let mut menu = MenuBuilder::<_, _, _, _>::new("Menu", display_area)
        .show_details_after(300)
        .add_item(NavigationItem::new(
            "Foo",
            "Some longer description text",
            (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this",
            "Description",
            false,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this",
            "Description",
            false,
            |_| (),
            BinaryColor::On,
        ))
        .add_item(Select::new(
            "Check this2",
            "Description",
            TestEnum::A,
            |_| (),
            BinaryColor::On,
        ))
        .build();

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    'running: loop {
        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
        let mut sub = ConstrainedDrawTarget::new(
            &mut display,
            Rectangle::new(Point::new(16, 16), Size::new(96, 33)),
        );
        menu.draw(&mut sub).unwrap();
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
