#[macro_use] extern crate bevy_fn_plugin;

use bevy::prelude::*;

#[bevy_plugin]
fn has_return_type(_: &mut App) -> usize { 0 }

fn main() {}
