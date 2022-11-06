use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    _player: Player,
    pos: PolarCoords,

    //
    #[bundle]
    sprite: SpriteSheetBundle,
}

///a set of coordinates in the [polar coordinate system](https://en.wikipedia.org/wiki/Polar_coordinate_system))
#[derive(Component)]
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
}
