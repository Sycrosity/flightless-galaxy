#![allow(unused)]

use crate::prelude::*;
use bevy::{
    asset::AssetServer,
    prelude::*,
    sprite::{collide_aabb::Collision, MaterialMesh2dBundle},
    transform,
};
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

pub fn spawn_game_assets(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    asset_server: Res<AssetServer>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    images: Res<Assets<Image>>,
    settings: Res<GameSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut planet = commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(100.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            // transform: Transform::from_xyz(100., 10., 0.),
            ..default()
        },
        Planet,
        Name::new("Planet"),
    ));

    planet.add_children(|parent| {
        //spawn player
        let player = parent.spawn((
            PlayerBundle::new(
                Polar::new(100., FRAC_PI_2, 10.),
                assets.ferris.clone(),
                settings.keybinds.clone(),
            ),
            Controllable(true),
            Speed(2.),
            Name::new("Player [1]"),
        ));

        //spawn a sprite

        for i in 1..16 {
            parent.spawn((
                SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(0),
                    texture_atlas: assets.sprite_atlas.clone(),
                    transform: Transform::from_rtz(24. * (i as f32), FRAC_PI_6, 10.),
                    ..default()
                },
                AnimationTimer(Timer::from_seconds(1., TimerMode::Repeating)),
                InputManagerBundle {
                    input_map: settings.keybinds.clone(),
                    ..default()
                },
                Controllable(false),
                Name::new(format!("Enemy [{i}]")),
            ));
        }
    });
}

pub fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        Option<&mut Controllable>,
    )>,
) {
    for (mut timer, mut sprite, mut controllable) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 8;

            //useful!
            if let Some(ref mut cntr) = controllable {
                cntr.0 = !cntr.0;
            }
        }
    }
}

// fn player_jump(query: Query<&ActionState<Action>, With<Player>>) {
//     let action_state = query.single();
//     // Each action has a button-like state of its own that you can check
//     if action_state.just_pressed(Action::Jump) {
//         println!("I'm jumping!");
//     }
// }

pub fn player_movement(
    time: Res<Time>,
    mut query: Query<(
        &ActionState<GameAction>,
        &mut Transform,
        Option<&Speed>,
        &Controllable,
    )>,
) {
    for (action_state, mut transform, speed, controllable) in query.iter_mut() {
        let speed = if let Some(speed) = speed { speed.0 } else { 1. };

        let normalise = 100. / Polar::from_vec3(transform.translation).r;

        let speed_normalised = speed * normalise;

        let turn = Polar::new(
            0.,
            //FRAC_PI_2 is a quarter of a circle, in one second
            FRAC_PI_2 * speed_normalised * time.delta_seconds(),
            0.,
        );

        if controllable.0 {
            //each action has a button-like state of its own that you can check
            if action_state.pressed(GameAction::Left) {
                transform.translation += turn;
                transform.rotate_z(turn.theta);
            }

            if action_state.pressed(GameAction::Right) {
                transform.translation -= turn;
                transform.rotate_z(-turn.theta);
            }

            let up: Polar = Polar::new(1. * speed, 0., 0.);

            if action_state.pressed(GameAction::Up) {
                transform.translation += up;
            }

            if action_state.pressed(GameAction::Down) {
                transform.translation -= up;
            }

            if action_state.just_pressed(GameAction::Jump) {
                transform.translation += up;
            }
        }
    }
}

pub fn load_settings() {}
