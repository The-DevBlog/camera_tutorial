use bevy::prelude::*;
use bevy_third_person_camera::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ThirdPersonCameraPlugin))
        .add_systems(Startup, (spawn_camera, spawn_world, spawn_player))
        .add_systems(Update, movement)
        .run();
}

fn spawn_camera(mut cmds: Commands) {
    let cam = (
        Camera3dBundle::default(),
        ThirdPersonCamera {
            aim_enabled: true,
            offset_enabled: true,
            offset: Offset::new(1.5, 1.25),
            offset_toggle_enabled: true,
            offset_toggle_speed: 10.0,
            zoom: Zoom::new(5.0, 7.5),
            ..default()
        },
    );
    cmds.spawn(cam);
}

fn spawn_world(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };

    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    cmds.spawn(floor);
    cmds.spawn(light);
}

fn spawn_player(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(2.0))),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        ThirdPersonCameraTarget,
    );

    cmds.spawn(player);
}

fn movement(
    mut player_q: Query<&mut Transform, With<ThirdPersonCameraTarget>>,
    cam_q: Query<&Transform, (With<ThirdPersonCamera>, Without<ThirdPersonCameraTarget>)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = player_q.get_single_mut() else { return };
    let Ok(cam_transform) = cam_q.get_single() else { return };

    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::W) {
        direction += cam_transform.forward();
    }

    if keys.pressed(KeyCode::A) {
        direction += cam_transform.left();
    }

    if keys.pressed(KeyCode::D) {
        direction += cam_transform.right();
    }

    if keys.pressed(KeyCode::S) {
        direction += cam_transform.back();
    }

    direction.y = 0.0;
    let movement = direction.normalize_or_zero() * 5.0 * time.delta_seconds();
    player_transform.translation += movement;

    if direction.length_squared() > 0.0 {
        player_transform.look_to(direction, Vec3::Y);
    }
}
