use bevy::{
    app::{AppExit},
    math::vec3,
    prelude::*,
    sprite::MaterialMesh2dBundle, render::camera::{ScalingMode},
};
use std::f32::consts::*;

fn main() {
    let mut app = App::new();
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        // .add_systems(
        //     Update,
        //     (
        //     ),
        // )
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .add_systems(Update, (exit_on_esc,))
        .run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(1.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Player,
    ))
    .insert(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(15.).into()).into(),
        material: materials.add(Color::rgb(0.2, 0.4, 0.7).into()),
        transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

fn exit_on_esc(keyboard_input: ResMut<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
