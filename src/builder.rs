use crate::{
    collection::{MenuItemCollection, MenuItems},
    interaction::InteractionController,
    selection_indicator::{style::IndicatorStyle, Indicator, SelectionIndicatorController},
    Menu, MenuDisplayMode, MenuItem, MenuStyle, NoItems,
};
use core::marker::PhantomData;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_layout::{
    layout::linear::LinearLayout, object_chain::ChainElement, prelude::*, view_group::ViewGroup,
};

pub struct MenuBuilder<IT, LL, R, C, P, S>
where
    IT: InteractionController,
    C: PixelColor,
    S: IndicatorStyle,
    P: SelectionIndicatorController,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    items: LL,
    style: MenuStyle<C, S, IT, P>,
}

impl<R, C, S, IT, P> MenuBuilder<IT, NoItems, R, C, P, S>
where
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
    pub const fn new(title: &'static str, style: MenuStyle<C, S, IT, P>) -> Self {
        Self {
            _return_type: PhantomData,
            title,
            items: NoItems,
            style,
        }
    }
}

impl<IT, R, C, P, S> MenuBuilder<IT, NoItems, R, C, P, S>
where
    IT: InteractionController,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I: MenuItem<R, MenuStyle<C, S, IT, P>>>(
        self,
        mut item: I,
    ) -> MenuBuilder<IT, Chain<I>, R, C, P, S> {
        item.set_style(&self.style);

        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: Chain::new(item),
            style: self.style,
        }
    }

    pub fn add_items<I: MenuItem<R, MenuStyle<C, S, IT, P>>>(
        self,
        item: &mut [I],
    ) -> MenuBuilder<IT, Chain<MenuItems<'_, I, R, MenuStyle<C, S, IT, P>>>, R, C, P, S> {
        item.iter_mut().for_each(|i| i.set_style(&self.style));

        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: Chain::new(MenuItems::new(item)),
            style: self.style,
        }
    }
}

impl<IT, CE, R, C, P, S> MenuBuilder<IT, Chain<CE>, R, C, P, S>
where
    IT: InteractionController,
    Chain<CE>: MenuItemCollection<R, MenuStyle<C, S, IT, P>>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I: MenuItem<R, MenuStyle<C, S, IT, P>>>(
        self,
        mut item: I,
    ) -> MenuBuilder<IT, Link<I, Chain<CE>>, R, C, P, S> {
        item.set_style(&self.style);

        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items.append(item),
            style: self.style,
        }
    }

    pub fn add_items<I: MenuItem<R, MenuStyle<C, S, IT, P>>>(
        self,
        item: &mut [I],
    ) -> MenuBuilder<IT, Link<MenuItems<'_, I, R, MenuStyle<C, S, IT, P>>, Chain<CE>>, R, C, P, S>
    {
        item.iter_mut().for_each(|i| i.set_style(&self.style));

        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items.append(MenuItems::new(item)),
            style: self.style,
        }
    }
}

impl<IT, I, CE, R, C, P, S> MenuBuilder<IT, Link<I, CE>, R, C, P, S>
where
    IT: InteractionController,
    Link<I, CE>: MenuItemCollection<R, MenuStyle<C, S, IT, P>> + ChainElement,
    CE: MenuItemCollection<R, MenuStyle<C, S, IT, P>> + ChainElement,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I2: MenuItem<R, MenuStyle<C, S, IT, P>>>(
        self,
        mut item: I2,
    ) -> MenuBuilder<IT, Link<I2, Link<I, CE>>, R, C, P, S> {
        item.set_style(&self.style);

        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items.append(item),
            style: self.style,
        }
    }

    pub fn add_items<I2: MenuItem<R, MenuStyle<C, S, IT, P>>>(
        self,
        item: &mut [I2],
    ) -> MenuBuilder<IT, Link<MenuItems<'_, I2, R, MenuStyle<C, S, IT, P>>, Link<I, CE>>, R, C, P, S>
    {
        item.iter_mut().for_each(|i| i.set_style(&self.style));

        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items.append(MenuItems::new(item)),
            style: self.style,
        }
    }
}

impl<IT, VG, R, C, P, S> MenuBuilder<IT, VG, R, C, P, S>
where
    IT: InteractionController,
    VG: ViewGroup + MenuItemCollection<R, MenuStyle<C, S, IT, P>>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn build(self) -> Menu<IT, VG, R, C, P, S> {
        Menu {
            _return_type: PhantomData,
            title: self.title,
            selected: self.items.count().saturating_sub(1),
            items: LinearLayout::vertical(self.items).arrange().into_inner(),
            interaction_state: Default::default(),
            recompute_targets: true,
            list_offset: 0,
            indicator: Indicator::new(self.style.indicator_controller, self.style.indicator_style),
            idle_timeout: self.style.details_delay.unwrap_or_default(),
            display_mode: MenuDisplayMode::List,
            style: self.style,
        }
    }
}
