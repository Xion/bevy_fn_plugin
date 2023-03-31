//! Procedural macro for "deriving" a Bevy [`Plugin`] from a function.

use quote::quote;
use syn::{parse_macro_input, FnArg, Ident, ItemFn, Pat, PatType, Signature, Type, ReturnType};


use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use proc_macro2::TokenStream as TokenStream2;


#[proc_macro_attribute]
#[proc_macro_error]
pub fn bevy_plugin(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(func as ItemFn);
    generate_bevy_plugin_type(item_fn).into()
}


fn generate_bevy_plugin_type(item_fn: ItemFn) -> TokenStream2 {
    if !is_signature_valid(&item_fn.sig) {
        abort! {
            item_fn.sig,
            "Invalid plugin function signature";
            help = "Function should take a single argument of the type `&mut bevy::app::App`";
        }
    }
    if !item_fn.sig.generics.params.is_empty() {
        // TODO: support generics through PhantomData
        abort! {
            item_fn.sig.generics,
            "Generic functions are not supported yet"
        }
    }

    let vis = item_fn.vis;
    // TODO: allow passing a rename="" attribute to the #[bevy_plugin] attribute
    let ident = item_fn.sig.ident;
    let body = item_fn.block;

    let app_arg_name = typed_fn_arg_ident(&item_fn.sig.inputs[0])
        .map(|ident| quote! { #ident })
        .unwrap_or_else(|| quote! { _ });

    // TODO: include #[allow(non_camel_case_types)] if the name isn't camel-case
    quote! {
        #[derive(Default)]
        #vis struct #ident;

        impl ::bevy::app::Plugin for #ident {
            fn build(&self, #app_arg_name: &mut ::bevy::app::App) {
                #body
            }
        }
    }
}


fn is_signature_valid(sig: &Signature) -> bool {
    if sig.inputs.len() != 1 {
        return false
    }

    // Check the sole argument's type.
    let Some(at_mut_app) = fn_arg_as_mut_ref_type(&sig.inputs[0]) else { return false };
    let Type::Path(app) = at_mut_app else { return false };
    if app.path.segments.is_empty() {
        return false;
    }
    if app.path.segments.last().unwrap().ident.to_string() != "App" {
        return false;
    }

    // Check the return value (or rather, lack thereof).
    match sig.output {
        ReturnType::Default => {},
        ReturnType::Type(_, _) => {
            // TODO: check if the type is `()` because that's also valid
            // (technically, that `()` may also be aliased to something else, but we can't support
            // obnoxious cases like that without type information)
            return false
        },
    }

    true
}

#[inline]
fn fn_arg_as_mut_ref_type(fn_arg: &FnArg) -> Option<&Type> {
    let FnArg::Typed(PatType { ty, .. }) = fn_arg else { return None };
    let Type::Reference(ref ref_ty) = **ty else { return None };
    if ref_ty.mutability.is_none() {
        return None;
    }
    Some(&*ref_ty.elem)
}


#[inline]
fn typed_fn_arg_ident(fn_arg: &FnArg) -> Option<&Ident> {
    let FnArg::Typed(PatType { pat, .. }) = fn_arg else { return None };
    let Pat::Ident(ref pat_ident) = **pat else { return None };
    Some(&pat_ident.ident)
}
