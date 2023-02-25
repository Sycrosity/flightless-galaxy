use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{inspector_options::std_options::*, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

///a [`Component`] to display if the entity can be controlled by actions.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Controllable(pub bool);

impl Controllable {
    ///creates a new [`Controllable`] struct with whether the entity is being `controlled` or not
    pub fn new(controlled: bool) -> Self {
        Self(controlled)
    }
}

///a [`Component`] to dictate the speed of an object moving around an origin, with the 'default' of 1 being π/2 radians of a circle (1/4 of a circle)
#[derive(Component, Reflect, Default, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct Speed(#[inspector(min = 0.0, max = 256.0, display = NumberDisplay::Slider)] pub f32);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Planet;

#[derive(Bundle)]
pub struct PlanetBundle {
    _player: Player,
    pos: Polar,
    #[bundle]
    sprite_bundle: SpriteBundle,
    // mesh: Mesh,
}

impl PlanetBundle {
    pub fn new(pos: Polar, texture: Handle<Image>) -> Self {
        Self {
            _player: Player,
            pos,
            // mesh: shape::Circle::new(100.).into(),
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform::from_polar_seperate(
                    Polar::new(pos.r, pos.theta, pos.z),
                    Quat::from_rotation_z(pos.rotation()),
                    Vec3::splat(0.1),
                ),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    _player: Player,
    // pos: Polar,
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    input_manager: InputManagerBundle<GameAction>,
    // sprite_sheet_bundle: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn new(pos: Polar, texture: Handle<Image>, keybinds: InputMap<GameAction>) -> Self {
        Self {
            _player: Player,
            // pos,
            sprite_bundle: SpriteBundle {
                texture,
                transform: Transform::from_polar_seperate(
                    Polar::new(pos.r, pos.theta, pos.z),
                    Quat::from_rotation_z(pos.rotation()),
                    Vec3::splat(0.1),
                ),
                ..default()
            },
            input_manager: InputManagerBundle {
                input_map: keybinds,
                ..default()
            },
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    // #[asset(texture_atlas(tile_size_x = 18., tile_size_y = 18., columns = 20, rows = 9))]
    #[asset(key = "tileset_atlas")]
    pub tileset_atlas: Handle<TextureAtlas>,
    #[asset(key = "ferris")]
    pub ferris: Handle<Image>,
    // #[asset(texture_atlas(tile_size_x = 24., tile_size_y = 24., columns = 10, rows = 3))]
    #[asset(key = "sprite_atlas")]
    pub sprite_atlas: Handle<TextureAtlas>,
}
