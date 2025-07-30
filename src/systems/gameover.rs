use bevy::prelude::*;
use crate::resources::game_resource::GameResult;
use crate::components::GameOverScreen;
use crate::{
    GameState,
    FONT_SIZE_MESSAGE,
    COLOR_MESSAGE
};

pub fn setup_game_over_screen(
    mut commands: Commands,
    game_result: Res<GameResult>,
) {
    let message = match *game_result {
        GameResult::Winner(player) => format!("{} wins!", player),
        GameResult::Draw => "draw...".to_string(),
    };
    
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            // background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..default()
        },
        GameOverScreen,
    )).with_children(|parent| {
        // 結果メッセージ
            parent.spawn((
                    Text::new(message),
                    TextFont {font_size: FONT_SIZE_MESSAGE, ..Default::default()},
                    TextColor (COLOR_MESSAGE)
            ));
        // リスタートメッセージ
            parent.spawn((
                    Text::new("\nClick to Restart!"),
                    TextFont {font_size: FONT_SIZE_MESSAGE / 2.0, ..Default::default()},
                    TextColor (COLOR_MESSAGE)
            ));
    });
}

pub fn handle_restart(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    query: Query<Entity, With<Node>>, // 盤面のUI要素を取得
    text_query: Query<Entity, With<Text>>, // セルのテキスト要素を取得
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // 古い盤面とセル内のテキストを削除
        for entity in query.iter().chain(text_query.iter()) {
            commands.entity(entity).despawn();
        }
        
        // ゲームをPlaying状態に戻す
        next_state.set(GameState::Playing);
    }
}

pub fn cleanup_game_over_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOverScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
