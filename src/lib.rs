pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use crate::components::coordinates::Coordinate;
use crate::resources::{
    Player,
    game_resource::{ Board, CurrentTurn }
};

const CELL_SIZE: f32 = 120.0;
const CELL_PADDING: f32 = 10.0;
const BOARD_DIMENSION: f32 = (BOARD_SIZE as f32 * CELL_SIZE) + ((BOARD_SIZE - 1) as f32 * CELL_PADDING);

pub const FONT_SIZE_CELL: f32 = 100.0;
pub const FONT_SIZE_MESSAGE: f32 = 60.0;
pub const BOARD_SIZE: usize = 3;
const COLOR_BOARD: Srgba = Srgba::rgb(0.2, 0.2, 0.2);
pub const COLOR_X: Srgba = Srgba::rgb(0.9, 0.2, 0.2);
pub const COLOR_O: Srgba = Srgba::rgb(0.2, 0.2, 0.9);
pub const COLOR_CELL: Srgba = Srgba::rgb(0.4, 0.4, 0.4);
pub const COLOR_CELL_HOVER: Srgba = Srgba::rgb(0.5, 0.5, 0.5);
pub const COLOR_MESSAGE: Color = Color::WHITE;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

pub fn setup_game(mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
    println!("start setup game.");
    // 盤面とターンをリソースとして初期化
    commands.insert_resource(Board::default());
    commands.insert_resource(CurrentTurn(Player::X));

    println!("initialize...");

    // 盤面の親Node
    commands
        .spawn((
            Node {
                width: Val::Px(BOARD_DIMENSION),
                height: Val::Px(BOARD_DIMENSION),
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                // background_color: COLOR_BOARD.into(),
                ..default()
            },
            BackgroundColor (COLOR_BOARD.into())
        ))
        .with_children(|parent| {
            for row in 0..BOARD_SIZE {
                parent
                    .spawn(Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(CELL_SIZE),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    })
                    .with_children(|parent_row| {
                        for col in 0..BOARD_SIZE {
                            parent_row
                                .spawn((
                                    Node {
                                        width: Val::Px(CELL_SIZE),
                                        height: Val::Px(CELL_SIZE),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        //background_color: COLOR_CELL.into(),
                                        ..default()
                                    },
                                    Button,
                                    Interaction::None,
                                    BackgroundColor (COLOR_CELL.into()),
                                    Coordinate { row, col },
                                ));
                        }
                    });
            }
        });

    next_state.set(GameState::Playing);

        println!("setup finished.");
}
