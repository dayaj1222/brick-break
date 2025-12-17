use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::css::{BLUE, GREEN, PURPLE, RED, YELLOW},
    prelude::*,
};
use rand::Rng;

const GRAVITY: Vec2 = Vec2::new(0., -0.5);
const JUMP_STRENGTH: f32 = 15.0;
const OBSTACLE_SPEED: f32 = 4.0;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Rect {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
struct PhysicsStates {
    pub is_on_ground: bool,
}

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct ObstacleSpawnTime(Timer);

impl Rect {
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn intersect(&self, self_pos: Vec2, other: &Rect, other_pos: Vec2) -> bool {
        let self_half_w = self.width / 2.0;
        let self_half_h = self.height / 2.0;
        let other_half_w = other.width / 2.0;
        let other_half_h = other.height / 2.0;

        (self_pos.x - self_half_w < other_pos.x + other_half_w)
            && (self_pos.x + self_half_w > other_pos.x - other_half_w)
            && (self_pos.y - self_half_h < other_pos.y + other_half_h)
            && (self_pos.y + self_half_h > other_pos.y - other_half_h)
    }
}

// Random float between min and max
fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

// Random element from array
fn random_choice<T: Clone>(choices: &[T]) -> T {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..choices.len());
    choices[index].clone()
}

fn spawn_random_obstacle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    screen_width: f32,
    screen_height: f32,
) {
    let obstacle_width = random_range(50.0, 100.0);
    let obstacle_height = random_range(50.0, 150.0);

    // Random position
    let x = screen_width + 10.0;

    // Ground parameters (same as in your setup)
    let ground_height = screen_height * 0.12;
    let ground_y = -screen_height / 2.0 + ground_height / 2.0;

    // Place obstacle on top of ground
    let y = ground_y + ground_height / 2.0 + obstacle_height / 2.0;

    // Random color
    let colors = [RED, BLUE, GREEN, YELLOW, PURPLE];
    let color = random_choice(&colors);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(color))),
        Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(obstacle_width, obstacle_height, 1.0)),
        RenderLayers::layer(1),
        Rect::new(obstacle_width, obstacle_height),
        Obstacle,
    ));
}

#[derive(Component)]
struct Ground;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(ObstacleSpawnTime(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
        .add_systems(
            Update,
            (
                move_sprite,
                apply_gravity,
                check_collision,
                move_obstacles,
                spawn_obstacles_over_time,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) {
    let window = window_query.single().unwrap();
    let screen_width = window.width();
    let screen_height = window.height();
    // Cameras
    commands.spawn((Camera2d, IsDefaultUiCamera));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        RenderLayers::layer(1),
    ));

    //Background
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            height: Val::Px(screen_height),
            width: Val::Px(screen_width),
            ..default()
        },
        BackgroundColor(Color::srgb(0.078, 0.094, 0.157)),
    ));

    // Ground (converted to Sprite)
    let ground_height = screen_height * 0.12;
    let ground_y = -screen_height / 2.0 + ground_height / 2.0;

    commands.spawn((
        Sprite {
            color: Color::srgb(0.38, 0.27, 0.19),
            custom_size: Some(Vec2::new(screen_width, ground_height)),
            ..default()
        },
        Transform::from_xyz(0., ground_y, 0.),
        Ground,
        Rect::new(screen_width, ground_height),
        RenderLayers::layer(1),
    ));

    // Character
    commands.spawn((
        Sprite {
            image: asset_server.load("character.png"),
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        RenderLayers::layer(1),
        Velocity(Vec2::new(0., 0.)),
        Rect::new(75., 75.),
        PhysicsStates {
            is_on_ground: false,
        },
    ));
}

fn spawn_obstacles_over_time(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<ObstacleSpawnTime>,
    window_query: Query<&Window>,
) {
    // Tick the timer with delta time
    if timer.0.tick(time.delta()).just_finished() {
        let window = window_query.single().unwrap();
        let screen_width = window.width();
        let screen_height = window.height();

        spawn_random_obstacle(
            &mut commands,
            &mut meshes,
            &mut materials,
            screen_width,
            screen_height,
        );
    }
}

fn move_obstacles(query: Query<&mut Transform, With<Obstacle>>) {
    for mut transform in query {
        transform.translation.x -= OBSTACLE_SPEED;
    }
}

fn move_sprite(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (&mut Transform, &mut Velocity, &mut PhysicsStates),
        (With<Sprite>, Without<Ground>),
    >,
) {
    for (mut transform, mut velocity, mut states) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard.just_pressed(KeyCode::Space) {
            if states.is_on_ground {
                velocity.0.y = JUMP_STRENGTH;
                states.is_on_ground = false;
            }
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * 8.0;
        }
    }
}

fn apply_gravity(mut query: Query<(&mut Transform, &mut Velocity), With<Sprite>>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0 += GRAVITY;
        transform.translation += velocity.0.extend(0.0);
    }
}

fn check_collision(
    mut movers: Query<
        (&mut Transform, &mut Velocity, &Rect, &mut PhysicsStates),
        (With<Sprite>, Without<Ground>, Without<Obstacle>),
    >,
    colliders: Query<(&Transform, &Rect), Or<(With<Ground>, With<Obstacle>)>>,
) {
    for (mut transform_a, mut velocity, rect_a, mut states) in movers.iter_mut() {
        for (transform_b, rect_b) in colliders.iter() {
            if rect_a.intersect(
                transform_a.translation.truncate(),
                rect_b,
                transform_b.translation.truncate(),
            ) {
                let ground_top = transform_b.translation.y + rect_b.height / 2.0;
                let sprite_half_height = rect_a.height / 2.0;
                transform_a.translation.y = ground_top + sprite_half_height;
                states.is_on_ground = true;
                if velocity.0.y < 0.0 {
                    velocity.0.y = 0.0;
                }
            }
        }
    }
}
