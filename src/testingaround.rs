use bevy::prelude::*;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

fn main() {
    #[cfg(target_os = "macos")]
    unsafe {
        use cocoa::appkit::NSApplication as _;
        cocoa::appkit::NSApp().setActivationPolicy_(
            cocoa::appkit::NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        );
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(text_update_system)
        // .add_system(track_mouse)
        .run();
}
#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default()); //a non moving static camera
    commands
        .spawn_bundle(SpriteBundle {
            //importing and loading the sprite for ferris
            texture: asset_server.load("sprites/ferris.png"),
            transform: Transform {
                scale: Vec3::new(10., 10., 10.),
                ..default()
            },
            ..default()
        })
        .insert(Player);

    //fps counter text
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/SpaceMono-Regular.ttf"),
                    font_size: 32.0,
                    color: Color::WHITE,
                }),
                TextSection::new(
                    " FPS",
                    TextStyle {
                        font: asset_server.load("fonts/SpaceMono-Regular.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                ),
            ])
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(FpsText);

    commands.spawn_bundle(
        TextBundle::from_section(
            "no bitches???",
            TextStyle {
                font: asset_server.load("fonts/SpaceMono-Regular.ttf"),
                font_size: 16.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[0].value = format!("{average:.2}");
            }
        }
    }
}

// fn track_mouse(mut motion_evr: EventReader<MouseMotion>) {
//     for ev in motion_evr.iter() {
//         println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
//     }
// }
