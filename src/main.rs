#![allow(unused_imports)]

use bevy::{
    app::AppExit,
    asset::{AssetServer, LoadState},
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
    sprite::collide_aabb::Collision,
};

use bevy_asset_loader::prelude::*;

mod components;
mod inputs;

use components::*;

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
        .insert_resource(WindowDescriptor {
            title: "Flightless Galaxy".to_string(),
            // cursor_locked: true,
            // cursor_visible: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //prevents blurry sprites
        .insert_resource(ImageSettings::default_nearest())
        .add_loading_state(
            //while the game loads, use the state AssetLoading - once finished, go to Playing
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    "game-assets.assets",
                ])
                .with_collection::<ImageAssets>(),
        )
        .add_state(GameState::AssetLoading)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(animate_sprite_system))
        .run();
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(key = "tilemaps.tileset")]
    tileset: Handle<TextureAtlas>,
    #[asset(key = "sprites.ferris")]
    ferris: Handle<Image>,
    #[asset(key = "tilemaps.sprites")]
    sprites: Handle<TextureAtlas>,
}

fn setup(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    //any changes to the mesh will be reloaded automatically! Try making a change to Monkey.gltf.
    //you should see the changes immediately show up in your app.

    //camera
    commands.spawn_bundle(Camera2dBundle::default());

    //spawn player
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.ferris.clone(),
            transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        })
        .insert(Player);

    //spawn a
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: assets.sprites.clone(),
            transform: Transform::from_xyz(0., -150., 0.),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1., true)));
}

#[derive(Component)]
struct AnimationTimer(Timer);

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8;
        }
    }
}
