use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

enum SnakeTypes {
    Idle,
    Moving,
    Crashed,
}

pub struct Snake;

pub struct SnakeState {
    state: Vec3,
}

#[derive(Component)]
struct SnakeHead;

impl Plugin for Snake {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render);
        app.add_systems(Update, move_the_snake);
    }
}

fn render(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let head = Mesh2dHandle(meshes.add(Rectangle::new(30.0, 40.0)));

    cmd.spawn(MaterialMesh2dBundle {
        mesh: head,
        material: materials.add(Color::srgb_u8(114, 191, 120)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    })
    .insert(Name::new("SnakeHead"))
    .insert(SnakeHead);
}

fn move_the_snake(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<SnakeHead>>,

    time: Res<Time>,
) {
    let mut transform = match query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => {
            println!("No SnakeHead entity found!");
            return;
        }
    };

    let speed = 200.0;
    let rotation_speed = 2.0;

    if keys.pressed(KeyCode::KeyW) {
        let forward = transform.rotation * Vec3::Y;
        // transform.translation.y += forward * speed * time.delta_seconds();
    }
    if keys.pressed(KeyCode::KeyS) {
        transform.translation.y -= speed * time.delta_seconds();
        transform.rotation = Quat::from_rotation_y(-rotation_speed * time.delta_seconds());
    }
    if keys.pressed(KeyCode::KeyD) {
        transform.translation.x += speed * time.delta_seconds();
        transform.rotation = Quat::from_rotation_x(rotation_speed * time.delta_seconds());
    }
    if keys.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed * time.delta_seconds();
        transform.rotation = Quat::from_rotation_x(-rotation_speed * time.delta_seconds());
    }
}
