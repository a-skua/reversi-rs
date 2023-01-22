use reversi::*;

fn show(table: Vec<&[Status]>) {
    println!(" ________________ ");
    for row in table {
        print!("|");
        for col in row {
            match col {
                Status::Player(Player::Player1) => print!("o "),
                Status::Player(Player::Player2) => print!("x "),
                Status::Empty => print!("  "),
            }
        }
        println!("|");
    }
    println!(" ```````````````` ");
}

fn main() {
    let mut board = Board::new();
    let mut player = Player::Player1;

    let actions = [
        Point::new(4, 2),
        Point::new(5, 2),
        Point::new(5, 3),
        Point::new(3, 2),
    ];

    show(board.table());
    for p in actions {
        board = board.action(player, p).unwrap();
        player = player.switch();
        show(board.table());
    }
}
