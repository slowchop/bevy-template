# Slowchop Studio's Bevy Template

This is a template for a [Bevy](https://bevyengine.org/) game.

Features:

- Bevy 0.12.1
- Wasm build using [wasm-pack](https://rustwasm.github.io/wasm-pack/) that can be deployed to [itch.io](https://itch.io/) and [netlify](https://www.netlify.com/).
- Contains a `justfile` ([just](https://github.com/casey/just)) for building and deploying.
- Everything is a Plugin!
- A basic GameState used throughout the template.
- A basic 2D character that moves around.
- Some AI generated placeholders that you should not use.
- Most things have a `Name` component for easier debugging.
 
Plugins:

- Asset loading using [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader)
  - A splash/loading screen which loads assets first.
- UI DSL using [bevy_ui_dsl](https://github.com/Anti-Alias/bevy_ui_dsl/).
  - A very basic main menu.
  - A very basic in game UI (TODO)
- A Quake style console using [slowchop_console](https://github.com/slowchop/console)
  - `slowchop_console` is brand new, don't use this for anything too important.
- Keyboard actions using [leafwing-input-manager](https://github.com/Leafwing-Studios/leafwing-input-manager/)
- Inspect your entities [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui)

