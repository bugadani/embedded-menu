//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc`

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};
use embedded_menu::{
    interaction::InteractionType,
    items::{select::SelectValue, NavigationItem, Select},
    Menu, MenuItem, MenuItemCollection, MenuStyle,
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

type Style = MenuStyle<
    BinaryColor,
    embedded_menu::selection_indicator::style::line::Line,
    embedded_menu::interaction::programmed::Programmed,
    embedded_menu::selection_indicator::AnimatedPosition,
>;

fn main() -> Result<(), core::convert::Infallible> {
    let style: Style = MenuStyle::new(BinaryColor::On).with_animated_selection_indicator(10);

    let mut item1 = NavigationItem::new("Foo", ())
        .with_marker(">")
        .with_detail_text("Some longer description text");

    let mut item2 = Select::new("Check this", false).with_detail_text("Description");
    let mut item3 = Select::new("Check this too", TestEnum::A).with_detail_text("Description");

    let mut items: [&mut dyn MenuItem<(), Style>; 3] = [&mut item1, &mut item2, &mut item3];

    let mut menu = Menu::with_style("Menu", style)
        .add_items(&mut items)
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
                SimulatorEvent::Quit => break 'running,
                _ => None,
            };
        }
    }

    Ok(())
}
