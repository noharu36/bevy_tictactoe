use bevy::prelude::*;
use tictactoe::{
    GameState,
    setup_game,
    systems::{
        playing::{handle_cell_click, update_cell_visuals},
        gameover::{setup_game_over_screen, handle_restart, cleanup_game_over_screen},
    }
};

#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // 初期化処理
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::Playing), setup_game)
            // 毎フレーム実行されるシステム
            .add_systems(Update, (
                handle_cell_click.run_if(in_state(GameState::Playing)),
                update_cell_visuals.run_if(in_state(GameState::Playing)),
            ))
            // ゲームオーバー時の処理
            .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            // ゲームオーバー画面での入力処理
            .add_systems(Update, handle_restart.run_if(in_state(GameState::GameOver)))
            // ゲーム終了時のクリーンアップ
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over_screen);
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "tictactoe".to_string(),
            resolution: (700., 800.).into(),
            resizable : false,
            ..Default::default()
        }),
        ..default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Startup, setup_camera);
    app.add_plugins(GamePlugin);

    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
