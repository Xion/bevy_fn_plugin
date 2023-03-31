# bevy_fn_plugin

Create Bevy plugins from simple functions

## Introduction

`bevy_fn_plugin` allows to easily create [Bevy](https://bevyengine.org) plugins
from simple Rust functions.

It exposes a single attribute `#[bevy_plugin]` that can applied to `fn` items in order to
turn them into Bevy `Plugin` types. The function should take the `&mut App` type,
and may perform any initialization logic that a regular Bevy plugin would.

## Usage

Simply add `#[bevy_plugin]` to a function that implements your plugin logic:

```rust
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;

#[bevy_plugin]
fn GameTitlePlugin(app: &mut App) {
    app.insert_resource(GameTitle("My Awesome Game".into()));
}

#[derive(Resource)]
struct GameTitle(String);
```

You can then add the resulting `Plugin` to a Bevy `App` as usual:

```rust
fn main() {
    App::new()
        .add_plugin(GameTitlePlugin)
        .run();
}
```

## Compatibility

`bevy_fn_plugin` has been tested with Bevy 0.10.

Bevy plugin API has been very stable, though, so it should be possible to use the crate
with Bevy 0.9 or 0.8 at the very least.

`bevy_fn_plugin` doesn't depend on `bevy` itself.

## FAQ

### Why the `CamelCase` functions?

`bevy_fn_plugin` converts the function into a type that implements the  `Plugin` trait.
Because type names in Rust are conventionally camel-cased, the function name should be camel-cased
to follow the language conventions.

Note that the source function isn't actually present in the final code, so its name cannot trigger
any compiler warnings.

### How does it compare to `seldom_fn_plugin` crate?

The `seldom_fn_plugin` crate doesn't actually create Bevy `Plugin` types. Instead, it applies
the annotated functions directly to Bevy `App` object. This requires importing a custom extension
trait and precludes the usage of `App::add_plugin`, `App::is_plugin_added`, and related plugin API.

In contrast, `bevy_fn_plugin` does crete full-fledged Bevy `Plugin`s. If you are writing a library,
the resulting plugin can be exposed in your public API, without requiring your users to depend
on `bevy_fn_plugin` themselves.

## License

`bevy_fn_plugin` is licensed under MIT.
