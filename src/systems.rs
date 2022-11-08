use crate::prelude::*;
use bevy::{
    asset::AssetServer,
    prelude::*,
    sprite::{collide_aabb::Collision, MaterialMesh2dBundle},
};

pub fn spawn_assets(
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
        .spawn_bundle(PlayerBundle::new());

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

pub fn animate_sprite_system(
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
