//! Tests for the #[bevy_plugin] attribute proc macro.

use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;


#[test]
fn empty() {
    #[bevy_plugin]
    fn EmptyPlugin(_: &mut App) {}

    let mut app = App::new();
    app.add_plugin(EmptyPlugin);
    assert!(app.is_plugin_added::<EmptyPlugin>());
}

#[test]
fn simple_resource() {
    #[bevy_plugin]
    fn SimpleResourcePlugin(app: &mut App) {
        app.insert_resource(SimpleResource(42));
    }

    #[derive(Resource)]
    struct SimpleResource(usize);

    let mut app = App::new();
    app.add_plugin(SimpleResourcePlugin);
    assert!(app.is_plugin_added::<SimpleResourcePlugin>());

    let simple_resource: &SimpleResource = app.world.get_resource().unwrap();
    assert_eq!(simple_resource.0, 42);
}
