#[derive(Debug, Clone, Copy)]
enum Vector {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Vector {
    fn values() -> Vec<Self> {
        vec![
            Self::TopLeft,
            Self::Top,
            Self::TopRight,
            Self::Right,
            Self::BottomRight,
            Self::Bottom,
            Self::BottomLeft,
            Self::Left,
        ]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn move_point(&self, vector: Vector) -> Self {
        match vector {
            Vector::TopLeft => Self::new(self.x - 1, self.y - 1),
            Vector::Top => Self::new(self.x, self.y - 1),
            Vector::TopRight => Self::new(self.x + 1, self.y - 1),
            Vector::Right => Self::new(self.x + 1, self.y),
            Vector::BottomRight => Self::new(self.x + 1, self.y + 1),
            Vector::Bottom => Self::new(self.x, self.y + 1),
            Vector::BottomLeft => Self::new(self.x - 1, self.y + 1),
            Vector::Left => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    pub fn switch(self) -> Self {
        match self {
            Self::Player1 => Self::Player2,
            Self::Player2 => Self::Player1,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    Empty,
    Player(Player),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pub x: usize,
    pub y: usize,
    source: Vec<State>,
}

impl Board {
    pub fn new() -> Self {
        let x = 8;
        let y = 8;
        let mut source = Vec::with_capacity(x * y);
        for i in 0..x * y {
            match i {
                27 | 36 => source.push(State::Player(Player::Player1)),
                28 | 35 => source.push(State::Player(Player::Player2)),
                _ => source.push(State::Empty),
            }
        }
        Self { x, y, source }
    }

    pub fn table(&self) -> Vec<&[State]> {
        let mut table = Vec::with_capacity(self.y);
        for i in 0..self.y {
            let start = i * self.x;
            let end = start + self.x;
            table.push(&self.source[start..end]);
        }
        table
    }

    fn index(&self, p: Point) -> usize {
        p.y as usize * self.y + p.x as usize
    }

    pub fn state(&self, p: Point) -> Option<State> {
        if !self.is_on_board(p) {
            return None;
        }

        Some(self.source[self.index(p)])
    }

    fn is_on_board(&self, p: Point) -> bool {
        p.x >= 0 && p.x < self.x as i32 && p.y >= 0 && p.y < self.y as i32
    }

    fn is_act_vector(&self, player: Player, p: Point, v: Vector) -> bool {
        match self.state(p) {
            Some(State::Empty) => {}
            _ => return false,
        }

        let mut p = p;
        let mut is_possible = false;
        loop {
            p = p.move_point(v);
            if !self.is_on_board(p) {
                return false;
            }

            let state = if let Some(s) = self.state(p) {
                s
            } else {
                return false;
            };

            match state {
                State::Player(p) => {
                    if p == player {
                        return is_possible;
                    } else {
                        is_possible = true;
                    }
                }
                _ => return false,
            }
        }
    }

    pub fn is_act(&self, player: Player, p: Point) -> bool {
        let mut is_act = false;
        for v in Vector::values() {
            is_act |= self.is_act_vector(player, p, v);
        }

        is_act
    }

    pub fn actionable(&self, player: Player) -> Vec<Point> {
        let mut actionable = vec![];
        for y in 0..self.y {
            for x in 0..self.x {
                let p = Point::new(x as i32, y as i32);
                if self.is_act(player, p) {
                    actionable.push(p);
                }
            }
        }

        actionable
    }

    fn action_vector(mut self, player: Player, p: Point, v: Vector) -> Self {
        if !self.is_act_vector(player, p, v) {
            return self;
        }

        let mut p = p;
        loop {
            p = p.move_point(v);
            let state = if let Some(s) = self.state(p) {
                s
            } else {
                return self;
            };

            match state {
                State::Player(target) => {
                    if target == player {
                        return self;
                    } else {
                        let i = self.index(p);
                        self.source[i] = State::Player(player);
                    }
                }
                _ => return self,
            }
        }
    }

    pub fn action(&self, player: Player, p: Point) -> Result<Self, &'static str> {
        let is_act = self.is_act(player, p);
        if !is_act {
            return Result::Err("failed action");
        }

        let mut new = self.clone();

        for v in Vector::values() {
            new = new.action_vector(player, p, v);
        }

        new.source[self.index(p)] = State::Player(player);
        Result::Ok(new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_point() {
        let p = Point::new(0, 0);
        assert_eq!(p.move_point(Vector::TopLeft), Point::new(-1, -1));
        assert_eq!(p.move_point(Vector::Top), Point::new(0, -1));
        assert_eq!(p.move_point(Vector::TopRight), Point::new(1, -1));
        assert_eq!(p.move_point(Vector::Right), Point::new(1, 0));
        assert_eq!(p.move_point(Vector::BottomRight), Point::new(1, 1));
        assert_eq!(p.move_point(Vector::Bottom), Point::new(0, 1));
        assert_eq!(p.move_point(Vector::BottomLeft), Point::new(-1, 1));
        assert_eq!(p.move_point(Vector::Left), Point::new(-1, 0));
    }

    #[test]
    fn board_table() {
        let source = [
            // y = 0
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            // y = 1
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            // y = 2
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            // y = 3
            State::Empty,
            State::Empty,
            State::Empty,
            State::Player(Player::Player1),
            State::Player(Player::Player2),
            State::Empty,
            State::Empty,
            State::Empty,
            // y = 4
            State::Empty,
            State::Empty,
            State::Empty,
            State::Player(Player::Player2),
            State::Player(Player::Player1),
            State::Empty,
            State::Empty,
            State::Empty,
            // y = 5
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            // y = 6
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            // y = 7
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
            State::Empty,
        ];

        let expect = vec![
            &source[0..8],
            &source[8..16],
            &source[16..24],
            &source[24..32],
            &source[32..40],
            &source[40..48],
            &source[48..56],
            &source[56..64],
        ];

        let board = Board::new();
        let actual = board.table();

        assert_eq!(actual.len(), expect.len());
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(actual[y][x], expect[y][x]);
            }
        }
    }

    #[test]
    fn actionable_player2() {
        let board = Board::new();

        let actual = board.actionable(Player::Player2);
        let expect = vec![
            Point::new(3, 2),
            Point::new(2, 3),
            Point::new(5, 4),
            Point::new(4, 5),
        ];

        assert_eq!(actual.len(), expect.len());
        for i in 0..expect.len() {
            assert_eq!(actual[i], expect[i]);
        }
    }

    #[test]
    fn is_act_vector_player2() {
        let board = Board::new();

        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::TopLeft),
            false
        );
        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::Top),
            false
        );
        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::TopRight),
            false
        );
        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::Right),
            false
        );
        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::BottomRight),
            false
        );
        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::Bottom),
            true
        );
        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::BottomLeft),
            false
        );
        assert_eq!(
            board.is_act_vector(Player::Player2, Point::new(3, 2), Vector::Left),
            false
        );
    }

    #[test]
    fn is_act_player2() {
        let board = Board::new();

        assert_eq!(board.is_act(Player::Player2, Point::new(2, 3)), true);
        assert_eq!(board.is_act(Player::Player2, Point::new(2, 2)), false);
        assert_eq!(board.is_act(Player::Player2, Point::new(3, 2)), true);
        assert_eq!(board.is_act(Player::Player2, Point::new(3, 3)), false);
    }

    #[test]
    fn action_player2() {
        let board = Board::new();
        assert_eq!(
            board.action(Player::Player2, Point::new(2, 3)),
            Result::Ok(Board {
                x: 8,
                y: 8,
                source: vec![
                    // y = 0
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    // y = 1
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    // y = 2
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    // y = 3
                    State::Empty,
                    State::Empty,
                    State::Player(Player::Player2),
                    State::Player(Player::Player2),
                    State::Player(Player::Player2),
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    // y = 4
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Player(Player::Player2),
                    State::Player(Player::Player1),
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    // y = 5
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    // y = 6
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    // y = 7
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                    State::Empty,
                ],
            }),
        );
    }
}
