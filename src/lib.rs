//! Procedural macro for "deriving" a Bevy `Plugin` from a function.

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input,
    FnArg, GenericParam, Ident, ItemFn, Pat, PatType, Signature, Type, ReturnType,
    WhereClause, WherePredicate,
};


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

    let vis = item_fn.vis;
    let ident = item_fn.sig.ident;
    let body = item_fn.block;

    let generics = item_fn.sig.generics;
    let has_generics = !generics.params.is_empty();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let type_decl = if has_generics {
        // Bind the generic parameters in a PhantomData<(...)> (phantom tuple) field.
        let tuple_types = generics.params.iter()
            .map(|param| generic_param_to_phantom_type(param, where_clause));
        let phantom_data_param = quote! { ( #(#tuple_types),* ) };

        quote! {
            struct #ident #generics
                #where_clause
            {
                _marker: ::core::marker::PhantomData<#phantom_data_param>,
            }
        }
    } else {
        quote! { struct #ident; }
    };

    let derive = if has_generics { quote! {} } else { quote! { #[derive(Default)] } };
    let extra_trait_impls = if has_generics {
        // For generic types, we need to explicitly implement `Default` so as to not mandate
        // extra `T: Default` bounds on all its type parameters.
        let default_impl = quote! {
            impl #impl_generics ::core::default::Default for #ident #ty_generics
                #where_clause
            {
                fn default() -> Self {
                    Self { _marker: ::core::marker::PhantomData }
                }
            }
        };

        quote! {
            #default_impl
        }
    } else {
        quote! {}
    };

    let app_arg_name = typed_fn_arg_ident(&item_fn.sig.inputs[0])
        .map(|ident| quote! { #ident })
        .unwrap_or_else(|| quote! { _ });

    quote! {
        #derive
        #vis #type_decl

        impl #impl_generics ::bevy::app::Plugin for #ident #ty_generics
            #where_clause
        {
            fn build(&self, #app_arg_name: &mut ::bevy::app::App) {
                #body
            }
        }

        #extra_trait_impls
    }
}


fn is_signature_valid(sig: &Signature) -> bool {
    if sig.inputs.len() != 1 {
        return false;
    }

    // Check the sole argument's type.
    let Some(at_mut_app) = fn_arg_as_mut_ref_type(&sig.inputs[0]) else { return false };
    let Type::Path(app) = at_mut_app else { return false };
    if app.path.segments.is_empty() {
        return false;
    }
    if app.path.segments.last().unwrap().ident != "App" {
        return false;
    }

    // Check the return value (or rather, lack thereof).
    match sig.output {
        ReturnType::Default => {},
        ReturnType::Type(_, _) => {
            // TODO: check if the type is `()` because that's also valid
            // (technically, that `()` may also be aliased to something else, but we can't support
            // obnoxious cases like that without type information)
            return false;
        },
    }

    true
}

#[inline]
#[allow(clippy::question_mark)]
fn fn_arg_as_mut_ref_type(fn_arg: &FnArg) -> Option<&Type> {
    let FnArg::Typed(PatType { ty, .. }) = fn_arg else { return None };
    let Type::Reference(ref ref_ty) = **ty else { return None };
    if ref_ty.mutability.is_none() {
        return None;
    }
    Some(&*ref_ty.elem)
}


fn generic_param_to_phantom_type(
    param: &GenericParam, where_clause: Option<&WhereClause>,
) -> TokenStream2 {
    match param {
        GenericParam::Lifetime(lt_param) => {
            let lifetime = &lt_param.lifetime;

            // Because Bevy plugins need to be 'static types, lifetimes params are generally
            // not supported, except for the case when they are (redundantly) bounded by 'static.
            let has_static_bound =
                lt_param.bounds.iter().any(|bound| bound.ident == "static") ||
                where_clause.map(|wc| wc.predicates.iter().any(|pred| {
                    match pred {
                        WherePredicate::Lifetime(where_lt) =>
                            where_lt.lifetime.ident == lifetime.ident &&
                            where_lt.bounds.iter().any(|bound| bound.ident == "static"),
                        _ => false,
                    }
                })).unwrap_or(false);
            if !has_static_bound {
                abort! {
                    lt_param,
                    "Generic lifetime arguments to a #[bevy_plugin] function are not valid";
                    help = "Bevy plugins need to be owned types"
                }
            }

            quote! { &#lifetime () }
        },
        GenericParam::Type(type_param) => {
            let ty = &type_param.ident;

            let has_bounds =
                !type_param.bounds.is_empty() ||
                where_clause.map(|wc| !wc.predicates.is_empty()).unwrap_or(false);
            if !has_bounds {
                abort! {
                    type_param,
                    "Invalid type parameters bounds";
                    note = "Generic type arguments to a #[bevy_plugin] function must be 'static";
                    help = format!(
                        "Add an explicit `{ty}: 'static` bound, or another bound that implies it")
                }
            }

            // We don't use the `ty` type directly to avoid misleading the compiler about
            // its ownership rules, and to not introduce extra trait bounds on it, like `Send`.
            quote! { fn() -> #ty }
        },
        GenericParam::Const(const_param) => {
            let ident = &const_param.ident;
            quote! { fn() -> &'static [(); #ident] }
        },
    }
}


#[inline]
fn typed_fn_arg_ident(fn_arg: &FnArg) -> Option<&Ident> {
    let FnArg::Typed(PatType { pat, .. }) = fn_arg else { return None };
    let Pat::Ident(ref pat_ident) = **pat else { return None };
    Some(&pat_ident.ident)
}
