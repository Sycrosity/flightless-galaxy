// #![allow(unused_imports)]

use bevy::{
    app::AppExit,
    asset::{AssetServer, LoadState},
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    log::{Level, LogPlugin, LogSettings},
    prelude::*,
    render::texture::ImageSettings,
    sprite::{collide_aabb::Collision, MaterialMesh2dBundle},
};

use bevy_asset_loader::prelude::*;

use leafwing_input_manager::prelude::*;

// use iyes_progress::prelude::*;
// use iyes_loopless::prelude::*;

use flightless_galaxy::{prelude::*, systems};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    MainMenu,
    Playing,
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
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                }),
        )
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(InputManagerPlugin::<GameInputs>::default())
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                // .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                //     "game-assets.assets",
                // ])
                .with_collection::<ImageAssets>(),
        )
        .add_startup_system(setup)
        .add_state(GameState::AssetLoading)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(systems::spawn_assets))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(systems::animate_sprite_system)
                .with_system(
                    systems::player_movement
                        // `player_movement` must always run after `input_handling`
                        .after(systems::input_handling),
                ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
