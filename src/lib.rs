//! Procedural macro for "deriving" a Bevy [`Plugin`] from a function.

use quote::quote;
use syn::{parse_macro_input, ItemFn};


use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;


#[proc_macro_attribute]
pub fn bevy_plugin(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(func as ItemFn);
    generate_bevy_plugin_type(item_fn).into()
}


fn generate_bevy_plugin_type(item_fn: ItemFn) -> TokenStream2 {
    // TODO: validate signature (single App argument, no return type)
    // XXX: make sure the name of Plugin::build argument is the same as the one in input function

    let vis = item_fn.vis;
    // TODO: allow passing a rename="" attribute to the #[bevy_plugin] attribute
    let ident = item_fn.sig.ident;
    let body = item_fn.block;
    // TODO: support generics through PhantomData

    // TODO: include #[allow(non_camel_case_types)] if the name isn't camel-case
    quote! {
        #[derive(Default)]
        #vis struct #ident;

        impl ::bevy::app::Plugin for #ident {
            fn build(&self, app: &mut ::bevy::app::App) {
                #body
            }
        }
    }
}
