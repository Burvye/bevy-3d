use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};
use std::f32::consts::FRAC_PI_2;

#[derive(Component)]
pub struct Protagonist {
    pub paused: bool
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct CameraSensitivity(pub Vec2);


pub fn update_input(query: Query<(&mut Velocity, &Transform), With<Protagonist>>, keys: Res<ButtonInput<KeyCode>>) {
    let speed = 0.1;
    for (mut vel, transform) in query {
        let mut mov = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            mov += transform.forward() * speed;
        }
        if keys.pressed(KeyCode::KeyS) {
            mov -= transform.forward() * speed;
        }
        if keys.pressed(KeyCode::KeyD) {
            mov += transform.right() * speed;
        }
        if keys.pressed(KeyCode::KeyA) {
            mov -= transform.right() * speed;
        }
        vel.0 = mov;
        // need grounded collisions first for jumping
    }
}

pub fn update_rotation_with_mouse(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    protagonist: Single<(&mut Transform, &CameraSensitivity), With<Protagonist>>,
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