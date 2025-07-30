pub mod game_resource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
    None
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
            Player::None => write!(f, "None")
        }
    }
}

impl Player {
    pub fn next(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::None => Player::None,
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Player::None => true,
            _ => false
        }
    }
}

