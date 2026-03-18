use avian3d::prelude::*;
use bevy::{ input::mouse::AccumulatedMouseMotion, prelude::* };
use bevy::window::{ CursorGrabMode, CursorOptions, PrimaryWindow };
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct Protagonist {
    pub paused: bool,
    pub position: Vec3,
}

#[derive(Component)]
pub struct CameraSensitivity(pub Vec2);

pub fn export_protagonist_info(query: Query<(&Transform, &mut Protagonist), With<Protagonist>>) {
    for (mov, mut protagonist) in query {
        protagonist.position = mov.translation;
    }
}

pub fn update_input(
    mut query: Query<(&mut LinearVelocity, &Transform), With<Protagonist>>,
    keys: Res<ButtonInput<KeyCode>>
) {
    let speed = 10.0;
    for (mut vel, transform) in query.iter_mut() {
        let mut mov = Vec3::ZERO;

        // use instead of forward without vertical movement
        let mut front: Vec3 = *transform.forward();
        front.y = 0.0;
        front = front.normalize_or_zero();

        if keys.pressed(KeyCode::KeyW) {
            mov += front;
        }
        if keys.pressed(KeyCode::KeyS) {
            mov -= front;
        }
        if keys.pressed(KeyCode::KeyD) {
            mov += *transform.right();
        }
        if keys.pressed(KeyCode::KeyA) {
            mov -= *transform.right();
        }
        if keys.just_pressed(KeyCode::Space) {
            mov += *transform.up();
        }
        vel.y += mov.y * 15.0;
        vel.x = mov.x * speed;
        vel.z = mov.z * speed;
    }
}

pub fn update_rotation_with_mouse(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    protagonist: Single<(&mut Transform, &CameraSensitivity), With<Protagonist>>
) {
    let (mut transform, camera_sensitivity) = protagonist.into_inner();

    const MAX_PITCH: f32 = FRAC_PI_2 - 0.01;

    let d = accumulated_mouse_motion.delta;
    let dyaw = -d.x * camera_sensitivity.0.x;
    let dpitch = -d.y * camera_sensitivity.0.y;

    let (mut yaw, mut pitch, ..) = transform.rotation.to_euler(EulerRot::YXZ);
    yaw += dyaw;
    pitch += dpitch;
    pitch = pitch.clamp(-MAX_PITCH, MAX_PITCH);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

pub fn lock_cursor(
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