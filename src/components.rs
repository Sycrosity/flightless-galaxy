use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {

    _player: Player,
    direction: Dir,


    //
    #[bundle]
    sprite: SpriteSheetBundle,

}

#[derive(Component)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}