use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    bracketed, parenthesized,
    parse::{Parse, ParseStream},
    Attribute, DataStruct, DeriveInput, Field, LitStr, Path, Token, Type,
};

struct DataItem {
    label: String,
    field: Ident,
}

impl Parse for DataItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut label = None;
        let mut field = None;

        while !input.is_empty() {
            let option = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;

            if option == "label" {
                label = Some(input.parse::<LitStr>()?.value());
            } else if option == "field" {
                field = Some(input.parse::<Ident>()?);
            } else {
                return Err(syn::Error::new_spanned(
                    &option,
                    format!("Unknown option \"{option}\""),
                ));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            label: label.unwrap(),
            field: field.unwrap(),
        })
    }
}

struct NavItem {
    label: String,
    event: Path,
}

impl Parse for NavItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut label = None;
        let mut event = None;

        while !input.is_empty() {
            let option = input.parse::<Ident>()?;
            input.parse::<Token![=]>()?;

            if option == "label" {
                label = Some(input.parse::<LitStr>()?.value());
            } else if option == "event" {
                event = Some(input.parse::<Path>()?);
            } else {
                return Err(syn::Error::new_spanned(
                    &option,
                    format!("Unknown option \"{option}\""),
                ));
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            label: label.unwrap(),
            event: event.unwrap(),
        })
    }
}

enum MenuItemOption {
    Navigation(NavItem),
    Data(DataItem),
}

impl Parse for MenuItemOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_kind = input.parse::<Ident>()?;

        let args;
        parenthesized!(args in input);

        if item_kind == "data" {
            Ok(Self::Data(args.parse::<DataItem>()?))
        } else if item_kind == "navigation" {
            Ok(Self::Navigation(args.parse::<NavItem>()?))
        } else {
            return Err(syn::Error::new_spanned(
                &item_kind,
                format!("Unknown menu item kind \"{item_kind}\""),
            ));
        }
    }
}

impl TryFrom<&Field> for MenuItemOption {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        let Some(ident) = field.ident.clone() else {
            return Err(syn::Error::new(
                Span::call_site(),
                "Menu can only be placed on named structs",
            ));
        };

        Ok(Self::Data(DataItem {
            label: ident.to_string(),
            field: ident,
        }))
    }
}

enum MenuItem {
    Nav(NavItem),
    Data { data: DataItem, ty: Path },
}
impl MenuItem {
    fn menu_item_in_ty(&self, events: &Ident) -> TokenStream {
        match self {
            MenuItem::Nav { .. } => {
                quote!(NavigationItem<&'static str, &'static str, #events>)
            }
            MenuItem::Data { ty, .. } => quote!(Select<&'static str, #events, #ty>),
        }
    }

    fn as_data_field(&self) -> Option<&Ident> {
        let Self::Data {
            data: DataItem { field, .. },
            ..
        } = self
        else {
            return None;
        };

        Some(field)
    }

    fn menu_item(&self, events: &Ident) -> TokenStream {
        match self {
            MenuItem::Nav(NavItem { label, event }) => quote! {
                NavigationItem::new(#label, #events::NavigationEvent(#event))
            },
            MenuItem::Data {
                data: DataItem { field, label },
                ..
            } => quote! {
                Select::new(#label, self.#field)
                    .with_value_converter(#events::#field),
            },
        }
    }

    fn as_enum_variant(&self) -> Option<TokenStream> {
        if let MenuItem::Data {
            data: DataItem { field, .. },
            ty,
        } = self
        {
            Some(quote!(#field(#ty)))
        } else {
            None
        }
    }
}

struct MenuOptions {
    title: String,
    items: Vec<MenuItemOption>,
    navigation_marker: Option<String>,
    navigation_event_ty: Option<Path>,
}

impl MenuOptions {
    fn contains(&self, ident: &Ident) -> bool {
        for item in self.items.iter() {
            if let MenuItemOption::Data(DataItem { field, .. }) = item {
                if field == ident {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for MenuOptions {
    fn default() -> Self {
        Self {
            title: String::from("Menu"),
            items: Vec::new(),
            navigation_marker: None,
            navigation_event_ty: None,
        }
    }
}

impl Parse for MenuOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut options = Self {
            title: String::new(),
            items: Vec::new(),
            navigation_marker: None,
            navigation_event_ty: None,
        };

        while !input.is_empty() {
            let option = input.parse::<Ident>()?;

            if option == "title" {
                if !options.title.is_empty() {
                    return Err(syn::Error::new_spanned(option, "Title is already set."));
                }

                let _ = input.parse::<Token![=]>()?;
                options.title = input.parse::<LitStr>()?.value();
            } else if option == "navigation" {
                let args;
                parenthesized!(args in input);

                while !args.is_empty() {
                    let option = args.parse::<Ident>()?;
                    args.parse::<Token![=]>()?;

                    if option == "events" {
                        if options.navigation_marker.is_some() {
                            return Err(syn::Error::new_spanned(
                                option,
                                "Event type is already set.",
                            ));
                        }
                        options.navigation_event_ty = Some(args.parse::<Path>()?);
                    } else if option == "marker" {
                        if options.navigation_marker.is_some() {
                            return Err(syn::Error::new_spanned(
                                option,
                                "Navigation marker is already set.",
                            ));
                        }
                        options.navigation_marker = Some(args.parse::<LitStr>()?.value());
                    } else {
                        return Err(syn::Error::new_spanned(
                            &option,
                            format!("Unknown option \"{option}\""),
                        ));
                    }

                    if !args.is_empty() {
                        args.parse::<Token![,]>()?;
                    }
                }
            } else if option == "items" {
                if !options.items.is_empty() {
                    return Err(syn::Error::new_spanned(option, "Items are already set."));
                }

                let _ = input.parse::<Token![=]>()?;

                let items;
                bracketed!(items in input);

                while !items.is_empty() {
                    let item = items.parse::<MenuItemOption>()?;

                    if !items.is_empty() {
                        let _ = items.parse::<Token![,]>()?;
                    }

                    options.items.push(item);
                }
            } else {
                return Err(syn::Error::new_spanned(
                    &option,
                    format!("Unknown option \"{option}\""),
                ));
            }

            if !input.is_empty() {
                let _ = input.parse::<Token![,]>()?;
            }
        }

        Ok(options)
    }
}

struct MenuData {
    ty_name: Ident,
    title: String,
    navigation_event_ty: Option<Path>,
    navigation_marker: Option<String>,
    items: Vec<MenuItem>,
}

impl TryFrom<MenuInput> for MenuData {
    type Error = syn::Error;

    fn try_from(input: MenuInput) -> Result<Self, Self::Error> {
        let ty_name = input.ident;

        let mut attributes = input
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("menu"));

        let attribute = attributes.next();

        if let Some(second) = attributes.next() {
            return Err(syn::Error::new_spanned(
                second,
                "Only one \"menu\" attribute is allowed",
            ));
        }

        let mut menu_options = if let Some(attribute) = attribute {
            attribute.parse_args()?
        } else {
            MenuOptions::default()
        };

        for item in menu_options.items.iter() {
            if let MenuItemOption::Data(DataItem { field, .. }) = item {
                if !input
                    .data
                    .fields
                    .iter()
                    .any(|f| f.ident.as_ref() == Some(field))
                {
                    return Err(syn::Error::new_spanned(
                        field,
                        format!("Field \"{field}\" is not a member of the struct"),
                    ));
                }
            }
        }

        // Collect undecorated fields at the bottom
        for field in input.data.fields.iter() {
            if !menu_options.contains(field.ident.as_ref().unwrap()) {
                menu_options.items.push(MenuItemOption::try_from(field)?);
            }
        }

        let mut items = Vec::new();

        for item in menu_options.items {
            match item {
                MenuItemOption::Navigation(nav) => items.push(MenuItem::Nav(nav)),
                MenuItemOption::Data(data) => {
                    for field in input.data.fields.iter() {
                        if field.ident.as_ref() == Some(&data.field) {
                            let Type::Path(ty) = &field.ty else {
                                return Err(syn::Error::new_spanned(
                                    field,
                                    "Field must be of type bool or enum",
                                ));
                            };

                            items.push(MenuItem::Data {
                                data,
                                ty: ty.path.clone(),
                            });
                            break;
                        }
                    }
                }
            }
        }

        Ok(Self {
            title: menu_options.title,
            ty_name,
            navigation_event_ty: menu_options.navigation_event_ty,
            items,
            navigation_marker: menu_options.navigation_marker,
        })
    }
}

struct MenuInput {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: DataStruct,
}

pub fn expand_menu(input: DeriveInput) -> syn::Result<TokenStream> {
    let syn::Data::Struct(data) = input.data else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Menu can only be placed on non-generic structs",
        ));
    };

    let input = MenuInput {
        ident: input.ident,
        attrs: input.attrs,
        data,
    };

    let menu_data = MenuData::try_from(input)?;

    let wrapper = format_ident!("{}MenuWrapper", menu_data.ty_name);
    let events = format_ident!("{}MenuEvents", menu_data.ty_name);
    let module = format_ident!("{}_module", menu_data.ty_name);

    let event_variants = menu_data
        .items
        .iter()
        .filter_map(|item| item.as_enum_variant());
    let menu_items_in_ty = menu_data
        .items
        .iter()
        .map(|item| item.menu_item_in_ty(&events));
    let event_set_data_fields = menu_data
        .items
        .iter()
        .filter_map(|item| item.as_data_field());
    let menu_items = menu_data.items.iter().map(|item| {
        let tokens = item.menu_item(&events);
        if matches!(item, MenuItem::Nav(..)) {
            let navigation_marker = menu_data.navigation_marker.iter();
            quote! { #tokens #(.with_marker(#navigation_marker))* }
        } else {
            tokens
        }
    });

    let title = menu_data.title;
    let ty_name = menu_data.ty_name;
    let navigation_event_ty = if let Some(ty) = menu_data.navigation_event_ty {
        quote! {#ty}
    } else {
        quote! {()}
    };

    Ok(quote! {
        #[allow(non_snake_case)]
        mod #module {
            use super::*;

            use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
            use embedded_layout::object_chain::*;
            use embedded_menu::{
                builder::MenuBuilder,
                interaction::{programmed::Programmed, InputAdapter, InputAdapterSource},
                items::{NavigationItem, Select},
                selection_indicator::{
                    style::{line::Line, IndicatorStyle},
                    SelectionIndicatorController, StaticPosition,
                },
                Menu, MenuStyle, NoItems,
            };

            #[derive(Clone, Copy)]
            #[allow(non_camel_case_types)]
            pub enum #events {
                NavigationEvent(#navigation_event_ty),
                #(#event_variants),*
            }

            pub struct #wrapper<IT, P, S>
            where
                IT: InputAdapterSource<#events>,
                P: SelectionIndicatorController,
                S: IndicatorStyle,
            {
                menu: Menu<
                    &'static str,
                    IT,
                    embedded_layout::chain! {
                        #(#menu_items_in_ty),*
                    },
                    #events,
                    BinaryColor,
                    P,
                    S,
                >,
                data: #ty_name,
            }

            impl<IT, P, S> #wrapper<IT, P, S>
            where
                IT: InputAdapterSource<#events>,
                P: SelectionIndicatorController,
                S: IndicatorStyle,
            {
                pub fn data(&self) -> &#ty_name {
                    &self.data
                }

                #[allow(unreachable_code)]
                pub fn interact(&mut self, event: <IT::InputAdapter as InputAdapter>::Input) -> Option<#navigation_event_ty> {
                    match self.menu.interact(event)? {
                        #(#events::#event_set_data_fields(value) => self.data.#event_set_data_fields = value,)*
                        #events::NavigationEvent(event) => return Some(event),
                    };

                    None
                }

                pub fn update(&mut self, display: &impl Dimensions) {
                    self.menu.update(display)
                }
            }

            impl<IT, P, S> Drawable for #wrapper<IT, P, S>
            where
                IT: InputAdapterSource<#events>,
                P: SelectionIndicatorController,
                S: IndicatorStyle,
            {
                type Color = BinaryColor;
                type Output = ();

                fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
                where
                    D: DrawTarget<Color = BinaryColor>,
                {
                    self.menu.draw(display)
                }
            }

            impl #ty_name {
                fn setup_menu<S, IT, P>(
                    self,
                    builder: MenuBuilder<&'static str, IT, NoItems, #events, BinaryColor, P, S>,
                ) -> #wrapper<IT, P, S>
                where
                    S: IndicatorStyle,
                    IT: InputAdapterSource<#events>,
                    P: SelectionIndicatorController,
                {
                    #wrapper {
                        data: self,
                        menu: builder
                            #(.add_item(#menu_items))*
                            .build(),
                    }
                }

                pub fn create_menu(self) -> #wrapper<Programmed, StaticPosition, Line> {
                    self.create_menu_with_style(MenuStyle::default())
                }

                pub fn create_menu_with_style<S, IT, P>(
                    self,
                    style: MenuStyle<BinaryColor, S, IT, P, #events>,
                ) -> #wrapper<IT, P, S>
                where
                    S: IndicatorStyle,
                    IT: InputAdapterSource<#events>,
                    P: SelectionIndicatorController,
                {
                    let builder = Menu::with_style(#title, style);
                    self.setup_menu(builder)
                }
            }
        }

        pub use #module::{#wrapper, #events};
    })
}
