//stop windows from opening a console window on app start
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::prelude::*;

use flightless_galaxy::{prelude::*, systems};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) //prevents blurry sprites
                // .set(LogPlugin {
                //     level: bevy::log::Level::INFO,
                //     ..default()
                // })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }) //hotreloading of files
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        // fill the entire browser window
                        fit_canvas_to_parent: true,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EntityCountDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin)
        // .register_type::<ActionState<GameAction>>()
        // .register_type::<InputMap<GameAction>>()
        .register_type::<AnimationTimer>()
        .register_type::<Controllable>()
        .register_type::<Speed>()
        // .register_type::<Planet>()
        .add_plugin(RonAssetPlugin::<StandardDynamicAssetCollection>::new(&[
            "assets.ron",
        ]))
        // .add_plugin(RonAssetPlugin::)
        //This plugin maps inputs to an input-type agnostic action-state
        .add_plugin(InputManagerPlugin::<GameAction>::default())
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                // .set_standard_dynamic_asset_collection_file_endings(vec![".ron"])
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec!["game.assets.ron"])
                .with_collection::<ImageAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(systems::spawn_game_assets),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(systems::animate_sprite_system)
                // .with_system(keymouse_inputs)
                .with_system(
                    systems::player_movement, //`player_movement` must always run after `input_handling`
                ), //.after(systems::input_handling())
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            /*
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(16.),
                ..default()
            },*/
            ..default()
        },
        Name::new("2DCamera [1]"),
    ));
}
