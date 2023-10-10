use crate::{
    collection::{MenuItemCollection, MenuItems},
    interaction::{InputAdapterSource, InputState},
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    Menu, MenuDisplayMode, MenuItem, MenuState, MenuStyle, NoItems,
};
use core::marker::PhantomData;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_layout::{
    layout::linear::LinearLayout,
    object_chain::ChainElement,
    prelude::*,
    view_group::{EmptyViewGroup, ViewGroup},
};

pub struct MenuBuilder<T, IT, LL, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    C: PixelColor,
    S: IndicatorStyle,
    P: SelectionIndicatorController,
{
    title: T,
    items: LL,
    style: MenuStyle<C, S, IT, P, R>,
}

impl<T, R, C, S, IT, P> MenuBuilder<T, IT, NoItems, R, C, P, S>
where
    T: AsRef<str>,
    C: PixelColor,
    S: IndicatorStyle,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
{
    pub const fn new(title: T, style: MenuStyle<C, S, IT, P, R>) -> Self {
        Self {
            title,
            items: NoItems,
            style,
        }
    }
}

impl<T, IT, R, C, P, S> MenuBuilder<T, IT, NoItems, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I: MenuItem<R>>(self, mut item: I) -> MenuBuilder<T, IT, Chain<I>, R, C, P, S> {
        item.set_style(&self.style);

        MenuBuilder {
            title: self.title,
            items: Chain::new(item),
            style: self.style,
        }
    }

    pub fn add_items<I, IC>(
        self,
        mut items: IC,
    ) -> MenuBuilder<T, IT, Chain<MenuItems<IC, I, R>>, R, C, P, S>
    where
        I: MenuItem<R>,
        IC: AsRef<[I]> + AsMut<[I]>,
    {
        items
            .as_mut()
            .iter_mut()
            .for_each(|i| i.set_style(&self.style));

        MenuBuilder {
            title: self.title,
            items: Chain::new(MenuItems::new(items)),
            style: self.style,
        }
    }
}

impl<T, IT, CE, R, C, P, S> MenuBuilder<T, IT, Chain<CE>, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    Chain<CE>: MenuItemCollection<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I: MenuItem<R>>(
        self,
        mut item: I,
    ) -> MenuBuilder<T, IT, Link<I, Chain<CE>>, R, C, P, S> {
        item.set_style(&self.style);

        MenuBuilder {
            title: self.title,
            items: self.items.append(item),
            style: self.style,
        }
    }

    pub fn add_items<I, IC>(
        self,
        mut items: IC,
    ) -> MenuBuilder<T, IT, Link<MenuItems<IC, I, R>, Chain<CE>>, R, C, P, S>
    where
        I: MenuItem<R>,
        IC: AsRef<[I]> + AsMut<[I]>,
    {
        items
            .as_mut()
            .iter_mut()
            .for_each(|i| i.set_style(&self.style));

        MenuBuilder {
            title: self.title,
            items: self.items.append(MenuItems::new(items)),
            style: self.style,
        }
    }
}

impl<T, IT, I, CE, R, C, P, S> MenuBuilder<T, IT, Link<I, CE>, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    Link<I, CE>: MenuItemCollection<R> + ChainElement,
    CE: MenuItemCollection<R> + ChainElement,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I2: MenuItem<R>>(
        self,
        mut item: I2,
    ) -> MenuBuilder<T, IT, Link<I2, Link<I, CE>>, R, C, P, S> {
        item.set_style(&self.style);

        MenuBuilder {
            title: self.title,
            items: self.items.append(item),
            style: self.style,
        }
    }

    pub fn add_items<I2, IC>(
        self,
        mut items: IC,
    ) -> MenuBuilder<T, IT, Link<MenuItems<IC, I2, R>, Link<I, CE>>, R, C, P, S>
    where
        I2: MenuItem<R>,
        IC: AsRef<[I2]> + AsMut<[I2]>,
    {
        items
            .as_mut()
            .iter_mut()
            .for_each(|i| i.set_style(&self.style));

        MenuBuilder {
            title: self.title,
            items: self.items.append(MenuItems::new(items)),
            style: self.style,
        }
    }
}

impl<T, IT, VG, R, C, P, S> MenuBuilder<T, IT, VG, R, C, P, S>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: ViewGroup + MenuItemCollection<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn build(self) -> Menu<T, IT, VG, R, C, P, S> {
        let default_timeout = self.style.details_delay.unwrap_or_default();

        self.build_with_state(MenuState {
            selected: 0,
            list_offset: 0,
            display_mode: MenuDisplayMode::List(default_timeout),
            interaction_state: Default::default(),
            indicator_state: Default::default(),
            last_input_state: InputState::Idle,
        })
    }

    pub fn build_with_state(
        mut self,
        mut state: MenuState<IT::InputAdapter, P, S>,
    ) -> Menu<T, IT, VG, R, C, P, S> {
        // We have less menu items than before. Avoid crashing.
        let max_idx = self.items.count().saturating_sub(1);

        LinearLayout::vertical(EmptyViewGroup).arrange_view_group(&mut self.items);

        if max_idx < state.selected {
            state.selected = max_idx;

            let max_indicator_pos = MenuItemCollection::bounds_of(&self.items, max_idx)
                .top_left
                .y;

            self.style
                .indicator
                .change_selected_item(max_indicator_pos, &mut state.indicator_state);
            self.style
                .indicator
                .jump_to_target(&mut state.indicator_state);
        }

        Menu {
            state,
            _return_type: PhantomData,
            title: self.title,
            items: self.items,
            style: self.style,
        }
    }
}
