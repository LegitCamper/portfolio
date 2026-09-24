use arcade_core::{ArcadePlugin, GamePhase, despawn_hint, focused, spawn_hint};
use bevy::prelude::*;

const ARENA_W: f32 = 440.;
const ARENA_H: f32 = 440.;
const PADDLE_W: f32 = 80.;
const PADDLE_H: f32 = 12.;
const PADDLE_SPEED: f32 = 380.;
const PADDLE_Y: f32 = -ARENA_H / 2. + 30.;
const BALL_SIZE: f32 = 10.;
const BALL_SPEED: f32 = 280.;
const BRICK_COLS: i32 = 8;
const BRICK_ROWS: i32 = 5;
const BRICK_W: f32 = 48.;
const BRICK_H: f32 = 18.;
const START_LIVES: u32 = 3;

fn main() {
    App::new()
        .add_plugins(ArcadePlugin {
            canvas: "#breakout-canvas",
        })
        .insert_resource(Score(0))
        .insert_resource(Lives(START_LIVES))
        .add_systems(Startup, (setup, show_hint))
        .add_systems(
            Update,
            start.run_if(not(in_state(GamePhase::Playing))).run_if(focused),
        )
        .add_systems(
            Update,
            (move_paddle, move_ball, lose_life)
                .chain()
                .run_if(in_state(GamePhase::Playing))
                .run_if(focused),
        )
        .add_systems(Update, update_hud)
        .add_systems(OnEnter(GamePhase::Playing), despawn_hint)
        .add_systems(OnEnter(GamePhase::GameOver), show_hint)
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball(Vec2);

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Hud;

#[derive(Resource, Deref, DerefMut)]
struct Score(u32);

#[derive(Resource, Deref, DerefMut)]
struct Lives(u32);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("Score: 0   Lives: 3"),
        Transform::from_xyz(0.0, ARENA_H / 2. + 20., 0.0),
        Hud,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PADDLE_W, PADDLE_H))),
        MeshMaterial2d(materials.add(Color::srgb_u8(202, 211, 245))),
        Transform::from_xyz(0., PADDLE_Y, 0.),
        Paddle,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb_u8(160, 204, 15))),
        Transform::from_xyz(0., PADDLE_Y + 24., 0.),
        Ball(Vec2::new(0.5, 1.).normalize() * BALL_SPEED),
    ));

    spawn_bricks(&mut commands, &mut meshes, &mut materials);
}

fn spawn_bricks(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(BRICK_W, BRICK_H));
    for row in 0..BRICK_ROWS {
        let color = materials.add(Color::srgb_u8(120 + (row as u8) * 20, 90, 180));
        for col in 0..BRICK_COLS {
            let x = (col as f32 - (BRICK_COLS - 1) as f32 / 2.) * (BRICK_W + 6.);
            let y = ARENA_H / 2. - 40. - row as f32 * (BRICK_H + 6.);
            commands.spawn((
                Mesh2d(mesh.clone()),
                MeshMaterial2d(color.clone()),
                Transform::from_xyz(x, y, 0.),
                Brick,
            ));
        }
    }
}

fn show_hint(mut commands: Commands, lives: Res<Lives>, bricks: Query<(), With<Brick>>) {
    let text = if bricks.is_empty() {
        "Cleared!\nPress Space to play again"
    } else if lives.0 == 0 {
        "Game over\nPress Space to play again"
    } else {
        "Breakout\nLeft/Right or A/D - Press Space to Start"
    };
    spawn_hint(&mut commands, text);
}

#[allow(clippy::too_many_arguments)]
fn start(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut score: ResMut<Score>,
    mut lives: ResMut<Lives>,
    mut balls: Query<(&mut Transform, &mut Ball)>,
    bricks: Query<Entity, With<Brick>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    if lives.0 == 0 || bricks.is_empty() {
        for entity in &bricks {
            commands.entity(entity).despawn();
        }
        spawn_bricks(&mut commands, &mut meshes, &mut materials);
        score.0 = 0;
        lives.0 = START_LIVES;
    }

    reset_ball(&mut balls);
    next_state.set(GamePhase::Playing);
}

fn reset_ball(balls: &mut Query<(&mut Transform, &mut Ball)>) {
    for (mut transform, mut ball) in balls.iter_mut() {
        transform.translation = Vec3::new(0., PADDLE_Y + 24., 0.);
        ball.0 = Vec2::new(0.5, 1.).normalize() * BALL_SPEED;
    }
}

fn move_paddle(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paddles: Query<&mut Transform, With<Paddle>>,
) {
    let left = keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA);
    let right = keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD);
    let dir = (right as i32 - left as i32) as f32;
    let limit = ARENA_W / 2. - PADDLE_W / 2.;

    for mut transform in &mut paddles {
        transform.translation.x =
            (transform.translation.x + dir * PADDLE_SPEED * time.delta_secs()).clamp(-limit, limit);
    }
}

fn move_ball(
    mut commands: Commands,
    time: Res<Time>,
    mut score: ResMut<Score>,
    mut balls: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
    paddles: Query<&Transform, With<Paddle>>,
    bricks: Query<(Entity, &Transform), (With<Brick>, Without<Ball>, Without<Paddle>)>,
) {
    for (mut transform, mut ball) in &mut balls {
        transform.translation.x += ball.0.x * time.delta_secs();
        transform.translation.y += ball.0.y * time.delta_secs();

        let x_limit = ARENA_W / 2. - BALL_SIZE / 2.;
        if transform.translation.x.abs() > x_limit {
            transform.translation.x = transform.translation.x.clamp(-x_limit, x_limit);
            ball.0.x = -ball.0.x;
        }
        let y_limit = ARENA_H / 2. - BALL_SIZE / 2.;
        if transform.translation.y > y_limit {
            transform.translation.y = y_limit;
            ball.0.y = -ball.0.y;
        }

        for paddle in &paddles {
            let hit = overlaps(
                transform.translation.truncate(),
                BALL_SIZE,
                BALL_SIZE,
                paddle.translation.truncate(),
                PADDLE_W,
                PADDLE_H,
            );
            if hit && ball.0.y < 0. {
                let offset = (transform.translation.x - paddle.translation.x) / (PADDLE_W / 2.);
                ball.0 = Vec2::new(offset, 1.).normalize() * BALL_SPEED;
            }
        }

        for (entity, brick) in &bricks {
            if !overlaps(
                transform.translation.truncate(),
                BALL_SIZE,
                BALL_SIZE,
                brick.translation.truncate(),
                BRICK_W,
                BRICK_H,
            ) {
                continue;
            }

            commands.entity(entity).despawn();
            score.0 += 10;

            // bounce off the shallower axis of penetration
            let dx = (transform.translation.x - brick.translation.x).abs() / (BRICK_W + BALL_SIZE);
            let dy = (transform.translation.y - brick.translation.y).abs() / (BRICK_H + BALL_SIZE);
            if dx > dy {
                ball.0.x = -ball.0.x;
            } else {
                ball.0.y = -ball.0.y;
            }
            break;
        }
    }
}

fn overlaps(a: Vec2, aw: f32, ah: f32, b: Vec2, bw: f32, bh: f32) -> bool {
    (a.x - b.x).abs() <= (aw + bw) / 2. && (a.y - b.y).abs() <= (ah + bh) / 2.
}

fn lose_life(
    mut lives: ResMut<Lives>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut balls: Query<(&mut Transform, &mut Ball)>,
    bricks: Query<(), With<Brick>>,
) {
    if bricks.is_empty() {
        next_state.set(GamePhase::GameOver);
        return;
    }

    let dropped = balls
        .iter()
        .any(|(transform, _)| transform.translation.y < -ARENA_H / 2. - BALL_SIZE);
    if !dropped {
        return;
    }

    lives.0 = lives.0.saturating_sub(1);
    reset_ball(&mut balls);

    if lives.0 == 0 {
        next_state.set(GamePhase::GameOver);
    }
}

fn update_hud(score: Res<Score>, lives: Res<Lives>, mut query: Query<&mut Text2d, With<Hud>>) {
    if !score.is_changed() && !lives.is_changed() {
        return;
    }
    for mut text in &mut query {
        text.0 = format!("Score: {}   Lives: {}", score.0, lives.0);
    }
}
