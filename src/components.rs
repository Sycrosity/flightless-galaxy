use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    _player: Player,
    pos: PolarCoords,
    #[bundle]
    sprite_bundle: SpriteBundle

    // #[bundle]
    // sprite_bundle: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn new(pos: PolarCoords) -> Self {
        Self {
            _player: Player,
            pos,
            sprite_bundle: SpriteBundle {
                texture: assets.ferris.clone(),
                transform: Transform::new_polar(
                    Polar::new(50., 270., 10.),
                    Quat::from_rotation_z(f32::to_radians(180.)),
                    Vec3::splat(0.1),
                ),
                ..default()
            }
        }
    }
}

///a set of coordinates in the [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system))
#[derive(Default, Component)]
pub struct PolarCoords {
    /// The radius (distance) from the reference pole.
    r: f32,
    /// The polar angle from the reference direction.
    theta: f32,
}

impl PolarCoords {
    /// Creates a new [`PolarCoords`].
    pub fn new(r: f32, theta: f32) -> Self {
        Self { r, theta }
    }

    // /// Creates a [`PolarCoords`] at 0,0.
    // pub fn default() -> Self {
    //     Self { r: 0., theta: 0. }
    // }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

// #[derive(AssetCollection)]
// pub struct ImageAssets {
//     #[asset(key = "tilemaps.tileset")]
//     pub tileset: Handle<TextureAtlas>,
//     #[asset(key = "sprites.ferris")]
//     pub ferris: Handle<Image>,
//     #[asset(key = "tilemaps.sprites")]
//     pub sprites: Handle<TextureAtlas>,
// }

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 18., columns = 20, rows = 9))]
    #[asset(path = "tilemaps/tileset.png")]
    pub tileset: Handle<TextureAtlas>,
    #[asset(path = "sprites/ferris.png")]
    pub ferris: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 24., tile_size_y = 24., columns = 10, rows = 3))]
    #[asset(path = "tilemaps/sprites.png")]
    pub sprites: Handle<TextureAtlas>,
}
