use bevy::prelude::*;

#[derive(Component)]
pub struct Protagonist {
    pub paused: bool
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

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
        vel.x = mov.x;
        vel.y = mov.y;
        vel.z = mov.z;
        // need grounded collisions first for jumping
    }
}