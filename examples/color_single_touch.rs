//! Run using `cargo run --example color --target x86_64-pc-windows-msvc` --features=simulator
//!
//! Navigate using up/down arrows, interact using the Enter key

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Point, RgbColor, Size},
    primitives::Rectangle,
    Drawable,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_menu::{
    interaction::single_touch::SingleTouch,
    items::{select::SelectValue, Select},
    selection_indicator::style::AnimatedTriangle,
    theme::Theme,
    Menu, MenuStyle,
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

    fn marker(&self) -> &'static str {
        match self {
            TestEnum::A => "A",
            TestEnum::B => "AB",
            TestEnum::C => "ABC",
        }
    }
}

#[derive(Clone, Copy)]
struct ExampleTheme;

impl Theme for ExampleTheme {
    type Color = Rgb888;

    fn text_color(&self) -> Self::Color {
        Rgb888::WHITE
    }

    fn selected_text_color(&self) -> Self::Color {
        Rgb888::BLACK
    }

    fn selection_color(&self) -> Self::Color {
        Rgb888::new(51, 255, 51)
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut menu = Menu::with_style(
        "Color Menu",
        MenuStyle::new(ExampleTheme)
            .with_selection_indicator(AnimatedTriangle::new(160))
            .with_input_adapter(SingleTouch {
                ignore_time: 10,
                debounce_time: 1,
                max_time: 100,
            }),
    )
    .add_item(Select::new("Foo", ">"))
    .add_item(Select::new("Check this", false))
    .add_item(Select::new("Check this", false))
    .add_item(Select::new("Check this too", false))
    .build();

    let output_settings = OutputSettingsBuilder::new().scale(4).build();
    let mut window = Window::new("Menu demonstration w/color", &output_settings);

    let mut space_pressed = false;
    'running: loop {
        let mut display = SimulatorDisplay::new(Size::new(128, 64));
        let mut sub = display.cropped(&Rectangle::new(Point::new(16, 16), Size::new(96, 34)));
        menu.update(&sub);
        menu.draw(&mut sub).unwrap();
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::KeyDown {
                    keycode: Keycode::Space,
                    ..
                } => space_pressed = true,
                SimulatorEvent::KeyUp {
                    keycode: Keycode::Space,
                    ..
                } => space_pressed = false,
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        menu.interact(space_pressed);
    }

    Ok(())
}
