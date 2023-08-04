Unreleased (??)
===============

 - **breaking**: Renamed `MenuExt` to `MenuItemCollection`
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
