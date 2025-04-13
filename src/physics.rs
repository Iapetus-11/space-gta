use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct VelocityMaximum(pub Vec2);

#[derive(Component, Deref, DerefMut)]
pub struct Acceleration(pub Vec2);

#[derive(Component)]
pub struct Drag {
    pub recommend: f32,
    pub actual: f32,
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

pub fn apply_drag(mut query: Query<(&mut Velocity, &Drag)>, time: Res<Time>) {
    for (mut velocity, drag) in &mut query {
        let drag = 1.0 + drag.actual * time.delta_secs();

        velocity.x /= drag;
        velocity.y /= drag;
    }
}

pub fn apply_acceleration(
    mut query: Query<(&mut Velocity, Option<&VelocityMaximum>, &Acceleration)>,
    time: Res<Time>,
) {
    for (mut velocity, velocity_max, acceleration) in &mut query {
        **velocity += **acceleration * time.delta_secs();

        if let Some(velocity_max) = velocity_max {
            **velocity = velocity.clamp(-**velocity_max, **velocity_max);
        }
    }
}
