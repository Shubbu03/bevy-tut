use bevy::prelude::*;

mod components;
mod states;
mod systems;

use states::GameState;
use systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Tut".into(),
                        resolution: (640, 480).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (character_movement, check_rock_collision, update_health_bar)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), show_game_over)
        .add_systems(
            Update,
            handle_game_over_buttons.run_if(in_state(GameState::GameOver)),
        )
        .run();
}
