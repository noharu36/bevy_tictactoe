use bevy::prelude::*;
use crate::resources::Player;
use crate::BOARD_SIZE;


#[derive(Debug, Resource)]
pub struct Board {
    pub grid: [[Player; BOARD_SIZE]; BOARD_SIZE],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[Player::None; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

#[derive(Debug, Resource)]
pub struct CurrentTurn(pub Player);

#[derive(Debug, Resource)]
pub enum GameResult {
    Winner(Player),
    Draw,
}
