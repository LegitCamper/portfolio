use arcade_core::{ArcadePlugin, GamePhase, despawn_hint, focused, spawn_hint};
use bevy::prelude::*;

const COLS: i32 = 10;
const ROWS: i32 = 20;
const CELL: f32 = 20.;
const FALL_SECS: f32 = 0.5;
const SOFT_DROP_SECS: f32 = 0.05;

fn main() {
    App::new()
        .add_plugins(ArcadePlugin {
            canvas: "#tetris-canvas",
        })
        .insert_resource(Board::default())
        .insert_resource(Piece::spawn_random())
        .insert_resource(Score(0))
        .add_systems(Startup, (setup, show_hint))
        .add_systems(
            Update,
            start.run_if(not(in_state(GamePhase::Playing))).run_if(focused),
        )
        .add_systems(
            Update,
            (handle_input, gravity, render)
                .chain()
                .run_if(in_state(GamePhase::Playing))
                .run_if(focused),
        )
        .add_systems(Update, update_score_text)
        .add_systems(OnEnter(GamePhase::Playing), despawn_hint)
        .add_systems(OnEnter(GamePhase::GameOver), show_hint)
        .run();
}

/// Landed cells, row-major, `None` = empty. Index 0 is the bottom row.
#[derive(Resource)]
struct Board {
    cells: Vec<Option<u8>>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: vec![None; (COLS * ROWS) as usize],
        }
    }
}

impl Board {
    fn get(&self, pos: IVec2) -> Option<u8> {
        if pos.x < 0 || pos.x >= COLS || pos.y < 0 || pos.y >= ROWS {
            return None;
        }
        self.cells[(pos.y * COLS + pos.x) as usize]
    }

    fn set(&mut self, pos: IVec2, kind: u8) {
        if pos.x >= 0 && pos.x < COLS && pos.y >= 0 && pos.y < ROWS {
            self.cells[(pos.y * COLS + pos.x) as usize] = Some(kind);
        }
    }

    fn blocked(&self, pos: IVec2) -> bool {
        pos.x < 0 || pos.x >= COLS || pos.y < 0 || self.get(pos).is_some()
    }

    /// Removes full rows, shifting everything above down. Returns rows cleared.
    fn clear_lines(&mut self) -> usize {
        let mut kept: Vec<Option<u8>> = Vec::with_capacity(self.cells.len());
        let mut cleared = 0;

        for row in 0..ROWS {
            let start = (row * COLS) as usize;
            let slice = &self.cells[start..start + COLS as usize];
            if slice.iter().all(Option::is_some) {
                cleared += 1;
            } else {
                kept.extend_from_slice(slice);
            }
        }

        kept.resize(self.cells.len(), None);
        self.cells = kept;
        cleared
    }
}

#[derive(Resource, Clone)]
struct Piece {
    kind: u8,
    /// Cell offsets relative to `pos`.
    cells: [IVec2; 4],
    pos: IVec2,
}

const SHAPES: [[(i32, i32); 4]; 7] = [
    [(0, 0), (1, 0), (-1, 0), (2, 0)],  // I
    [(0, 0), (1, 0), (0, 1), (1, 1)],   // O
    [(0, 0), (-1, 0), (1, 0), (0, 1)],  // T
    [(0, 0), (-1, 0), (1, 0), (1, 1)],  // L
    [(0, 0), (-1, 0), (1, 0), (-1, 1)], // J
    [(0, 0), (1, 0), (0, 1), (-1, 1)],  // S
    [(0, 0), (-1, 0), (0, 1), (1, 1)],  // Z
];

impl Piece {
    fn spawn_random() -> Self {
        use rand::Rng;
        let kind = rand::rng().random_range(0..SHAPES.len() as u8);
        Self::of_kind(kind)
    }

    fn of_kind(kind: u8) -> Self {
        let shape = SHAPES[kind as usize];
        Self {
            kind,
            cells: std::array::from_fn(|i| IVec2::new(shape[i].0, shape[i].1)),
            pos: IVec2::new(COLS / 2, ROWS - 2),
        }
    }

    fn world_cells(&self) -> [IVec2; 4] {
        self.cells.map(|cell| cell + self.pos)
    }

    /// Rotation is a 90° turn about the piece origin; O pieces stay put.
    fn rotated(&self) -> Self {
        let mut next = self.clone();
        if next.kind != 1 {
            next.cells = next.cells.map(|c| IVec2::new(-c.y, c.x));
        }
        next
    }

    fn moved(&self, delta: IVec2) -> Self {
        let mut next = self.clone();
        next.pos += delta;
        next
    }

    fn fits(&self, board: &Board) -> bool {
        !self.world_cells().iter().any(|&cell| board.blocked(cell))
    }
}

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct ScoreText;

#[derive(Resource, Deref, DerefMut)]
struct Score(u32);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("Score: 0"),
        Transform::from_xyz(0.0, ROWS as f32 * CELL / 2. + 20., 0.0),
        ScoreText,
    ));
}

fn show_hint(mut commands: Commands) {
    spawn_hint(
        &mut commands,
        "Tetris\nLeft/Right move, Up rotate, Down drop\nPress Space to Start",
    );
}

fn start(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GamePhase>>,
    mut board: ResMut<Board>,
    mut piece: ResMut<Piece>,
    mut score: ResMut<Score>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }
    *board = Board::default();
    *piece = Piece::spawn_random();
    score.0 = 0;
    next_state.set(GamePhase::Playing);
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, board: Res<Board>, mut piece: ResMut<Piece>) {
    let candidate = if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
        piece.moved(IVec2::new(-1, 0))
    } else if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
        piece.moved(IVec2::new(1, 0))
    } else if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
        piece.rotated()
    } else {
        return;
    };

    if candidate.fits(&board) {
        *piece = candidate;
    }
}

fn gravity(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut timer: Local<Timer>,
    mut board: ResMut<Board>,
    mut piece: ResMut<Piece>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(FALL_SECS, TimerMode::Repeating);
    }

    let soft_drop = keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS);
    let interval = if soft_drop { SOFT_DROP_SECS } else { FALL_SECS };
    if timer.duration().as_secs_f32() != interval {
        timer.set_duration(std::time::Duration::from_secs_f32(interval));
    }

    if !timer.tick(time.delta()).just_finished() {
        return;
    }

    let dropped = piece.moved(IVec2::new(0, -1));
    if dropped.fits(&board) {
        *piece = dropped;
        return;
    }

    // landed: bake into the board, score any cleared lines, spawn the next piece
    for cell in piece.world_cells() {
        board.set(cell, piece.kind);
    }
    score.0 += match board.clear_lines() {
        0 => 0,
        1 => 100,
        2 => 300,
        3 => 500,
        _ => 800,
    };

    let next = Piece::spawn_random();
    if next.fits(&board) {
        *piece = next;
    } else {
        next_state.set(GamePhase::GameOver);
    }
}

fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<Board>,
    piece: Res<Piece>,
    existing: Query<Entity, With<Cell>>,
) {
    for entity in &existing {
        commands.entity(entity).despawn();
    }

    let mesh = meshes.add(Rectangle::new(CELL - 2., CELL - 2.));
    let mut draw = |pos: IVec2, kind: u8| {
        let color = kind_color(kind);
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_translation(grid_to_world(pos)),
            Cell,
        ));
    };

    for y in 0..ROWS {
        for x in 0..COLS {
            let pos = IVec2::new(x, y);
            if let Some(kind) = board.get(pos) {
                draw(pos, kind);
            }
        }
    }
    for cell in piece.world_cells() {
        draw(cell, piece.kind);
    }
}

fn kind_color(kind: u8) -> Color {
    match kind {
        0 => Color::srgb_u8(125, 196, 228),
        1 => Color::srgb_u8(238, 212, 159),
        2 => Color::srgb_u8(198, 160, 246),
        3 => Color::srgb_u8(245, 169, 127),
        4 => Color::srgb_u8(138, 173, 244),
        5 => Color::srgb_u8(166, 218, 149),
        _ => Color::srgb_u8(237, 135, 150),
    }
}

fn grid_to_world(pos: IVec2) -> Vec3 {
    Vec3::new(
        (pos.x as f32 - COLS as f32 / 2.) * CELL + CELL / 2.,
        (pos.y as f32 - ROWS as f32 / 2.) * CELL + CELL / 2.,
        0.,
    )
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text2d, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            text.0 = format!("Score: {}", score.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_lines_removes_full_rows_and_shifts_down() {
        let mut board = Board::default();
        for x in 0..COLS {
            board.set(IVec2::new(x, 0), 1);
        }
        board.set(IVec2::new(3, 1), 2);

        assert_eq!(board.clear_lines(), 1);
        assert_eq!(board.get(IVec2::new(3, 0)), Some(2), "row above shifts down");
        assert_eq!(board.get(IVec2::new(3, 1)), None);
        assert!(board.cells.iter().filter(|c| c.is_some()).count() == 1);
    }

    #[test]
    fn pieces_cannot_leave_the_well_or_overlap() {
        let mut board = Board::default();
        let piece = Piece::of_kind(1); // O

        assert!(piece.fits(&board));
        assert!(!piece.moved(IVec2::new(-COLS, 0)).fits(&board), "left wall");
        assert!(!piece.moved(IVec2::new(COLS, 0)).fits(&board), "right wall");
        assert!(!piece.moved(IVec2::new(0, -ROWS)).fits(&board), "floor");

        for cell in piece.world_cells() {
            board.set(cell, 1);
        }
        assert!(!piece.fits(&board), "overlapping landed cells");
    }

    #[test]
    fn rotation_is_four_fold_and_o_is_fixed() {
        let t = Piece::of_kind(2);
        let spun = t.rotated().rotated().rotated().rotated();
        assert_eq!(spun.cells, t.cells);

        let o = Piece::of_kind(1);
        assert_eq!(o.rotated().cells, o.cells);
    }
}
