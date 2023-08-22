//! Run using `cargo run --example scrolling --target x86_64-pc-windows-msvc` --features=simulator

use embedded_graphics::{pixelcolor::BinaryColor, prelude::Size, Drawable};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_menu::{
    interaction::simulator::Simulator,
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

#[derive(Clone, Copy)]
struct MenuData {
    slice_data: [bool; 5],
    select: TestEnum,
}

#[derive(Clone, Copy)]
enum MenuEvent {
    SliceCheckbox(usize, bool),
    Select(TestEnum),
    Nothing,
}

fn do_loop(
    window: &mut Window,
    state: &mut MenuState<Simulator, AnimatedPosition, Line>,
    data: &mut MenuData,
    item_count: usize,
) -> bool {
    let style = MenuStyle::new(BinaryColor::On)
        .with_input_adapter(Simulator::default())
        .with_animated_selection_indicator(10);

    let title = format!("{item_count} items");

    for _ in 0..60 {
        let mut items = (0..item_count)
            .map(|i| {
                Select::new("Changing", data.slice_data[i])
                    .with_value_converter(match i {
                        0 => |data| MenuEvent::SliceCheckbox(0, data),
                        1 => |data| MenuEvent::SliceCheckbox(1, data),
                        2 => |data| MenuEvent::SliceCheckbox(2, data),
                        3 => |data| MenuEvent::SliceCheckbox(3, data),
                        4 => |data| MenuEvent::SliceCheckbox(4, data),
                        _ => panic!(),
                    })
                    .with_detail_text("Description")
            })
            .take(item_count)
            .collect::<Vec<_>>();

        let mut menu = Menu::with_style(&title, style)
            .add_item(
                NavigationItem::new("Foo", MenuEvent::Nothing)
                    .with_marker(">")
                    .with_detail_text("Some longer description text"),
            )
            .add_items(&mut items)
            .add_item(
                Select::new("Check this too", data.select)
                    .with_detail_text("Description")
                    .with_value_converter(MenuEvent::Select),
            )
            .build_with_state(*state);

        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

        menu.update(&display);
        menu.draw(&mut display).unwrap();
        window.update(&display);

        for event in window.events() {
            if let Some(change) = menu.interact(event) {
                match change {
                    MenuEvent::SliceCheckbox(idx, value) => data.slice_data[idx] = value,
                    MenuEvent::Select(select) => data.select = select,
                    MenuEvent::Nothing => {}
                }
            }

            match event {
                SimulatorEvent::Quit => return false,
                _ => continue,
            }
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

    let mut data = MenuData {
        slice_data: [false; 5],
        select: TestEnum::A,
    };

    'running: loop {
        for items in 1..6 {
            if !do_loop(&mut window, &mut state, &mut data, items) {
                break 'running;
            }
        }
        for items in (2..5).rev() {
            if !do_loop(&mut window, &mut state, &mut data, items) {
                break 'running;
            }
        }
        for items in 1..6 {
            if !do_loop(&mut window, &mut state, &mut data, items) {
                break 'running;
            }
        }
    }

    Ok(())
}
