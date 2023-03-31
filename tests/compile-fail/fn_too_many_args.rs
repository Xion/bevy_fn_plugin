#[macro_use] extern crate bevy_fn_plugin;

use bevy::prelude::*;

#[bevy_plugin]
fn multiple_args(_: &mut App, _: usize) {}

fn main() {}
