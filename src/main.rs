#![allow(unused_imports)]

use bevy::{
    app::AppExit,
    asset::LoadState,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
    sprite::collide_aabb::Collision,
};

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
        //prevents blurry sprites
        .insert_resource(ImageSettings::default_nearest())
        .add_loading_state(
            //while the game loads, use the state AssetLoading - once finished, go to Playing
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    "game-assets.assets",
                ])
                .with_collection::<MyAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_plugins(DefaultPlugins)
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .run();
}

#[derive(AssetCollection)]
struct MyAssets {
    #[asset(key = "tilemaps.tileset")]
    tileset: Handle<TextureAtlas>,
    #[asset(key = "sprites.ferris")]
    ferris: Handle<Image>,
    #[asset(key = "tilemaps.sprites")]
    sprites: Handle<TextureAtlas>,
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    assets: Res<MyAssets>,
    asset_server: Res<AssetServer>,
    
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    //any changes to the mesh will be reloaded automatically! Try making a change to Monkey.gltf.
    //you should see the changes immediately show up in your app.

    //camera
    commands.spawn_bundle(Camera2dBundle::default());

    //add player
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.ferris.clone(),
            ..default()
        })
        .insert(Player);

    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.ferris.clone(),
            ..default()
        })
        .insert(Player);
}
