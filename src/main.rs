use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

mod camera;
mod input;
mod movement;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            RapierPhysicsPlugin::<NoUserData>::default(),
            WorldInspectorPlugin::default(),
            InputManagerPlugin::<input::PlayerAction>::default(),
        ))
        .add_plugins((camera::CameraPlugin, movement::MovementPlugin))
        .register_type::<PlayerConfig>()
        .insert_resource(PlayerConfig::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct PlayerConfig {
    pub mouse_sensitivity: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            mouse_sensitivity: 0.5,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera3dBundle::default(), camera::PrimaryCamera::default()));

    commands.spawn((
        TransformBundle::default(),
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::capsule_y(0.5, 1.0),
        Player,
        Damping {
            angular_damping: 0.0,
            linear_damping: 2.0,
        },
        movement::MoveDirection::default(),
        movement::Speed::new(100.0),
        LockedAxes::ROTATION_LOCKED,
        input::InputListenerBundle::input_map(),
    ));

    let transform = Transform::from_xyz(0.0, -2.0, 0.0);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(200.0, 1.0, 200.0).into()),
            material: materials.add(asset_server.load("check_texture.png").into()),
            transform,
            ..default()
        },
        RigidBody::Fixed,
        Collider::cuboid(100.0, 0.5, 100.0),
    ));
}
