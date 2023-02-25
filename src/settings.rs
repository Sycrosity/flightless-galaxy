use crate::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::*;
use leafwing_input_manager::prelude::*;

use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::Path};

///Various game settings which can be imported from a .settings.ron file
#[derive(Serialize, Deserialize, Reflect, Resource, InspectorOptions, Debug)]
#[reflect(Resource)]
pub struct GameSettings {
    #[reflect(ignore)]
    pub keybinds: InputMap<GameAction>,
}

impl GameSettings {
    pub fn new<P: AsRef<Path> + Debug + Copy>(path: P) -> Self {
        let message = path.clone();
        parse_ron(
            path,
            format!(
                "reverting to default settings map - failed to parse {:?}",
                message
            )
            .as_str(),
        )
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            keybinds: default_input_map(),
        }
    }
}

pub mod prelude {

    pub use super::*;
}

//for when some types of input aren't used
#[allow(unused)]
//the default, no-file found input map setup
pub fn default_input_map() -> InputMap<GameAction> {
    //this allows us to replace `GameAction::Up` with `Up`, significantly reducing boilerplate
    use GameAction::*;
    type Key = KeyCode;
    type Mouse = MouseButton;
    type Gamepad = GamepadButtonType;

    let mut input_map = InputMap::default();

    input_map
            .insert(Key::W, Up)
            .insert(Key::Up, Up)
            .insert(Gamepad::DPadUp, Up)

            .insert(Key::S, Down)
            .insert(Key::Down, Down)
            .insert(Gamepad::DPadDown, Down)

            .insert(Key::A, Left)
            .insert(Key::Left, Left)
            .insert(Gamepad::DPadLeft, Left)

            .insert(Key::D, Right)
            .insert(Key::Right, Right)
            .insert(Gamepad::DPadRight, Right)

            .insert(Key::Space, Jump)
            .insert(Gamepad::North, Jump)

            //abilities
            .insert(Mouse::Left, Primary)
            .insert(Gamepad::East, Primary)

            .insert(Mouse::Right, Secondary)
            .insert(Gamepad::South, Secondary)
            // .insert(Key::E, Ability3)
            // .insert(Gamepad::East, Ability3)
            // .insert(Key::R, Ultimate)
            // .insert(Gamepad::LeftTrigger2, Ultimate)
            ;

    input_map
}
