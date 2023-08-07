//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc`

use std::iter::repeat_with;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::{programmed::Programmed, InteractionType},
    items::{select::SelectValue, NavigationItem, Select},
    selection_indicator::{style::line::Line, AnimatedPosition},
    Menu, MenuState, MenuStyle,
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

    fn name(&self) -> &'static str {
        match self {
            TestEnum::A => "A",
            TestEnum::B => "AB",
            TestEnum::C => "ABC",
        }
    }
}

fn do_loop(
    window: &mut Window,
    state: &mut MenuState<Programmed, AnimatedPosition, Line>,
    item_count: usize,
) -> bool {
    let style = MenuStyle::new(BinaryColor::On).with_animated_selection_indicator(10);

    let title = format!("{item_count} items");

    let mut items = repeat_with(|| Select::new("Changing", false).with_detail_text("Description"))
        .take(item_count)
        .collect::<Vec<_>>();

    for _ in 0..60 {
        let mut menu = Menu::with_style(unsafe { std::mem::transmute(title.as_str()) }, style)
            .add_item(
                NavigationItem::new("Foo", ())
                    .with_marker(">")
                    .with_detail_text("Some longer description text"),
            )
            .add_items(&mut items)
            .add_item(Select::new("Check this too", TestEnum::A).with_detail_text("Description"))
            .build_with_state(*state);

        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

        menu.update(&display);
        menu.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => match keycode {
                    Keycode::Return => menu.interact(InteractionType::Select),
                    Keycode::Up => menu.interact(InteractionType::Previous),
                    Keycode::Down => menu.interact(InteractionType::Next),
                    _ => None,
                },
                SimulatorEvent::Quit => return false,
                _ => None,
            };
        }

        *state = menu.state();
    }

    true
}

fn main() -> Result<(), core::convert::Infallible> {
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Menu demonstration", &output_settings);

    let mut state = Default::default();

    'running: loop {
        for items in 1..6 {
            if !do_loop(&mut window, &mut state, items) {
                break 'running;
            }
        }
        for items in (2..5).rev() {
            if !do_loop(&mut window, &mut state, items) {
                break 'running;
            }
        }
        for items in 1..6 {
            if !do_loop(&mut window, &mut state, items) {
                break 'running;
            }
        }
    }

    Ok(())
}
