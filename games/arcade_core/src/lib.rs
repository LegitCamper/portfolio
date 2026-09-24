//! Shared bits every arcade game needs: canvas setup, a `Ready -> Playing -> GameOver`
//! lifecycle, and a pause that freezes gameplay whenever the canvas loses focus.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Lifecycle shared by every game in the arcade.
#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default)]
pub enum GamePhase {
    /// Waiting for Space.
    #[default]
    Ready,
    Playing,
    GameOver,
}

#[derive(Component)]
struct PauseOverlay;

/// Sets up `DefaultPlugins` bound to a canvas, the [`GamePhase`] state, and the pause overlay.
pub struct ArcadePlugin {
    /// CSS selector of the canvas, e.g. `"#snake-canvas"`.
    pub canvas: &'static str,
}

impl Plugin for ArcadePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(self.canvas.into()),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GamePhase>()
        .add_systems(Update, toggle_pause_overlay);
    }
}

/// Run condition: true while the game window has focus. Gate gameplay systems on this so a
/// backgrounded game neither reads input nor advances its timers (no catch-up burst on return).
pub fn focused(windows: Query<&Window, With<PrimaryWindow>>) -> bool {
    windows.single().is_ok_and(|window| window.focused)
}

fn toggle_pause_overlay(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    overlay: Query<Entity, With<PauseOverlay>>,
    phase: Res<State<GamePhase>>,
) {
    let has_focus = windows.single().is_ok_and(|window| window.focused);
    let show = matches!(*phase.get(), GamePhase::Playing) && !has_focus;

    match (show, overlay.single()) {
        (true, Err(_)) => {
            commands.spawn((
                Text2d::new("Paused\nclick to resume"),
                TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                Transform::from_xyz(0.0, 0.0, 10.0),
                PauseOverlay,
            ));
        }
        (false, Ok(entity)) => commands.entity(entity).despawn(),
        _ => {}
    }
}

/// Spawns the 2d camera plus the "Press Space to Start" text shown in [`GamePhase::Ready`].
pub fn spawn_hint(commands: &mut Commands, text: &str) {
    commands.spawn((
        Text2d::new(text.to_string()),
        TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
        Transform::from_xyz(0.0, 0.0, 1.0),
        HintText,
    ));
}

#[derive(Component)]
pub struct HintText;

/// Despawns anything spawned by [`spawn_hint`].
pub fn despawn_hint(mut commands: Commands, hints: Query<Entity, With<HintText>>) {
    for entity in &hints {
        commands.entity(entity).despawn();
    }
}
