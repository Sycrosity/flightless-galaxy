// #![allow(unused_imports)]

use bevy::{
    app::AppExit,
    asset::{AssetServer, LoadState},
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::texture::ImageSettings,
    sprite::{collide_aabb::Collision, MaterialMesh2dBundle},
};

use bevy_asset_loader::prelude::*;

// use iyes_progress::prelude::*;
// use iyes_loopless::prelude::*;

mod components;
// mod inputs;
mod helpers;

use components::*;

use helpers::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ImageSettings::default_nearest()) //prevents blurry sprite
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Playing)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec!["game-assets.assets"])
                .with_collection::<ImageAssets>(),
        )
        .add_startup_system(setup)
        .add_state(GameState::AssetLoading)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_assets))
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

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_assets(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(100.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });

    //spawn player
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.ferris.clone(),
            transform: Transform::new_polar(
                Polar::new(50., 270., 10.),
                Quat::from_rotation_z(f32::to_radians(180.)),
                Vec3::splat(0.1),
            ),
            ..default()
        })
        .insert(Player);

    //spawn a
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: assets.sprites.clone(),
            transform: Transform::from_rtz(150., 270., 0.),
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
