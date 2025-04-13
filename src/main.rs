use bevy::{color::palettes::tailwind, prelude::*};

mod physics;

#[derive(Component)]
struct PlayerMarker;

#[derive(Bundle)]
struct PlayerVehicleBundle {
    marker: PlayerMarker,
    velocity: physics::Velocity,
    velocity_maximum: physics::VelocityMaximum,
    acceleration: physics::Acceleration,
    drag: physics::Drag,
    transform: Transform,
    mesh: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
}

impl PlayerVehicleBundle {
    fn new(mesh: Mesh2d, mesh_material: MeshMaterial2d<ColorMaterial>) -> Self {
        Self {
            marker: PlayerMarker,
            velocity: physics::Velocity(Vec2::ZERO),
            velocity_maximum: physics::VelocityMaximum(Vec2::new(700.0, 700.0)),
            acceleration: physics::Acceleration(Vec2::ZERO),
            drag: physics::Drag {
                recommend: 1.0,
                actual: 0.0,
            },
            transform: default(),
            mesh,
            mesh_material,
        }
    }
}

#[derive(Component)]
struct ChaserMarker;

#[derive(Bundle)]
struct ChaserVehicleBundle {
    marker: ChaserMarker,
    velocity: physics::Velocity,
    velocity_maximum: physics::VelocityMaximum,
    acceleration: physics::Acceleration,
    transform: Transform,
    mesh: Mesh2d,
    mesh_material: MeshMaterial2d<ColorMaterial>,
}

impl ChaserVehicleBundle {
    fn new(mesh: Mesh2d, mesh_material: MeshMaterial2d<ColorMaterial>) -> Self {
        Self {
            marker: ChaserMarker,
            velocity: physics::Velocity(Vec2::ZERO),
            velocity_maximum: physics::VelocityMaximum(Vec2::new(680.0, 680.0)),
            acceleration: physics::Acceleration(Vec2::ZERO),
            transform: default(),
            mesh,
            mesh_material,
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
        PlayerVehicleBundle::new(
            Mesh2d(meshes.add(Circle::new(25.0))),
            MeshMaterial2d(materials.add(Color::from(tailwind::TEAL_500))),
        ),
        Camera2d,
        Projection::from(OrthographicProjection {
            scale: 2.0,
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((ChaserVehicleBundle::new(
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::RED_500))),
    ),));
    commands.spawn((ChaserVehicleBundle::new(
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::RED_600))),
    )
    .with_transform(Transform::default().with_translation(Vec3::new(100.0, 0.0, 0.0))),));
    commands.spawn((ChaserVehicleBundle::new(
        Mesh2d(meshes.add(Circle::new(25.0))),
        MeshMaterial2d(materials.add(Color::from(tailwind::RED_700))),
    )
    .with_transform(Transform::default().with_translation(Vec3::new(0.0, 100.0, 0.0))),));

    for idx in 0..100 {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(40.0, 20.0))),
            MeshMaterial2d(materials.add(Color::from(tailwind::PURPLE_800))),
            Transform::default().with_translation(Vec3::new(100.0 * idx as f32, 0.0, 0.0)),
        ));
    }
}

fn player_input(
    kb: Res<ButtonInput<KeyCode>>,
    player_vehicle_velocity_and_drag: Single<
        (
            &physics::Velocity,
            &mut physics::Acceleration,
            &mut physics::Drag,
        ),
        With<PlayerMarker>,
    >,
) {
    const SENSITIVITY: f32 = 1000.0;
    const SENSITIVITY_REVERSE_BOOST: f32 = 2.0;

    let (player_vehicle_vel, mut acceleration, mut player_vehicle_drag) =
        player_vehicle_velocity_and_drag.into_inner();

    **acceleration = Vec2::ZERO;
    player_vehicle_drag.actual = player_vehicle_drag.recommend;

    if kb.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
        let sensitivity_modifier = match player_vehicle_vel.y {
            ..-50.0 => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.y = SENSITIVITY * sensitivity_modifier;
        player_vehicle_drag.actual = 0.0;
    }

    if kb.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        let sensitivity_modifier = match player_vehicle_vel.x {
            ..-50.0 => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.x = SENSITIVITY * sensitivity_modifier;
        player_vehicle_drag.actual = 0.0;
    }

    if kb.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
        let sensitivity_modifier = match player_vehicle_vel.y {
            -50.0.. => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.y = -SENSITIVITY * sensitivity_modifier;
        player_vehicle_drag.actual = 0.0;
    }

    if kb.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        let sensitivity_modifier = match player_vehicle_vel.x {
            -50.0.. => SENSITIVITY_REVERSE_BOOST,
            _ => 1.0,
        };
        acceleration.x = -SENSITIVITY * sensitivity_modifier;
        player_vehicle_drag.actual = 0.0;
    }
}

fn update_chasers(
    mut chasers: Query<
        (&physics::Velocity, &Transform, &mut physics::Acceleration),
        With<ChaserMarker>,
    >,
    player_trans: Single<&Transform, With<PlayerMarker>>,
) {
    const MAX_ACCELERATION: Vec2 = Vec2::new(900.0, 900.0);

    for (velocity, transform, mut acceleration) in chasers.iter_mut() {
        let distance_vector = transform.translation - player_trans.translation;

        let distance_scalar = transform
            .translation
            .distance(player_trans.translation)
            .abs();

        let target = Vec2::new(
            650.0 * distance_vector.y.signum(),
            -650.0 * distance_vector.x.signum(),
        )
        .lerp(
            // Enforce that the chaser isn't getting too close and circles the player
            Vec2::new(
                -800.0 * distance_vector.x.signum() * (100.0 / distance_vector.y.max(100.0)),
                -800.0 * distance_vector.y.signum() * (100.0 / distance_vector.x.max(100.0)),
            ),
            (distance_scalar.min(300.0) / 300.0).tanh(),
        );

        **acceleration = (target - **velocity).clamp(-MAX_ACCELERATION, MAX_ACCELERATION);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup,))
        .add_systems(
            FixedUpdate,
            (
                player_input,
                physics::apply_acceleration,
                physics::apply_drag,
                physics::apply_velocity,
            ),
        )
        .add_systems(Update, update_chasers)
        .insert_resource(Time::<Fixed>::from_hz(64.0))
        .run();
}
