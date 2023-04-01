#[macro_use] extern crate bevy_fn_plugin;

use bevy::prelude::*;

#[bevy_plugin]
fn generic_plugin<T>(_: &mut App) {}

fn main() {}
