pub mod protagonist;
use protagonist::*;
use bevy::prelude::*;
use bevy::window::{ CursorOptions, CursorGrabMode, PrimaryWindow };

fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).add_plugins(MainPlugin).run()
}
pub struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (
            update_pos,
            update_input,
            update_rotation_with_mouse,
            lock_cursor,
        ));
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    println!("Hello World");

    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(20.0, 0.5, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 10, 0))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));

    cmds.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
    ));
    cmds.spawn((
        Camera3d::default(),
        Transform::from_xyz(-10.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        Velocity(Vec3::ZERO),
        Protagonist { paused: true },
        CameraSensitivity(Vec2::new(0.002,0.002)),
    ));
}
fn update_pos(query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut pos) in query {
        pos.translation += vel.0;
    }
}
fn lock_cursor(
    mut cursor_query: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut protag_query: Query<&mut Protagonist>,
    keys: Res<ButtonInput<KeyCode>>
) {
    if keys.just_pressed(KeyCode::Escape) {
        let mut protagonist = protag_query.single_mut().unwrap();
        let mut cursor = cursor_query.single_mut().unwrap();
        protagonist.paused = !protagonist.paused;
        cursor.visible = protagonist.paused;
        if protagonist.paused {
            cursor.grab_mode = CursorGrabMode::None;
        } else {
            cursor.grab_mode = CursorGrabMode::Locked;
        }
    }
}
