pub mod protagonist;
pub mod antagonist;
use avian3d::prelude::*;
use bevy::prelude::*;
use protagonist::*;
use antagonist::*;

fn main() -> AppExit {
    App::new().add_plugins((DefaultPlugins, PhysicsPlugins::default(), MainPlugin)).run()
}
pub struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        // initialize
        app.add_systems(Startup, build_world);
        // player update
        app.add_systems(Update, (
            update_input,
            update_rotation_with_mouse,
            update_spawn_enemy_input,
            export_protagonist_info,
            lock_cursor,
            update_antagonist,
        ));
    }
}

fn build_world(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    cmds.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
    // ground2
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -50.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.5, 50.0),
    ));

    // ground
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -0.25, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 0.5, 50.0),
    ));

    for _ in 0..=67 {
        // enemy cube
        cmds.spawn((
            Mesh3d(meshes.add(Sphere::new(1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 0))),
            Transform::from_xyz(0.0, 3.0, 0.0),
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            Antagonist { detecting: true, goto: Vec3 { x: -10.0, y: 5.0, z: 0.0 } },
        ));
    }

    // players
    cmds.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, -10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Protagonist { paused: true, position: Vec3::ZERO },
        CameraSensitivity(Vec2::new(0.002, 0.002)),
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::capsule(0.3, 1.0),
        children![
            (
                PointLight {
                    shadows_enabled: true,
                    ..default()
                },
            ),
            (
                Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 3.0))),
                MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 255))),
                Transform::from_xyz(0.5, 0.0, 3.0),
            )
        ],
    ));
}

pub fn update_spawn_enemy_input(
    keys: Res<ButtonInput<KeyCode>>,
    cmds: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>
) {
    if keys.pressed(KeyCode::KeyE) {
        spawn_enemy(cmds, meshes, materials);
    }
}

pub fn spawn_enemy(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // enemy cube
    cmds.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 0))),
        Transform::from_xyz(0.0, 3.0, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Antagonist { detecting: true, goto: Vec3 { x: -10.0, y: 5.0, z: 0.0 } },
    ));
}
