use arcade_core::{ArcadePlugin, GamePhase, despawn_hint, focused, spawn_hint};
use bevy::prelude::*;

const ARENA_W: f32 = 440.;
const ARENA_H: f32 = 440.;
const PADDLE_W: f32 = 12.;
const PADDLE_H: f32 = 80.;
const PADDLE_SPEED: f32 = 320.;
const AI_SPEED: f32 = 240.;
const BALL_SIZE: f32 = 12.;
const BALL_SPEED: f32 = 260.;
const WIN_SCORE: u32 = 11;

fn main() {
    App::new()
        .add_plugins(ArcadePlugin {
            canvas: "#pong-canvas",
        })
        .insert_resource(Score::default())
        .add_systems(Startup, (setup, show_hint))
        .add_systems(
            Update,
            start.run_if(not(in_state(GamePhase::Playing))).run_if(focused),
        )
        .add_systems(
            Update,
            (move_player, move_ai, move_ball, score_point)
                .chain()
                .run_if(in_state(GamePhase::Playing))
                .run_if(focused),
        )
        .add_systems(Update, update_score_text)
        .add_systems(OnEnter(GamePhase::Playing), despawn_hint)
        .add_systems(OnEnter(GamePhase::GameOver), show_hint)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ai;

#[derive(Component)]
struct Ball(Vec2);

#[derive(Component)]
struct ScoreText;

#[derive(Resource, Default)]
struct Score {
    player: u32,
    ai: u32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("0 - 0"),
        Transform::from_xyz(0.0, ARENA_H / 2. + 20., 0.0),
        ScoreText,
    ));

    let paddle = meshes.add(Rectangle::new(PADDLE_W, PADDLE_H));
    let white = materials.add(Color::srgb_u8(202, 211, 245));

    commands.spawn((
        Mesh2d(paddle.clone()),
        MeshMaterial2d(white.clone()),
        Transform::from_xyz(-ARENA_W / 2. + PADDLE_W, 0., 0.),
        Player,
    ));
    commands.spawn((
        Mesh2d(paddle),
        MeshMaterial2d(white.clone()),
        Transform::from_xyz(ARENA_W / 2. - PADDLE_W, 0., 0.),
        Ai,
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(BALL_SIZE, BALL_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb_u8(160, 204, 15))),
        Transform::from_xyz(0., 0., 0.),
        Ball(Vec2::new(-1., 0.4).normalize() * BALL_SPEED),
    ));
}

fn show_hint(mut commands: Commands, score: Res<Score>) {
    let text = if score.player >= WIN_SCORE {
        "You win!\nPress Space to play again"
    } else if score.ai >= WIN_SCORE {
        "You lose!\nPress Space to play again"
    } else {
        "Pong\nW/S or Up/Down - Press Space to Start"
    };
    spawn_hint(&mut commands, text);
}

fn start(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut score: ResMut<Score>,
    mut balls: Query<(&mut Transform, &mut Ball)>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    if score.player >= WIN_SCORE || score.ai >= WIN_SCORE {
        score.player = 0;
        score.ai = 0;
    }
    for (mut transform, mut ball) in &mut balls {
        transform.translation = Vec3::ZERO;
        ball.0 = Vec2::new(-1., 0.4).normalize() * BALL_SPEED;
    }
    next_state.set(GamePhase::Playing);
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut paddles: Query<&mut Transform, With<Player>>,
) {
    let up = keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW);
    let down = keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS);
    let dir = (up as i32 - down as i32) as f32;

    for mut transform in &mut paddles {
        transform.translation.y =
            clamp_paddle(transform.translation.y + dir * PADDLE_SPEED * time.delta_secs());
    }
}

fn move_ai(
    time: Res<Time>,
    balls: Query<&Transform, With<Ball>>,
    mut paddles: Query<&mut Transform, (With<Ai>, Without<Ball>)>,
) {
    let Ok(ball) = balls.single() else {
        return;
    };
    for mut transform in &mut paddles {
        let delta = ball.translation.y - transform.translation.y;
        let step = delta.clamp(-AI_SPEED * time.delta_secs(), AI_SPEED * time.delta_secs());
        transform.translation.y = clamp_paddle(transform.translation.y + step);
    }
}

fn clamp_paddle(y: f32) -> f32 {
    let limit = ARENA_H / 2. - PADDLE_H / 2.;
    y.clamp(-limit, limit)
}

fn move_ball(
    time: Res<Time>,
    mut balls: Query<(&mut Transform, &mut Ball)>,
    // Without<Ball> keeps this disjoint from the ball query; bevy checks that statically
    paddles: Query<&Transform, (Or<(With<Player>, With<Ai>)>, Without<Ball>)>,
) {
    for (mut transform, mut ball) in &mut balls {
        transform.translation.x += ball.0.x * time.delta_secs();
        transform.translation.y += ball.0.y * time.delta_secs();

        // top/bottom walls
        let limit = ARENA_H / 2. - BALL_SIZE / 2.;
        if transform.translation.y.abs() > limit {
            transform.translation.y = transform.translation.y.clamp(-limit, limit);
            ball.0.y = -ball.0.y;
        }

        for paddle in &paddles {
            let dx = (transform.translation.x - paddle.translation.x).abs();
            let dy = (transform.translation.y - paddle.translation.y).abs();
            let hit = dx <= (PADDLE_W + BALL_SIZE) / 2. && dy <= (PADDLE_H + BALL_SIZE) / 2.;
            // only bounce when travelling into the paddle, so the ball can't get stuck inside it
            let incoming = (paddle.translation.x - transform.translation.x).signum() == ball.0.x.signum();

            if hit && incoming {
                ball.0.x = -ball.0.x;
                // steer by where the ball struck the paddle
                let offset = (transform.translation.y - paddle.translation.y) / (PADDLE_H / 2.);
                ball.0.y = offset * BALL_SPEED * 0.75;
                ball.0 = ball.0.normalize() * BALL_SPEED;
            }
        }
    }
}

fn score_point(
    mut balls: Query<(&mut Transform, &mut Ball)>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    for (mut transform, mut ball) in &mut balls {
        let out = ARENA_W / 2. + BALL_SIZE;
        if transform.translation.x.abs() < out {
            continue;
        }

        let player_scored = transform.translation.x > 0.;
        if player_scored {
            score.player += 1;
        } else {
            score.ai += 1;
        }

        transform.translation = Vec3::ZERO;
        // serve towards whoever just conceded
        ball.0 = Vec2::new(if player_scored { 1. } else { -1. }, 0.4).normalize() * BALL_SPEED;

        if score.player >= WIN_SCORE || score.ai >= WIN_SCORE {
            next_state.set(GamePhase::GameOver);
        }
    }
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text2d, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            text.0 = format!("{} - {}", score.player, score.ai);
        }
    }
}
