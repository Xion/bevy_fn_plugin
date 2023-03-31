#![deny(non_camel_case_types)]

#[macro_use] extern crate bevy_fn_plugin;

use bevy::prelude::*;

#[bevy_plugin]
fn non_camel_case(_: &mut App) {}

fn main() {}
