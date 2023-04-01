# bevy_fn_plugin

[![Build Status](https://github.com/Xion/bevy_fn_plugin/workflows/ci/badge.svg?branch=master)][actions]
[![Latest Version](https://img.shields.io/crates/v/bevy_fn_plugin.svg)][crates.io]
[![Documentation(https://docs.rs/bevy_fn_plugin/badge.svg)]][docs.rs]

[actions]: https://github.com/Xion/bevy_fn_plugin/actions
[crates.io]: https://crates.io/crates/bevy_fn_plugin
[docs.rs]: https://docs.rs/bevy_fn_plugin/latest/bevy_fn_plugin/

Create Bevy plugins from simple functions

## Introduction

`bevy_fn_plugin` allows you to easily create [Bevy](https://bevyengine.org) plugins
from simple Rust functions.

It exposes a single attribute, `#[bevy_plugin]`, that can be applied to `fn` items
to turn them into Bevy `Plugin` types. The function should take a single `&mut App` argument,
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

However, Bevy plugin API has been very stable for several releases now. It should be possible to
use the crate with Bevy 0.9 and 0.8 as well, and possibly even earlier versions.

`bevy_fn_plugin` doesn't depend on `bevy` itself.

## Limitations

### Unique plugins

Since Bevy 0.9, plugins can potentially be installed multiple times if the `Plugin::is_unique`
method returns `false` (which isn't the default).

`bevy_fn_plugin` doesn't override this method, which means that the plugins it generates are always
unique. This shouldn't be a problem in practice, because the plugin instances have no state
and should thus be interchangeable.

### Generic functions

Generic functions are currently not supported. This isn't a fundamental limitation: support for
generic functions will be added in a future release.

## FAQ

### Why the `CamelCase` functions?

`bevy_fn_plugin` converts the function into a type that implements the  `Plugin` trait.
Because type names in Rust are conventionally CamelCased, the name of the source function
should thus follow the language conventions for types.

Note that the source function isn't actually present in the final code, so its name cannot trigger
any compiler warnings.

### How does it compare to thr `seldom_fn_plugin` crate?

The `seldom_fn_plugin` crate doesn't actually create Bevy `Plugin` types. Instead, it applies
the annotated functions directly to the Bevy `App` object. To work, it requires you to import
a custom extension trait, which precludes the usage of `App::add_plugin`, `App::is_plugin_added`,
and related plugin API.

In contrast, `bevy_fn_plugin` does create full-fledged Bevy `Plugin`s.
If you are writing a library, the resulting plugin can be exposed in your public API,
without requiring your users to depend on `bevy_fn_plugin` themselves.

## License

`bevy_fn_plugin` is licensed under MIT.
