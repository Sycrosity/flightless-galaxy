#![allow(unused_imports)]

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Playing,
    Menu,

}

fn main() {
    #[cfg(target_os = "macos")]
    unsafe {
        use cocoa::appkit::NSApplication as _;
        cocoa::appkit::NSApp().setActivationPolicy_(
            cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        );
    }

    App::new()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                .with_collection::<MyAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(use_my_assets))
        .run();
}

#[derive(AssetCollection)]
struct MyAssets {
    #[asset(path = "tilemaps/tileset.png")]
    tileset: Handle<TextureAtlas>,

}

fn use_my_assets(_my_assets: Res<MyAssets>) {
    // do something using the asset handles from the resource
}
