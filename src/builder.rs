use crate::{
    interaction::{programmed::Programmed, InteractionController},
    items::MenuLine,
    plumbing::MenuExt,
    private::NoItems,
    selection_indicator::{
        style::{line::Line, IndicatorStyle},
        AnimatedPosition, Indicator, SelectionIndicatorController, StaticPosition,
    },
    Menu, MenuDisplayMode, MenuItem, MenuStyle,
};
use core::marker::PhantomData;
use embedded_graphics::{pixelcolor::PixelColor, primitives::Rectangle};
use embedded_layout::{layout::linear::LinearLayout, prelude::*, view_group::ViewGroup};

pub struct MenuBuilder<IT, LL, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
{
    _return_type: PhantomData<R>,
    title: &'static str,
    bounds: Rectangle,
    items: LL,
    interaction: IT,
    style: MenuStyle<C>,
    indicator: Indicator<P, S>,
}

impl<R, C> MenuBuilder<Programmed, NoItems, R, C, StaticPosition, Line>
where
    R: Copy,
    C: PixelColor,
{
    pub fn new(title: &'static str, bounds: Rectangle, style: MenuStyle<C>) -> Self {
        Self {
            _return_type: PhantomData,
            title,
            bounds,
            items: NoItems,
            interaction: Programmed,
            style,
            indicator: Indicator::new(),
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
    pub fn with_selection_indicator_style<S2>(self, style: S2) -> MenuBuilder<IT, LL, R, C, P, S2>
    where
        S2: IndicatorStyle,
    {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction: self.interaction,
            style: self.style,
            indicator: self.indicator.with_indicator_style(style),
        }
    }

    pub fn with_animated_selection_indicator(
        self,
        frames: i32,
    ) -> MenuBuilder<IT, LL, R, C, AnimatedPosition, S> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction: self.interaction,
            style: self.style,
            indicator: self.indicator.with_animated_selection_indicator(frames),
        }
    }

    pub fn with_interaction_controller<ITC: InteractionController>(
        self,
        interaction: ITC,
    ) -> MenuBuilder<ITC, LL, R, C, P, S> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction,
            style: self.style,
            indicator: self.indicator,
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
            bounds: self.bounds,
            selected: (ViewGroup::len(&self.items) as u32).saturating_sub(1),
            items: LinearLayout::vertical(self.items).arrange().into_inner(),
            interaction: self.interaction,
            recompute_targets: true,
            list_offset: 0,
            indicator: self.indicator,
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
            bounds: self.bounds,
            items: Chain::new(MenuLine::new(item, self.style)),
            interaction: self.interaction,
            style: self.style,
            indicator: self.indicator,
        }
    }
}

impl<IT, CE, R, C, P, S> MenuBuilder<IT, Chain<CE>, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    CE: MenuItem<Data = R>,
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
            bounds: self.bounds,
            items: self.items.append(MenuLine::new(item, self.style)),
            interaction: self.interaction,
            style: self.style,
            indicator: self.indicator,
        }
    }
}

impl<IT, I, CE, R, C, P, S> MenuBuilder<IT, Link<I, CE>, R, C, P, S>
where
    R: Copy,
    IT: InteractionController,
    I: MenuItem<Data = R>,
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
            bounds: self.bounds,
            items: self.items.append(MenuLine::new(item, self.style)),
            interaction: self.interaction,
            style: self.style,
            indicator: self.indicator,
        }
    }
}
