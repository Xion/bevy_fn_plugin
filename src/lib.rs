//! Procedural macro for "deriving" a Bevy [`Plugin`] from a function.

#[macro_use] extern crate quote;
#[macro_use] extern crate syn;


use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn bevy_plugin(attr: TokenStream, func: TokenStream) -> TokenStream {
    func
}
