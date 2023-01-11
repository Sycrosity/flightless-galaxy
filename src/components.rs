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
    pub fn new(pos: Polar, texture: Handle<Image>) -> Self {
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
                input_map: PlayerBundle::default_input_map(),
                ..default()
            },
        }
    }

    pub fn default_input_map() -> InputMap<GameAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use GameAction::*;
        let mut input_map = InputMap::default();

        //movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(KeyCode::W, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(KeyCode::S, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(KeyCode::A, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(KeyCode::D, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        input_map.insert(KeyCode::Space, Jump);
        input_map.insert(GamepadButtonType::South, Jump);

        //abilities
        // input_map.insert(KeyCode::Q, Ability1);
        // input_map.insert(GamepadButtonType::West, Ability1);
        input_map.insert(MouseButton::Left, Primary);

        // input_map.insert(KeyCode::W, Ability2);
        // input_map.insert(GamepadButtonType::North, Ability2);
        input_map.insert(MouseButton::Right, Secondary);

        // input_map.insert(KeyCode::E, Ability3);
        // input_map.insert(GamepadButtonType::East, Ability3);

        // input_map.insert(KeyCode::R, Ultimate);
        // input_map.insert(GamepadButtonType::LeftTrigger2, Ultimate);

        input_map
    }
}

///a set of coordinates in the [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system).
// #[derive(Clone, Copy, Default, Component)]
// pub struct PolarPos {
//     /// The radius (distance) from the reference pole.
//     r: f32,
//     /// The polar angle from the reference direction.
//     theta: f32,
// }

// impl PolarPos {
//     /// Creates a new [`PolarPos`].
//     pub fn new(r: f32, theta: f32) -> Self {
//         Self { r, theta }
//     }

//     // /// Creates a [`PolarPos`] at 0,0.
//     pub fn default() -> Self {
//         Self { r: 0., theta: 0. }
//     }

//     pub fn rotation(self) -> f32 {
//         self.theta.to_radians()
//     }
// }

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
