use crate::app::components::{Ball, Collidable, Moveable, Paddle, Velocity};
use crate::app::constants::{BALL_RADIUS, BALL_SPEED, PADDLE_HEIGHT, PADDLE_WIDTH};
use bevy::{camera::visibility::RenderLayers, prelude::*};

pub struct Setup;
impl Plugin for Setup {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    //Window Info
    let mut window_height: f32 = 0.0;
    let window = windows.single();
    match window {
        Ok(dim) => {
            window_height = dim.height();
        }
        Err(err) => println!("Error Occured {:?}", err),
    }
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

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            align_items: AlignItems::Center,
            height: percent(100),
            width: percent(100),
            border: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        BorderColor::all(Color::srgb(0.05, 0.85, 0.95)),
        BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
    ));

    // Paddle
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::srgb(0.95, 0.25, 0.65))),
        Transform {
            translation: Vec3::new(0.0, -window_height / 2.0 + PADDLE_WIDTH / 2.0, 5.0),
            scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.0),
            ..default()
        },
        RenderLayers::layer(1),
        Paddle,
        Collidable {
            x: PADDLE_WIDTH,
            y: PADDLE_HEIGHT,
        },
        Moveable,
        Velocity(Vec3::new(0.0, 0.0, 0.0)),
    ));

    //Ball
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::srgb(0.95, 0.25, 0.65))),
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
        Velocity(Vec3::new(BALL_SPEED, BALL_SPEED, 0.0)),
    ));

    //Blocks
}
