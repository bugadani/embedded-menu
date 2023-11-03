use crate::{
    collection::{MenuItemCollection, MenuItems},
    interaction::{InputAdapterSource, InputState},
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    Menu, MenuItem, MenuState, MenuStyle, NoItems,
};
use core::marker::PhantomData;
use embedded_graphics::prelude::PixelColor;
use embedded_layout::{
    layout::linear::LinearLayout,
    object_chain::ChainElement,
    prelude::*,
    view_group::{EmptyViewGroup, ViewGroup},
};

pub struct MenuBuilder<T, IT, LL, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    S: IndicatorStyle,
    P: SelectionIndicatorController,
    C: PixelColor,
{
    title: T,
    items: LL,
    style: MenuStyle<S, IT, P, R, C>,
}

impl<T, R, S, IT, P, C> MenuBuilder<T, IT, NoItems, R, P, S, C>
where
    T: AsRef<str>,
    S: IndicatorStyle,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
    C: PixelColor,
{
    pub const fn new(title: T, style: MenuStyle<S, IT, P, R, C>) -> Self {
        Self {
            title,
            items: NoItems,
            style,
        }
    }
}

impl<T, IT, R, P, S, C> MenuBuilder<T, IT, NoItems, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: PixelColor + Default + 'static,
{
    pub fn add_item<I: MenuItem<R>>(self, mut item: I) -> MenuBuilder<T, IT, Chain<I>, R, P, S, C> {
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
    ) -> MenuBuilder<T, IT, Chain<MenuItems<IC, I, R>>, R, P, S, C>
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

impl<T, IT, CE, R, P, S, C> MenuBuilder<T, IT, Chain<CE>, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    Chain<CE>: MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: PixelColor + Default + 'static,
{
    pub fn add_item<I: MenuItem<R>>(
        self,
        mut item: I,
    ) -> MenuBuilder<T, IT, Link<I, Chain<CE>>, R, P, S, C> {
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
    ) -> MenuBuilder<T, IT, Link<MenuItems<IC, I, R>, Chain<CE>>, R, P, S, C>
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

impl<T, IT, I, CE, R, P, S, C> MenuBuilder<T, IT, Link<I, CE>, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    Link<I, CE>: MenuItemCollection<R> + ChainElement,
    CE: MenuItemCollection<R> + ChainElement,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: PixelColor + Default + 'static,
{
    pub fn add_item<I2: MenuItem<R>>(
        self,
        mut item: I2,
    ) -> MenuBuilder<T, IT, Link<I2, Link<I, CE>>, R, P, S, C> {
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
    ) -> MenuBuilder<T, IT, Link<MenuItems<IC, I2, R>, Link<I, CE>>, R, P, S, C>
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

impl<T, IT, VG, R, P, S, C> MenuBuilder<T, IT, VG, R, P, S, C>
where
    T: AsRef<str>,
    IT: InputAdapterSource<R>,
    VG: ViewGroup + MenuItemCollection<R>,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
    C: PixelColor,
{
    pub fn build(self) -> Menu<T, IT, VG, R, P, S, C> {
        self.build_with_state(MenuState {
            selected: 0,
            list_offset: 0,
            interaction_state: Default::default(),
            indicator_state: Default::default(),
            last_input_state: InputState::Idle,
        })
    }

    pub fn build_with_state(
        mut self,
        mut state: MenuState<IT::InputAdapter, P, S>,
    ) -> Menu<T, IT, VG, R, P, S, C> {
        // We have less menu items than before. Avoid crashing.
        let max_idx = self.items.count().saturating_sub(1);

        LinearLayout::vertical(EmptyViewGroup).arrange_view_group(&mut self.items);

        state.set_selected_item(state.selected, &self.items, &self.style);
        if max_idx < state.selected {
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
