use crate::components::*;
use bevy::prelude::*;

pub fn keyboard_inputs(input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
}
