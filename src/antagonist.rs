use avian3d::prelude::*;
use bevy::prelude::*;

use crate::protagonist::Protagonist;

#[derive(Component, Debug)]
pub struct Antagonist {
    pub detecting: bool,
    pub goto: Vec3,
}

pub fn update_antagonist(
    mut antagonists: Query<
        (&mut LinearVelocity, &mut Transform, &mut Antagonist),
        With<Antagonist>
    >,
    protagonists: Query<&Protagonist, With<Protagonist>>
) {
    let Ok(protagonist) = protagonists.single() else {
        return;
    };
    for (mut velocity, mut transform, mut antagonist) in &mut antagonists {
        antagonist.goto = protagonist.position;
        transform.look_at(antagonist.goto, Vec3::Y);

        let lookdir = *transform.forward();
        *velocity = LinearVelocity { 0: lookdir * 6.7 };
    }
}