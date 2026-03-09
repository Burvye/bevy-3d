pub mod protagonist;
use protagonist::*;
use bevy::prelude::*;
use bevy::window::{CursorOptions, CursorGrabMode, PrimaryWindow};

fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).add_plugins(MainPlugin).run()
}
pub struct MainPlugin;
impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_pos, update_input, lock_cursor));
    }
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    println!("Hello World");

    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 10, 0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
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
        Velocity { x: 0.0, y: 0.0, z: 0.0 },
        Protagonist { paused: false },
    ));
}
fn update_pos(query: Query<(&Velocity, &mut Transform)>) {
    for (vel, mut pos) in query {
        pos.translation.x += vel.x;
        pos.translation.y += vel.y;
        pos.translation.z += vel.z;
    }
}
fn lock_cursor(
    mut window_query: Query<(&mut CursorOptions, &mut Protagonist), With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    if let Ok((mut cursor, mut protagonist)) = window_query.single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            protagonist.paused = !protagonist.paused;
            cursor.visible = protagonist.paused;
            if protagonist.paused {
                cursor.grab_mode = CursorGrabMode::Locked;
            } else {
                cursor.grab_mode = CursorGrabMode::None;
            }
        }
    }
}
