use bevy::prelude::*;
use rand::Rng;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;
const CELL_SIZE: f32 = 20.; // pixels per cell

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .add_event::<FruitEatenEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_snake,
                handle_input,
                handle_fruit_eaten,
                check_collisions,
                update_score_text,
            ),
        )
        .run();
}

#[derive(Component)]
struct Snake {
    direction: Direction,
    position: IVec2, // grid coords
}

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    let mut rng = rand::rng();

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

    spawn_fruit(&mut commands, &mut meshes, &mut materials);

    let snake_pos = IVec2::new(10, 10);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE))),
        MeshMaterial2d(materials.add(Color::srgb_u8(160, 204, 15))),
        Transform::from_translation(grid_to_world(snake_pos)),
        Snake {
            direction: Direction::Up,
            position: snake_pos,
        },
    ));
}

fn grid_to_world(pos: IVec2) -> Vec3 {
    let world_x = (pos.x as f32 - GRID_WIDTH as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;
    let world_y = (pos.y as f32 - GRID_HEIGHT as f32 / 2.0) * CELL_SIZE + CELL_SIZE / 2.0;
    Vec3::new(world_x, world_y, 0.0)
}

fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Snake>) {
    for mut snake in &mut query {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) && snake.direction != Direction::Down {
            snake.direction = Direction::Up;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) && snake.direction != Direction::Up {
            snake.direction = Direction::Down;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowLeft) && snake.direction != Direction::Right {
            snake.direction = Direction::Left;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) && snake.direction != Direction::Left {
            snake.direction = Direction::Right;
        }
    }
}

fn move_snake(
    time: Res<Time>,
    mut query: Query<(&mut Snake, &mut Transform)>,
    mut timer: Local<Timer>,
) {
    // initialize timer once
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(0.2, TimerMode::Repeating);
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    for (mut snake, mut transform) in &mut query {
        // Move one cell based on stored direction
        match snake.direction {
            Direction::Up => snake.position.y += 1,
            Direction::Down => snake.position.y -= 1,
            Direction::Left => snake.position.x -= 1,
            Direction::Right => snake.position.x += 1,
        }

        // Update transform
        transform.translation = grid_to_world(snake.position);
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

fn check_collisions(
    mut snake_query: Query<&Snake>,
    mut fruit_query: Query<(Entity, &Fruit)>,
    mut commands: Commands,
    mut writer: EventWriter<FruitEatenEvent>,
) {
    for snake in &mut snake_query {
        for (fruit_entity, fruit) in &mut fruit_query {
            if snake.position == fruit.position {
                writer.write(FruitEatenEvent);
            }
        }
    }
}

fn handle_fruit_eaten(
    mut events: EventReader<FruitEatenEvent>,
    mut query: Query<(&mut Fruit, &mut Transform)>,
    mut score: ResMut<Score>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in events.read() {
        score.0 += 1;

        for (mut fruit, mut transform) in &mut query {
            let new_pos = IVec2::new(rng.gen_range(0..GRID_WIDTH), rng.gen_range(0..GRID_HEIGHT));

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
