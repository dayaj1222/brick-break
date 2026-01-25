use crate::app::components::{Ball, Block, Collidable, Moveable, Paddle, Velocity};
use rand::Rng;

use crate::app::constants::{
    BACKGROUND_COLOR, BALL_COLOR, BALL_RADIUS, BALL_SPEED, BLOCK_COLOR, BORDER_COLOR, COLS, GAP,
    PADDLE_COLOR, PADDLE_HEIGHT, PADDLE_WIDTH, ROWS,
};
use crate::app::resources::{GameStates, Score};
use bevy::{camera::visibility::RenderLayers, prelude::*};

pub struct Setup;
impl Plugin for Setup {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(OnEnter(GameStates::Playing), setup)
            .add_systems(OnExit(GameStates::Playing), cleanup_game);
    }
}

#[derive(Resource)]
struct Entities {
    paddle: Entity,
    ball: Entity,
    background: Entity,
    camera: Entity,
    blocks: Vec<Entity>,
    score: Entity,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
    score: Res<Score>,
) {
    //Window Info
    let mut window_height: f32 = 0.0;
    let mut window_width: f32 = 0.0;
    let window = windows.single();
    match window {
        Ok(dim) => {
            window_height = dim.height();
            window_width = dim.width();
        }
        Err(err) => println!("Error Occured {:?}", err),
    }

    let camera_entity = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            RenderLayers::layer(1),
        ))
        .id();

    let background_entity = commands
        .spawn((
            Node {
                position_type: PositionType::Relative,
                align_items: AlignItems::Center,
                height: percent(100),
                width: percent(100),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor::all(BORDER_COLOR),
            BackgroundColor(BACKGROUND_COLOR),
        ))
        .id();

    // Paddle
    let paddle_entity = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(PADDLE_COLOR)),
            Transform {
                translation: Vec3::new(0.0, -window_height / 2.0 + PADDLE_WIDTH / 2.0, 5.0),
                scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.0),
                ..default()
            },
            RenderLayers::layer(1),
            Paddle,
            Collidable {
                x: PADDLE_WIDTH - BALL_RADIUS,
                y: PADDLE_HEIGHT,
            },
            Moveable,
            Velocity(Vec3::new(0.0, 0.0, 0.0)),
        ))
        .id();

    //Ball
    let ball_entity = commands
        .spawn((
            Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
            MeshMaterial2d(materials.add(BALL_COLOR)),
            Transform {
                translation: Vec3::new(
                    0.0,
                    -window_height / 2.0 + PADDLE_WIDTH / 2.0 + PADDLE_HEIGHT / 2.0 + BALL_RADIUS,
                    5.0,
                ),
                ..default()
            },
            RenderLayers::layer(1),
            Ball,
            Moveable,
            Collidable {
                x: BALL_RADIUS * 2.0,
                y: BALL_RADIUS * 2.0,
            },
            Velocity(get_velocity()),
        ))
        .id();

    //Score

    let score_entity = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(10.0),
                left: px(50.0),
                width: px(200),
                height: px(40),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            children![(
                Text::new(format!("Score: {}", score.0)),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )],
            Transform {
                translation: Vec3::new(-window_width + 50.0, window_height - 10.0, 1.0),
                ..default()
            },
        ))
        .id();

    //Blocks

    // Calculate block dimensions
    let block_width = (window_width - 10.0) / COLS as f32;
    let block_height = 30.0;

    let start_x = -window_width / 2.0 + block_width / 2.0 + 5.;
    let start_y = window_height / 2.0 - block_height / 2.0 - 5.;

    let mut block_entities = Vec::new();
    for i in 0..COLS {
        for j in 0..ROWS {
            let x_pos = start_x + (i as f32 * block_width);
            let y_pos = start_y - (j as f32 * block_height);

            let block_entity = commands
                .spawn((
                    Mesh2d(meshes.add(Rectangle::default())),
                    MeshMaterial2d(materials.add(BLOCK_COLOR)),
                    Transform {
                        translation: Vec3::new(x_pos, y_pos, 5.0),
                        scale: Vec3::new(block_width - GAP, block_height - GAP, 0.0),
                        ..default()
                    },
                    RenderLayers::layer(1),
                    Block,
                    Collidable {
                        x: block_width,
                        y: block_height,
                    },
                ))
                .id();
            block_entities.push(block_entity);
        }
    }

    commands.insert_resource(Entities {
        paddle: paddle_entity,
        ball: ball_entity,
        background: background_entity,
        camera: camera_entity,
        blocks: block_entities,
        score: score_entity,
    });
}

// Generates random Velocity
fn get_velocity() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-1.0..=1.0);

    let y = rng.gen_range(0.5..=1.0);

    let vector = Vec3::new(x, y, 0.0).normalize() * BALL_SPEED;
    vector
}

fn cleanup_game(mut commands: Commands, entities: Res<Entities>) {
    commands.entity(entities.paddle).despawn();
    commands.entity(entities.ball).despawn();
    commands.entity(entities.background).despawn();
    commands.entity(entities.camera).despawn();
    commands.entity(entities.score).despawn();

    for block in &entities.blocks {
        commands.entity(*block).despawn();
    }

    commands.remove_resource::<Entities>();
}
