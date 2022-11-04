use bevy::prelude::*;
use crate::components::*;


pub fn keyboard_inputs(
    input: Res<Input<KeyCode>>,
    mut query: Query<Player>) {
    
    match input {

        KeyCode::A => query.player.x = 10

    }
}

