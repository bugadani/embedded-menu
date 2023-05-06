use crate::{
    interaction::{programmed::Programmed, InteractionController},
    items::MenuLine,
    plumbing::MenuExt,
    selection_indicator::{
        style::IndicatorStyle, AnimatedPosition, Indicator, SelectionIndicatorController,
        StaticPosition,
    },
    Menu, MenuDisplayMode, MenuItem, MenuStyle, NoItems,
};
use core::marker::PhantomData;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_layout::{layout::linear::LinearLayout, prelude::*, view_group::ViewGroup};

pub struct MenuBuilder<IT, LL, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
    S: IndicatorStyle,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    items: LL,
    interaction: IT,
    style: MenuStyle<C, S>,
    indicator_controller: P,
}

impl<R, C, S> MenuBuilder<Programmed, NoItems, R, C, StaticPosition, S>
where
    R: Copy,
    C: PixelColor,
    S: IndicatorStyle,
{
    pub fn new(title: &'static str, style: MenuStyle<C, S>) -> Self {
        Self {
            _return_type: PhantomData,
            title,
            items: NoItems,
            interaction: Programmed,
            style,
            indicator_controller: StaticPosition::new(),
        }
    }
}

impl<IT, LL, R, C, P, S> MenuBuilder<IT, LL, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn with_animated_selection_indicator(
        self,
        frames: i32,
    ) -> MenuBuilder<IT, LL, R, C, AnimatedPosition, S> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items,
            interaction: self.interaction,
            style: self.style,
            indicator_controller: AnimatedPosition::new(frames),
        }
    }

    pub fn with_interaction_controller<ITC: InteractionController>(
        self,
        interaction: ITC,
    ) -> MenuBuilder<ITC, LL, R, C, P, S> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items,
            interaction,
            style: self.style,
            indicator_controller: self.indicator_controller,
        }
    }
}

impl<IT, VG, R, C, P, S> MenuBuilder<IT, VG, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    VG: ViewGroup + MenuExt<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn build(self) -> Menu<IT, VG, R, C, P, S> {
        Menu {
            _return_type: PhantomData,
            title: self.title,
            selected: (ViewGroup::len(&self.items) as u32).saturating_sub(1),
            items: LinearLayout::vertical(self.items).arrange().into_inner(),
            interaction: self.interaction,
            recompute_targets: true,
            list_offset: 0,
            indicator: Indicator::new(
                self.indicator_controller,
                self.style.indicator_style.clone(),
            ),
            idle_timeout: self.style.details_delay.unwrap_or_default(),
            display_mode: MenuDisplayMode::List,
            style: self.style,
        }
    }
}

impl<IT, R, C, P, S> MenuBuilder<IT, NoItems, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I: MenuItem<Data = R>>(
        self,
        item: I,
    ) -> MenuBuilder<IT, Chain<MenuLine<I>>, R, C, P, S> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: Chain::new(MenuLine::new(item, self.style)),
            interaction: self.interaction,
            style: self.style,
            indicator_controller: self.indicator_controller,
        }
    }
}

impl<IT, CE, R, C, P, S> MenuBuilder<IT, Chain<CE>, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    Chain<CE>: MenuExt<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I: MenuItem<Data = R>>(
        self,
        item: I,
    ) -> MenuBuilder<IT, Link<MenuLine<I>, Chain<CE>>, R, C, P, S> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items.append(MenuLine::new(item, self.style)),
            interaction: self.interaction,
            style: self.style,
            indicator_controller: self.indicator_controller,
        }
    }
}

impl<IT, I, CE, R, C, P, S> MenuBuilder<IT, Link<I, CE>, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    Link<I, CE>: MenuExt<R>,
    CE: MenuExt<R>,
    C: PixelColor,
    P: SelectionIndicatorController,
    S: IndicatorStyle,
{
    pub fn add_item<I2: MenuItem<Data = R>>(
        self,
        item: I2,
    ) -> MenuBuilder<IT, Link<MenuLine<I2>, Link<I, CE>>, R, C, P, S> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            items: self.items.append(MenuLine::new(item, self.style)),
            interaction: self.interaction,
            style: self.style,
            indicator_controller: self.indicator_controller,
        }
    }
}
