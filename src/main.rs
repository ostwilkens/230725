use bevy::{
    app::AppExit,
    math::{vec2, vec3},
    prelude::*,
    render::camera::ScalingMode,
    sprite::MaterialMesh2dBundle,
};
use bevy_rapier2d::prelude::*;
use bevy_rapier_collider_gen::*;
use std::f32::consts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .add_systems(Update, (exit_on_esc, arrow_keys_apply_force))
        .add_systems(
            Update,
            generate_map_collider.run_if(in_state(GameState::Loading)),
        )
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Default, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct MapImageHandle {
    collider_image: Handle<Image>,
    visual_image: Handle<Image>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 512.0,
                min_height: 512.0,
            },
            ..default()
        },
        ..default()
    });

    commands.insert_resource(MapImageHandle {
        collider_image: asset_server.load("col.png"),
        visual_image: asset_server.load("map.png"),
    });
}

fn arrow_keys_apply_force(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    q_player: Query<Entity, With<Player>>,
    time: Res<Time>,
) {
    for entity in q_player.iter() {
        let mut impulse = vec2(0.0, 0.0);
        let mut torque_impulse = 0.0;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            torque_impulse += time.delta_seconds() * 0.02;
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            torque_impulse -= time.delta_seconds() * 0.02;
        }

        if keyboard_input.pressed(KeyCode::Space)
            || keyboard_input.pressed(KeyCode::W)
            || keyboard_input.pressed(KeyCode::Up)
        {
            impulse = Vec2::new(0.0, time.delta_seconds() * 10.0);
        }

        if impulse != Vec2::ZERO || torque_impulse != 0.0 {
            commands.entity(entity).insert(ExternalImpulse {
                impulse: impulse,
                torque_impulse: torque_impulse,
            });
        }
    }
}

fn generate_map_collider(
    image_assets: ResMut<Assets<Image>>,
    map_image_handle: Option<Res<MapImageHandle>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    if let Some(map_image_handle) = map_image_handle {
        if let Some(collider_image) = image_assets.get(&map_image_handle.collider_image.clone()) {
            let colliders = multi_polyline_collider_translated(collider_image);

            // spawn map colliders
            for collider in colliders {
                commands.spawn((
                    collider,
                    RigidBody::Fixed,
                    SpriteBundle {
                        texture: map_image_handle.visual_image.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                ));
            }

            // spawn player
            commands
                .spawn(Player)
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(15.0))
                .insert(Restitution::coefficient(0.7))
                .insert(Friction::new(5.0))
                .insert(SpriteBundle {
                    texture: asset_server.load("player.png"),
                    transform: Transform::from_xyz(0.0, 20.0, 1.0),
                    sprite: Sprite {
                        custom_size: Some(vec2(30.0, 30.0)),
                        ..default()
                    },
                    ..default()
                });

            next_state.set(GameState::Playing);
        }
    }
}

fn exit_on_esc(keyboard_input: ResMut<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
