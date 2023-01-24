use reversi as core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    fn parse(&self) -> core::Player {
        match self {
            Player::Player1 => core::Player::Player1,
            Player::Player2 => core::Player::Player2,
        }
    }
}

#[derive(Debug, PartialEq)]
#[wasm_bindgen]
pub enum State {
    Empty,
    Player1,
    Player2,
}

impl State {
    fn new(s: core::State) -> Self {
        match s {
            core::State::Player(core::Player::Player1) => State::Player1,
            core::State::Player(core::Player::Player2) => State::Player2,
            core::State::Empty => State::Empty,
        }
    }
}

#[wasm_bindgen]
pub struct Reversi {
    board: core::Board,
}

#[wasm_bindgen]
impl Reversi {
    pub fn new_game() -> Self {
        Reversi {
            board: core::Board::new(),
        }
    }

    fn new(board: core::Board) -> Self {
        Self { board }
    }

    pub fn x(&self) -> usize {
        self.board.x
    }

    pub fn y(&self) -> usize {
        self.board.y
    }

    pub fn is_act(&self, p: Player, x: i32, y: i32) -> bool {
        self.board.is_act(p.parse(), core::Point::new(x, y))
    }

    pub fn state(&self, x: i32, y: i32) -> Option<State> {
        match self.board.state(core::Point::new(x, y)) {
            Some(s) => Some(State::new(s)),
            None => None,
        }
    }

    pub fn action(&self, p: Player, x: i32, y: i32) -> Option<Reversi> {
        match self.board.action(p.parse(), core::Point::new(x, y)) {
            Ok(b) => Some(Self::new(b)),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reversi_x() {
        let game = Reversi::new_game();
        assert_eq!(game.x(), 8);
    }

    #[test]
    fn reversi_y() {
        let game = Reversi::new_game();
        assert_eq!(game.y(), 8);
    }

    #[test]
    fn reversi_is_act() {
        let game = Reversi::new_game();
        assert_eq!(game.is_act(Player::Player1, 2, 3), false);
        assert_eq!(game.is_act(Player::Player1, 2, 4), true);
        assert_eq!(game.is_act(Player::Player2, 2, 3), true);
        assert_eq!(game.is_act(Player::Player2, 2, 4), false);
    }

    #[test]
    fn reversi_state() {
        let game = Reversi::new_game();
        assert_eq!(game.state(2, 3), Some(State::Empty));
        assert_eq!(game.state(2, 4), Some(State::Player1));
        assert_eq!(game.state(2, 5), Some(State::Player2));
    }
}
