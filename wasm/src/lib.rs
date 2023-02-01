use reversi as core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    fn new(p: core::Player) -> Self {
        match p {
            core::Player::Player1 => Self::Player1,
            core::Player::Player2 => Self::Player2,
        }
    }

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
#[derive(Debug, PartialEq)]
pub struct Reversi {
    board: core::Board,
    player: core::Player,
}

#[wasm_bindgen]
impl Reversi {
    pub fn new_game() -> Self {
        Reversi {
            board: core::Board::new(),
            player: core::Player::Player1,
        }
    }

    fn new(board: core::Board, player: core::Player) -> Self {
        Self { board, player }
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

    pub fn current_player(&self) -> Player {
        Player::new(self.player)
    }

    pub fn action(&self, p: Player, x: i32, y: i32) -> Option<Reversi> {
        let p = p.parse();
        match self.board.action(p, core::Point::new(x, y)) {
            Ok(b) => Some(Self::new(b, p.switch())),
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
        assert_eq!(game.is_act(Player::Player2, 2, 3), false);
        assert_eq!(game.is_act(Player::Player2, 2, 4), true);
        assert_eq!(game.is_act(Player::Player1, 2, 3), true);
        assert_eq!(game.is_act(Player::Player1, 2, 4), false);
    }

    #[test]
    fn reversi_state() {
        let game = Reversi::new_game();
        assert_eq!(game.state(3, 2), Some(State::Empty));
        assert_eq!(game.state(3, 3), Some(State::Player2));
        assert_eq!(game.state(3, 4), Some(State::Player1));
    }

    #[test]
    fn reversi_current_player() {
        let game = Reversi::new_game();

        assert_eq!(game.current_player(), Player::Player1);
    }

    #[test]
    fn reversi_action() {
        let game = Reversi::new_game();

        assert_eq!(
            game.action(game.current_player(), 3, 2),
            Some(Reversi::new(
                core::Board::new()
                    .action(core::Player::Player1, core::Point::new(3, 2))
                    .unwrap(),
                core::Player::Player2,
            ))
        );
    }
}
