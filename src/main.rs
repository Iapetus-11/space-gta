use bevy::{color::palettes::tailwind, math::vec2, prelude::*};
use bevy_rapier2d::prelude::*;

#[inline]
fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

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
    friction: Friction,
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
            mass: AdditionalMassProperties::Mass(1000.0),
            continuous_collision_detection: Ccd::enabled(),
            restitution: Restitution {
                coefficient: 0.4,
                combine_rule: CoefficientCombineRule::Average,
            },
            friction: Friction {
                coefficient: 0.01,
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
struct Chaser {
    attack_angle: f32,
    acceleration_factor: f32,
}

#[derive(Bundle)]
struct ChaserVehicleBundle {
    chaser: Chaser,

    transform: Transform,
    rigid_body: RigidBody,
    velocity: Velocity,
    max_velocity: MaximumVelocity,
    acceleration: ExternalForce,
    gravity: GravityScale,
    mass: AdditionalMassProperties,
    continuous_collision_detection: Ccd,
    restitution: Restitution,
    friction: Friction,
    locked_axes: LockedAxes,
}

impl ChaserVehicleBundle {
    fn new(chaser: Chaser) -> Self {
        Self {
            chaser,

            transform: default(),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            max_velocity: MaximumVelocity {
                linear: Vec2::new(900.0, 900.0),
                angular: 0.0,
            },
            acceleration: default(),
            gravity: GravityScale(0.0),
            mass: AdditionalMassProperties::Mass(1500.0),
            continuous_collision_detection: Ccd::enabled(),
            restitution: Restitution {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Average,
            },
            friction: Friction {
                coefficient: 0.01,
                combine_rule: CoefficientCombineRule::Average,
            },
            locked_axes: LockedAxes::empty(),
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
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 2.0,
            ..OrthographicProjection::default_2d()
        }),
        Transform::default(),
    ));

    commands.spawn((
        PlayerVehicleBundle::new(),
        Mesh2d(meshes.add(Triangle2d::new(
            vec2(0.0, 40.0),
            vec2(-20.0, 0.0),
            vec2(20.0, 0.0),
        ))),
        MeshMaterial2d(materials.add(Color::from(tailwind::TEAL_500))),
        Collider::triangle(vec2(0.0, 40.0), vec2(-20.0, 0.0), vec2(20.0, 0.0)),
    ));

    for (idx, tw_color) in [
        tailwind::RED_300,
        tailwind::RED_400,
        tailwind::RED_500,
        tailwind::RED_600,
        tailwind::RED_700,
        tailwind::RED_800,
    ]
    .into_iter()
    .enumerate()
    {
        commands.spawn((
            ChaserVehicleBundle::new(Chaser { attack_angle: 0.0, acceleration_factor: 1.0 }).with_transform(Transform::from_translation(Vec3::new(
                100.0 * ((idx as f32 % 3.0) - 2.0),
                100.0 * (idx + 1) as f32,
                0.0,
            ))),
            Mesh2d(meshes.add(Triangle2d::new(
                vec2(0.0, 40.0),
                vec2(-20.0, 0.0),
                vec2(20.0, 0.0),
            ))),
            MeshMaterial2d(materials.add(Color::from(tw_color))),
            Collider::triangle(vec2(0.0, 40.0), vec2(-20.0, 0.0), vec2(20.0, 0.0)),
        ));
    }

    commands.spawn((
        ChaserVehicleBundle::new(Chaser { attack_angle: deg_to_rad(90.0), acceleration_factor: 2.0 })
            .with_transform(Transform::from_translation(Vec3::new(800.0, 800.0, 0.0))),
        Mesh2d(meshes.add(Capsule2d::new(50.0, 200.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::AMBER_500))),
        Collider::capsule_y(100.0, 50.0),
    ));

    for idx in 0..100 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(50.0, 10.0))),
            MeshMaterial2d(materials.add(Color::from(tailwind::PURPLE_800))),
            Transform::default().with_translation(Vec3::new(132.0 * idx as f32, 400.0, 0.0)),
            // Collider::cuboid(20.0, 10.0),
            // RigidBody::Fixed,
        ));
    }

    commands.spawn((
        Transform::from_translation(Vec3::new(-1000.0, 0.0, 0.0)),
        Mesh2d(meshes.add(Rectangle::new(200.0, 200.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::PURPLE_800))),
        Collider::cuboid(100.0, 100.0),
        RigidBody::Dynamic,
        GravityScale(0.0),
    ));
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
    damping.linear_damping = 0.1;

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
    mut chasers: Query<
        (&Velocity, &mut Transform, &mut ExternalForce, &Chaser),
        Without<PlayerMarker>,
    >,
    player_trans: Single<&Transform, With<PlayerMarker>>,
    time: Res<Time>,
) {
    let chasers = chasers.iter_mut().collect::<Vec<_>>();

    for (velocity, mut transform, mut acceleration, chaser) in chasers {
        let distance_vector = player_trans.translation - transform.translation;

        let distance_scalar = transform
            .translation
            .distance(player_trans.translation)
            .abs();

        // Make chasers chase the player
        let chase_force = Vec2::new(
            800000.0
                * distance_vector.x.signum()
                * chaser.acceleration_factor
                * (if velocity.linvel.x.signum() != distance_vector.x.signum() {
                    3.0
                } else {
                    1.0
                }),
            800000.0
                * distance_vector.y.signum()
                * chaser.acceleration_factor
                * (if velocity.linvel.y.signum() != distance_vector.y.signum() {
                    3.0
                } else {
                    1.0
                }),
        );

        // Make chasers circle the player
        let surround_force = Vec2::new(
            900000.0 * distance_vector.x.signum() * chaser.acceleration_factor,
            900000.0 * distance_vector.y.signum() * chaser.acceleration_factor,
        )
        .rotate(Vec2::new(
            std::f32::consts::FRAC_PI_2 * -1.25,
            std::f32::consts::FRAC_PI_2,
        ));

        acceleration.force = Vec2::lerp(
            surround_force,
            chase_force,
            distance_scalar.min(500.0) / 500.0,
        );

        let player_angle = (vec2(player_trans.translation.x, player_trans.translation.y)
            - vec2(transform.translation.x, transform.translation.y))
        .to_angle();
        let angle_modifier = if distance_scalar < 700.0 {
            chaser.attack_angle
        } else {
            0.0_f32
        };
        transform.rotation = transform.rotation.lerp(Quat::from_rotation_z(
            player_angle - deg_to_rad(90.0) + angle_modifier,
        ), time.delta_secs());
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

fn update_view_based_on_physics(
    player: Single<(&Velocity, &ExternalForce, &mut Transform), With<PlayerMarker>>,
    camera: Single<(&mut Transform,), (With<Camera2d>, Without<PlayerMarker>)>,
    time: Res<Time>,
) {
    let (player_vel, player_accel, mut player_transf) = player.into_inner();
    let (mut camera_transf,) = camera.into_inner();

    // Rotate player ship to face direction it's moving
    let rotation_angle = (if player_accel.force.length_squared().abs() > 2.0 {
        player_accel.force
    } else {
        player_vel.linvel
    })
    .to_angle();
    player_transf.rotation = player_transf.rotation.lerp(
        Quat::from_rotation_z(rotation_angle - deg_to_rad(90.0)),
        7.5 * time.delta_secs(),
    );

    // Have camera track player ship
    camera_transf.translation = camera_transf.translation.lerp(
        (player_transf.translation
            + Vec3::new(player_vel.linvel.x, player_vel.linvel.y, 0.0) / 1.5)
            .with_z(camera_transf.translation.z)
            .round(),
        2.0 * time.delta_secs(),
    );
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
        .add_systems(FixedUpdate, (enforce_velocity_maximum,))
        .add_systems(
            Update,
            (player_input, update_chasers, update_view_based_on_physics),
        )
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .run();
}
