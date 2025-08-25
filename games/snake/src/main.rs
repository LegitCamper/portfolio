use bevy::prelude::*;
use rand::Rng;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;
const CELL_SIZE: f32 = 20.; // pixels per cell

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#snake-canvas".into()),
                // ... any other window properties ...
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(Score(0))
        .insert_resource(SnakeBody {
            positions: vec![IVec2::new(10, 10)],
        })
        .insert_resource(SnakeDirection(Direction::Up))
        .add_event::<FruitEatenEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                start.run_if(in_state(GameState::GameOver)),
                handle_input.run_if(in_state(GameState::Running)),
                move_snake.run_if(in_state(GameState::Running)),
                check_death_collision.run_if(in_state(GameState::Running)),
                check_fruit_collision.run_if(in_state(GameState::Running)),
                handle_fruit_eaten.run_if(in_state(GameState::Running)),
                grow_snake.run_if(in_state(GameState::Running)),
                update_score_text,
            )
                .chain(),
        )
        .add_systems(OnEnter(GameState::GameOver), show_start_text)
        .add_systems(OnEnter(GameState::Running), hide_start_text)
        .add_systems(OnEnter(GameState::GameOver), cleanup_entities)
        .run();
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    Running,
    #[default]
    GameOver,
}

#[derive(Resource)]
struct SnakeBody {
    positions: Vec<IVec2>, // positions[0] is the head, positions[n] is the tail
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Resource, Deref, DerefMut)]
struct SnakeDirection(Direction);

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Fruit {
    position: IVec2, // grid coords
}

#[derive(Event, Default)]
struct FruitEatenEvent;

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct StartText;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    // snake starts with length 1 at (10, 10)
    let start_pos = IVec2::new(10, 10);

    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("Score: 0"),
        TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
        Transform::from_xyz(-300.0, 200.0, 0.0),
        ScoreText,
    ));

    for x in 0..GRID_WIDTH {
        for y in 0..GRID_HEIGHT {
            // Convert grid coords to world coords (centered)
            let world_x = (x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;
            let world_y = (y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;

            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(15., 15.))),
                MeshMaterial2d(materials.add(Color::srgb_u8(97, 98, 107))),
                Transform::from_xyz(world_x, world_y, -1.0),
            ));
        }
    }
}

fn start(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    mut snake_direction: ResMut<SnakeDirection>,
    mut snake: ResMut<SnakeBody>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // snake starts with length 1 at (10, 10)
        let start_pos = IVec2::new(10, 10);

        snake.positions.clear();
        snake.positions.push(start_pos);
        score.0 = 0;
        snake_direction.0 = Direction::Up;

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE))),
            MeshMaterial2d(materials.add(Color::srgb_u8(160, 204, 15))),
            Transform::from_translation(grid_to_world(start_pos)),
            SnakeSegment,
        ));

        spawn_fruit(&mut commands, &mut meshes, &mut materials);

        next_state.set(GameState::Running);
    }
}

fn grid_to_world(pos: IVec2) -> Vec3 {
    let world_x = (pos.x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;
    let world_y = (pos.y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;
    Vec3::new(world_x, world_y, 0.0)
}

fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut direction: ResMut<SnakeDirection>) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp)
        || keyboard_input.just_pressed(KeyCode::KeyW) && direction.0 != Direction::Down
    {
        direction.0 = Direction::Up;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown)
        || keyboard_input.just_pressed(KeyCode::KeyS) && direction.0 != Direction::Up
    {
        direction.0 = Direction::Down;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft)
        || keyboard_input.just_pressed(KeyCode::KeyA) && direction.0 != Direction::Right
    {
        direction.0 = Direction::Left;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::KeyD) && direction.0 != Direction::Left
    {
        direction.0 = Direction::Right;
    }
}

fn move_snake(
    time: Res<Time>,
    mut body: ResMut<SnakeBody>,
    mut segments: Query<&mut Transform, With<SnakeSegment>>,
    mut timer: Local<Timer>,
    direction: Res<SnakeDirection>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.2, TimerMode::Repeating);
    }
    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    // Move body positions
    let mut new_head = body.positions[0];
    match direction.0 {
        Direction::Up => new_head.y += 1,
        Direction::Down => new_head.y -= 1,
        Direction::Left => new_head.x -= 1,
        Direction::Right => new_head.x += 1,
    }
    body.positions.insert(0, new_head);
    body.positions.pop();

    // Sync entity transforms with body
    for (pos, mut transform) in body.positions.iter().zip(segments.iter_mut()) {
        transform.translation = grid_to_world(*pos);
    }
}

fn spawn_fruit(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let fruit_pos = IVec2::new(rng.gen_range(0..GRID_WIDTH), rng.gen_range(0..GRID_HEIGHT));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb_u8(100, 100, 10))),
        Transform::from_translation(grid_to_world(fruit_pos)),
        Fruit {
            position: fruit_pos,
        },
    ));
}

fn check_fruit_collision(
    body: Res<SnakeBody>,
    fruits: Query<&Fruit>,
    mut events: EventWriter<FruitEatenEvent>,
) {
    let head = body.positions[0];
    for fruit in &fruits {
        if fruit.position == head {
            events.send(FruitEatenEvent); // only sends event
        }
    }
}

fn check_death_collision(body: Res<SnakeBody>, mut next_state: ResMut<NextState<GameState>>) {
    let head = body.positions[0];

    // wall collision
    if head.x < 0 || head.x >= GRID_WIDTH || head.y < 0 || head.y >= GRID_HEIGHT {
        next_state.set(GameState::GameOver);
        return;
    }

    // self collision (ignore last tail segment because it moves)
    if body.positions.len() > 2 && body.positions[1..body.positions.len() - 1].contains(&head) {
        next_state.set(GameState::GameOver);
    }
}

fn handle_fruit_eaten(
    mut events: EventReader<FruitEatenEvent>,
    mut query: Query<(&mut Fruit, &mut Transform)>,
    snake: Res<SnakeBody>,
    mut score: ResMut<Score>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in events.read() {
        score.0 += 1;

        for (mut fruit, mut transform) in &mut query {
            let mut new_pos;

            loop {
                new_pos = IVec2::new(rng.gen_range(0..GRID_WIDTH), rng.gen_range(0..GRID_HEIGHT));
                if !snake.positions.contains(&new_pos) {
                    break;
                }
            }

            fruit.position = new_pos;
            transform.translation = grid_to_world(new_pos);
        }
    }
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text2d, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            text.0 = format!("Score: {}", score.0);
        }
    }
}

fn grow_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut body: ResMut<SnakeBody>,
    mut events: EventReader<FruitEatenEvent>,
    mut segments: Query<(Entity, &mut SnakeSegment)>,
) {
    for _ in events.read() {
        // add new tail position
        let tail_pos = *body.positions.last().unwrap();
        body.positions.push(tail_pos);

        // spawn new entity
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE))),
            MeshMaterial2d(materials.add(Color::srgb_u8(140, 180, 15))),
            Transform::from_translation(grid_to_world(tail_pos)),
            SnakeSegment,
        ));
    }
}

fn cleanup_entities(
    mut commands: Commands,
    segments: Query<Entity, With<SnakeSegment>>,
    mut snake: ResMut<SnakeBody>,
    fruits: Query<Entity, With<Fruit>>,
) {
    for e in &segments {
        commands.entity(e).despawn();
    }
    snake.positions.clear();

    for e in &fruits {
        commands.entity(e).despawn();
    }
}

fn show_start_text(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Press Space to Start"),
        TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
        Transform::from_xyz(0.0, 0.0, 1.0),
        StartText,
    ));
}

fn hide_start_text(mut commands: Commands, query: Query<Entity, With<StartText>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
