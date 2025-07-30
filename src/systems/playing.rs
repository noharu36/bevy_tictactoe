use bevy::prelude::*;
use crate::resources::{
    Player,
    game_resource::{ Board, CurrentTurn, GameResult }
};
use crate::components::coordinates::Coordinate;
use crate::{
    GameState,
    FONT_SIZE_CELL,
    COLOR_X,
    COLOR_O,
    BOARD_SIZE,
    COLOR_CELL,
    COLOR_CELL_HOVER
};

pub fn handle_cell_click(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &Coordinate, Entity), (Changed<Interaction>, With<Button>)>,
    mut board: ResMut<Board>,
    mut current_turn: ResMut<CurrentTurn>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, position, entity) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("clicked");
            // もしセルが空なら
            if board.grid[position.row][position.col] == Player::None {
                // 盤面の状態を更新
                board.grid[position.row][position.col] = current_turn.0;

                // セルに "X" か "O" のテキストを表示
                commands.entity(entity).with_children(|parent| {
                    parent.spawn((
                            Text::new(current_turn.0.to_string()),
                            TextFont {font_size: FONT_SIZE_CELL, ..Default::default()},
                            TextColor (if current_turn.0 == Player::X { COLOR_X } else { COLOR_O }.into())
                    ));
                });

                // 勝利判定
                if let Some(winner) = check_for_winner(&board) {
                    commands.insert_resource(GameResult::Winner(winner));
                    next_state.set(GameState::GameOver);
                    return;
                }

                // 引き分け判定
                if is_draw(&board) {
                    commands.insert_resource(GameResult::Draw);
                    next_state.set(GameState::GameOver);
                    return;
                }

                // ターンを交代
                current_turn.0 = current_turn.0.next();
            }
        }
    }
}

fn check_for_winner(board: &Board) -> Option<Player> {
    // 行をチェック
    for row in 0..BOARD_SIZE {
        if let p @ (Player::X | Player::O) = board.grid[row][0] {
            if (1..BOARD_SIZE).all(|col| board.grid[row][col] == p) {
                return Some(p);
            }
        }
    }

    // 列をチェック
    for col in 0..BOARD_SIZE {
        if let p @ (Player::X | Player::O) = board.grid[0][col] {
            if (1..BOARD_SIZE).all(|row| board.grid[row][col] == p) {
                return Some(p);
            }
        }
    }

    // 対角線（左上から右下）をチェック
    if let p @ (Player::X | Player::O) = board.grid[0][0] {
        if (1..BOARD_SIZE).all(|i| board.grid[i][i] == p) {
            return Some(p);
        }
    }
    
    // 対角線（右上から左下）をチェック
    if let p @ (Player::X | Player::O) = board.grid[0][BOARD_SIZE - 1] {
        if (1..BOARD_SIZE).all(|i| board.grid[i][BOARD_SIZE - 1 - i] == p) {
            return Some(p);
        }
    }

    None
}

fn is_draw(board: &Board) -> bool {
    board.grid.iter().all(|row| row.iter().all(|&cell| cell != Player::None))
}

pub fn update_cell_visuals(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => *color = COLOR_CELL_HOVER.into(),
            _ => *color = COLOR_CELL.into(),
        }
    }
}
