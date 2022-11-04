use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {

    pub _player: Player,


    //
    #[bundle]
    sprite: SpriteSheetBundle,

}
