use bevy::{color::palettes::tailwind, prelude::*};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct PlayerMarker;

#[derive(Component)]
struct MaximumVelocity {
    linear: Vec2,
    angular: f32,
}

#[derive(Bundle)]
struct PlayerVehicleBundle {
    marker: PlayerMarker,

    transform: Transform,
    rigid_body: RigidBody,
    velocity: Velocity,
    max_velocity: MaximumVelocity,
    acceleration: ExternalForce,
    gravity: GravityScale,
    mass: AdditionalMassProperties,
    continuous_collision_detection: Ccd,
    restitution: Restitution,
    damping: Damping,
    locked_axes: LockedAxes,
}

impl PlayerVehicleBundle {
    fn new() -> Self {
        Self {
            marker: PlayerMarker,

            transform: default(),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            max_velocity: MaximumVelocity {
                linear: Vec2::new(1000.0, 1000.0),
                angular: 0.0,
            },
            acceleration: default(),
            gravity: GravityScale(0.0),
            mass: AdditionalMassProperties::Mass(100.0),
            continuous_collision_detection: Ccd::enabled(),
            restitution: Restitution {
                coefficient: 0.4,
                combine_rule: CoefficientCombineRule::Average,
            },
            damping: Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

#[derive(Component)]
struct ChaserMarker;

#[derive(Bundle)]
struct ChaserVehicleBundle {
    marker: ChaserMarker,

    transform: Transform,
    rigid_body: RigidBody,
    velocity: Velocity,
    max_velocity: MaximumVelocity,
    acceleration: ExternalForce,
    gravity: GravityScale,
    mass: AdditionalMassProperties,
    continuous_collision_detection: Ccd,
    restitution: Restitution,
    locked_axes: LockedAxes,
}

impl ChaserVehicleBundle {
    fn new() -> Self {
        Self {
            marker: ChaserMarker,

            transform: default(),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            max_velocity: MaximumVelocity {
                linear: Vec2::new(900.0, 900.0),
                angular: 0.0,
            },
            acceleration: default(),
            gravity: GravityScale(0.0),
            mass: AdditionalMassProperties::Mass(100.0),
            continuous_collision_detection: Ccd::enabled(),
            restitution: Restitution {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Average,
            },
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }

    fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        PlayerVehicleBundle::new(),
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 2.0,
            ..OrthographicProjection::default_2d()
        }),
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::TEAL_500))),
        Collider::ball(25.0),
    ));

    commands.spawn((
        ChaserVehicleBundle::new()
            .with_transform(Transform::from_translation(Vec3::new(-10.0, 40.0, 0.0))),
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::RED_500))),
        Collider::ball(25.0),
    ));
    commands.spawn((
        ChaserVehicleBundle::new()
            .with_transform(Transform::from_translation(Vec3::new(-100.0, 70.0, 0.0))),
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::RED_600))),
        Collider::ball(25.0),
    ));
    commands.spawn((
        ChaserVehicleBundle::new()
            .with_transform(Transform::from_translation(Vec3::new(100.0, 20.0, 0.0))),
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::RED_700))),
        Collider::ball(25.0),
    ));

    for idx in 0..100 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(40.0, 20.0))),
            MeshMaterial2d(materials.add(Color::from(tailwind::PURPLE_800))),
            Transform::default().with_translation(Vec3::new(100.0 * idx as f32, 400.0, 0.0)),
            Collider::cuboid(20.0, 10.0),
            RigidBody::Fixed,
        ));
    }
}

fn player_input(
    kb: Res<ButtonInput<KeyCode>>,
    player_vehicle_dynamics: Single<
        (&Velocity, &mut ExternalForce, &mut Damping),
        With<PlayerMarker>,
    >,
) {
    const SENSITIVITY: f32 = 1000000.0;
    const SENSITIVITY_REVERSE_BOOST: f32 = 3.0;

    let (player_vehicle_vel, mut acceleration, mut damping) = player_vehicle_dynamics.into_inner();

    acceleration.force = Vec2::ZERO;
    damping.linear_damping = 1.0;

    if kb.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        let sensitivity_modifier = match player_vehicle_vel.linvel.y {
            ..-50.0 => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.force.y = SENSITIVITY * sensitivity_modifier;
        damping.linear_damping = 0.0;
    }

    if kb.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        let sensitivity_modifier = match player_vehicle_vel.linvel.x {
            ..-50.0 => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.force.x = SENSITIVITY * sensitivity_modifier;
        damping.linear_damping = 0.0;
    }

    if kb.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        let sensitivity_modifier = match player_vehicle_vel.linvel.y {
            -50.0.. => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.force.y = -SENSITIVITY * sensitivity_modifier;
        damping.linear_damping = 0.0;
    }

    if kb.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        let sensitivity_modifier = match player_vehicle_vel.linvel.x {
            -50.0.. => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.force.x = -SENSITIVITY * sensitivity_modifier;
        damping.linear_damping = 0.0;
    }
}

fn update_chasers(
    mut chasers: Query<(&Velocity, &Transform, &mut ExternalForce), With<ChaserMarker>>,
    player_trans: Single<&Transform, With<PlayerMarker>>,
) {
    let chasers = chasers.iter_mut().collect::<Vec<_>>();
    let chaser_translations = chasers
        .iter()
        .map(|c| c.1.translation.clone())
        .collect::<Vec<_>>();

    for (velocity, transform, mut acceleration) in chasers {
        let distance_vector = player_trans.translation - transform.translation;

        let distance_scalar = transform
            .translation
            .distance(player_trans.translation)
            .abs();

        if distance_scalar > 150.0 {
            acceleration.force = Vec2::new(
                800000.0
                    * distance_vector.x.signum()
                    * (distance_vector.x.abs() / 400.0)
                    * (if velocity.linvel.x.signum() != distance_vector.x.signum() {
                        3.0
                    } else {
                        1.0
                    }),
                800000.0
                    * distance_vector.y.signum()
                    * (distance_vector.y.abs() / 400.0)
                    * (if velocity.linvel.y.signum() != distance_vector.y.signum() {
                        3.0
                    } else {
                        1.0
                    }),
            );
        } else if distance_scalar < 150.0 {
            acceleration.force = Vec2::new(
                -1000000.0 * distance_vector.x.signum(),
                -1000000.0 * distance_vector.y.signum(),
            )
        }
}

fn enforce_velocity_maximum(mut query: Query<(&mut Velocity, &MaximumVelocity)>) {
    for (mut velocity, maximum_velocity) in &mut query {
        velocity.linvel = velocity
            .linvel
            .clamp(-maximum_velocity.linear, maximum_velocity.linear);
        velocity.angvel = velocity
            .angvel
            .clamp(-maximum_velocity.angular, maximum_velocity.angular);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SPACE GTA".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, (setup,))
        .add_systems(
            FixedUpdate,
            (
                player_input,
                enforce_velocity_maximum,
                // physics::apply_acceleration,
                // physics::apply_drag,
                // physics::apply_velocity,
                // update_camera_scale,
            ),
        )
        .add_systems(Update, update_chasers)
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .run();
}
