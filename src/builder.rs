use crate::{
    interaction::{programmed::Programmed, InteractionController},
    plumbing::MenuExt,
    private::NoItems,
    Animated, Menu, MenuDisplayMode, MenuItem, MenuStyle,
};
use core::marker::PhantomData;
use embedded_graphics::{
    pixelcolor::{BinaryColor, PixelColor},
    primitives::Rectangle,
};
use embedded_layout::{layout::linear::LinearLayout, prelude::*, view_group::ViewGroup};

pub struct MenuBuilder<IT, LL, R, C>
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
    idle_timeout: Option<u16>,
    style: MenuStyle<C>,
}

impl<R: Copy> MenuBuilder<Programmed, NoItems, R, BinaryColor> {
    pub fn new(title: &'static str, bounds: Rectangle) -> Self {
        Self {
            _return_type: PhantomData,
            title,
            bounds,
            items: NoItems,
            interaction: Programmed::new(),
            idle_timeout: None,
            style: MenuStyle::new(BinaryColor::On, BinaryColor::On),
        }
    }
}

impl<IT, LL, R, C> MenuBuilder<IT, LL, R, C>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
{
    pub fn show_details_after(self, timeout: u16) -> MenuBuilder<IT, LL, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction: self.interaction,
            idle_timeout: Some(timeout),
            style: self.style,
        }
    }

    pub fn with_interaction_controller<ITC: InteractionController>(
        self,
        interaction: ITC,
    ) -> MenuBuilder<ITC, LL, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }

    pub fn with_style<CC: PixelColor>(self, style: MenuStyle<CC>) -> MenuBuilder<IT, LL, R, CC> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items,
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style,
        }
    }
}

impl<IT, VG, R, C> MenuBuilder<IT, VG, R, C>
where
    R: Copy,
    IT: InteractionController,
    VG: ViewGroup + MenuExt<R>,
    C: PixelColor,
{
    pub fn build(self) -> Menu<IT, VG, R, C> {
        const ANIM_FRAMES: i32 = 10;
        Menu {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            selected: (ViewGroup::len(&self.items) as u32).saturating_sub(1),
            items: LinearLayout::vertical(self.items).arrange().into_inner(),
            interaction: self.interaction,
            recompute_targets: true,
            list_offset: 0,
            indicator_offset: Animated::new(0, ANIM_FRAMES),
            idle_timeout_threshold: self.idle_timeout,
            idle_timeout: self.idle_timeout.unwrap_or_default(),
            display_mode: MenuDisplayMode::List,
            style: self.style,
        }
    }
}

impl<IT, R, C> MenuBuilder<IT, NoItems, R, C>
where
    R: Copy,
    IT: InteractionController,
    C: PixelColor,
{
    pub fn add_item<I: MenuItem<Data = R>>(self, item: I) -> MenuBuilder<IT, Chain<I>, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: Chain::new(item),
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }
}

impl<IT, CE, R, C> MenuBuilder<IT, Chain<CE>, R, C>
where
    R: Copy,
    IT: InteractionController,
    CE: MenuItem<Data = R>,
    C: PixelColor,
{
    pub fn add_item<I: MenuItem<Data = R>>(
        self,
        item: I,
    ) -> MenuBuilder<IT, Link<I, Chain<CE>>, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items.append(item),
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }
}

impl<IT, P, CE, R, C> MenuBuilder<IT, Link<P, CE>, R, C>
where
    R: Copy,
    IT: InteractionController,
    P: MenuItem<Data = R>,
    CE: MenuExt<R>,
    C: PixelColor,
{
    pub fn add_item<I: MenuItem<Data = R>>(
        self,
        item: I,
    ) -> MenuBuilder<IT, Link<I, Link<P, CE>>, R, C> {
        MenuBuilder {
            _return_type: PhantomData,
            title: self.title,
            bounds: self.bounds,
            items: self.items.append(item),
            interaction: self.interaction,
            idle_timeout: self.idle_timeout,
            style: self.style,
        }
    }
}
