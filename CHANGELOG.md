Unreleased
==========

## New

 - Added initial color support
 - Added `MenuBuilder::add_section_title`
 - Style a menu by using a `Theme` object

## Changed

 - Replaced the `Invert` selection indicator with `Rectangle`
 - Renamed `SelectValue::name` to `marker`
 - Renamed `MenuItem` trait to `MenuListItem`
 - Moved `MenuListItem` and `Marker` to `items`
 - Renamed `Select` to `MenuItem`
 - `MenuListItem::draw_styled` and `MenuListItem::set_style` now only takes a font style instead of the whole menu style
 - Renamed `MenuBuilder::add_item(s)` to `add_menu_item(s)`
 - Renamed `Menu::new` to `build`
 - `SelectValue::marker` can now return values with non-`'static` lifetimes
 - `SelectValue` now requires `Clone` instead of `Copy`
 - `SelectValue::next` now takes `&mut self` and returns nothing

## Removed

 - Removed the concept of menu item descriptions. This change removes the following APIs:
   - `MenuItemCollection::details_of`
   - `MenuItem::details`
   - `NavigationItem::with_detail_text`
   - `Select::with_detail_text`
   - `MenuStyle::with_details_delay`
   - `Select` and `NavigationItem` now have one fewer type parameters
 - Removed `MenuItem::title`
 - Removed `MenuItem::value`
 - Removed `MenuItemCollection::title_of`
 - Removed `derive(Menu)`
 - Removed the `R` type parameter from `SectionTitle`
 - Removed `IndicatorStyle::Color`, `IndicatorStyle::color`, `IndicatorStyle::draw` is now generic over the theme
 - Removed `NavigationItem`
 - Removed `SectionTitle`

0.5.4 (2023-10-27)
==================

 - New: allow `Menu` to return the selected item's value without interacting with it.

0.5.3 (2023-10-19)
==================

 - New: `SectionTitle` menu item type.

0.5.2 (2023-10-14)
==================

 - Internal improvements

0.5.1 (2023-10-13)
==================

 - Internal improvements

0.5.0 (2023-10-10)
==================

 - Updated embedded-layout dependency to 0.4.0

0.4.4 (2023-10-10)
==================

 - Internal improvements

0.4.3 (2023-10-06)
==================

 - Fix incorrect selection indicator positioning when restoring menu state

0.4.2 (2023-10-06)
==================

 - Internal improvements

0.4.1 (2023-09-24)
==================

 - Fixed menu item details not showing up

0.4.0 (2023-08-23)
==================

 - `View` is now a supertrait of `MenuItem`
 - Fixed issues with displaying a slice of menuitems
 - Added `MenuState`, `MenuBuilder::build_with_state` and `Menu::state`
 - `Menu`, `Select` and `NavigationItem` are now generic over their string parameters
 - `Menu::add_items` now accepts owning collections (e.g. `Vec`)
 - Replaced `StyledDrawable` wtih `MenuItem::draw_styled` and `MenuItemCollection::draw_styled`
 - Empty menu titles are no longer displayed
 - Reworked input handling. `InteractionController` has been replaced by `InputAdapter`
 - Added more options for `InteractionType`
 - Added `selection_indicator::invert::Invert`
 - Renamed `MenuStyle::with_interaction_controller` to `with_input_adapter`
 - Added the `simulator` feature and the `interaction::simulator::Simulator` input adapter

0.3.1 (2023-08-06)
==================

 - The ˙collection` and `styled` modules are now public.

0.3.0 (2023-08-04)
==================

 - Add `MenuBuilder::add_items` that takes a slice of menu items.
 - **breaking**: Renamed `MenuExt` to `MenuItemCollection`
 - **breaking**: Changed associate type on `MenuItem` into a generic parameter.
 - Removed `SingleTouch::new`.
 - Fields of `SingleTouch` are now public.
 - **breaking**: Removed `MenuLine` from the menu type signature.
 - **breaking**: `MenuLine` is no longer generic.
 - Added `MenuItem::set_style` which is used internally to set up a menu item.

0.2.0 (2023-06-23)
==================

 - Removed unused `display-interface` dependency.
 - Single touch menu items now fire contiuously while held.
 - Single touch interaction now ignores initial held input.
 - Single touch interaction no longer selects next item after selecting a menu item.
 - **breaking**: `InteractionController::update` now takes `&mut self`.

0.1.0
=====

 - Initial release
