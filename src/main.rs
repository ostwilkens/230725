use bevy::{
    app::AppExit, math::vec3, prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle,
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
        .add_plugins(RapierDebugRenderPlugin::default())
        // .add_systems(
        //     Update,
        //     (
        //     ),
        // )
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .add_systems(Update, (exit_on_esc,))
        .add_systems(
            Update,
            generate_collider.run_if(in_state(GameState::Loading)),
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(1.0),
            ..default()
        },
        ..default()
    });

    // commands
    //     .spawn(Collider::cuboid(500.0, 50.0))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    // commands.spawn((
    //     Player,
    // ))
    // .insert(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(15.).into()).into(),
    //     material: materials.add(Color::rgb(0.2, 0.4, 0.7).into()),
    //     transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)),
    //     ..Default::default()
    // });

    commands.insert_resource(MapImageHandle { 
        collider_image: asset_server.load("col.png"),
        visual_image: asset_server.load("map.png"),
    });
}

fn generate_collider(
    mut image_assets: ResMut<Assets<Image>>,
    map_image_handle: Option<Res<MapImageHandle>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(map_image_handle) = map_image_handle {
        if let Some(collider_image) = image_assets.get(&map_image_handle.collider_image.clone()) {
            let colliders = multi_convex_polyline_collider_translated(collider_image);
            for collider in colliders {
                commands.spawn((
                    collider.unwrap(),
                    RigidBody::Fixed,
                    SpriteBundle {
                        texture: map_image_handle.visual_image.clone(),
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    },
                ));
            }
    
            next_state.set(GameState::Playing);
        }
    }
}

fn exit_on_esc(keyboard_input: ResMut<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
