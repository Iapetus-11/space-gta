use bevy::{input::keyboard, prelude::*};

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Vehicle {}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Vehicle {},
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::linear_rgb(1.0, 0.0, 0.9))),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn player_input(
    kb: Res<ButtonInput<KeyCode>>,
    mut vehicle_velocity: Single<&mut Velocity, With<Vehicle>>,
) {
    const MAX_VEL: f32 = 200.0;
    let sensitivity: f32 = 2.0;

    if kb.pressed(KeyCode::ArrowUp) {
        vehicle_velocity.y += sensitivity;
    }

    if kb.pressed(KeyCode::ArrowRight) {
        vehicle_velocity.x += sensitivity;
    }

    if kb.pressed(KeyCode::ArrowDown) {
        vehicle_velocity.y -= sensitivity;
    }

    if kb.pressed(KeyCode::ArrowLeft) {
        vehicle_velocity.x -= sensitivity;
    }

    let clamped =
        vehicle_velocity.clamp(Vec2::new(-MAX_VEL, -MAX_VEL), Vec2::new(MAX_VEL, MAX_VEL));
    vehicle_velocity.x = clamped.x;
    vehicle_velocity.y = clamped.y;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup,))
        .add_systems(Update, (apply_velocity, player_input))
        .run();
}
