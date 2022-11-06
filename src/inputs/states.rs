use bevy::prelude::*;

pub struct KeyState {
    ///whether the right "key" is pressed
    pub right: bool,
    ///whether the left "key" is pressed
    pub left: bool,
    ///whether the up "key" is pressed
    pub up: bool,
    ///whether the down "key" is pressed
    pub down: bool,
    ///whether the ability "key" is pressed
    pub ability: bool,
    ///a 2x1 matrix for gamepad inputs
    pub analog_matrix: Vec2,
}

impl Default for KeyState {
    fn default() -> Self {
        Self {
            right: false,
            left: false,
            up: false,
            down: false,
            ability: false,

            analog_matrix: Vec2::ZERO,
        }
    }
}

impl KeyState {
    pub fn dir_vec(&self) -> Vec2 {
        let dir: Vec2 = if self.analog_matrix == Vec2::ZERO {
            Vec2::new(
                if self.right { 1.0 } else { 0.0 } + if self.left { -1.0 } else { 0.0 },
                if self.up { 1.0 } else { 0.0 } + if self.down { -1.0 } else { 0.0 },
            )
        } else {
            self.analog_matrix
        };

        if dir.length_squared() <= 1.0 {
            dir
        } else {
            dir.normalize()
        }
    }
}
