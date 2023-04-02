// We actually want to fail on the lifetime warning, but that one is a force-warn
// category which cannot be affected by allow/deny directives.
// This is fine, however, because the warning will still be emitted regardless so trybuild
// can verify its presence.
#![deny(unused_imports)]

#[macro_use] extern crate bevy_fn_plugin;

use bevy::prelude::*;

#[bevy_plugin]
fn GenericPlugin<'a: 'static>(_: &mut App) {
    let _: Option<&'a ()> = None;
}

fn main() {}
