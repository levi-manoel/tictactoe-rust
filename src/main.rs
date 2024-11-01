use std::io;

const BOARD_SIZE: usize = 3;

fn main() {
    loop {
        let mut board = [[" "; BOARD_SIZE]; BOARD_SIZE];
        let mut current_player = "X";

        loop {
            display_board(&board);
            println!(
                "Player {}'s turn. Enter your move (row and column, e.g., 1 2):",
                current_player
            );

            let (row, col) = match get_player_move() {
                Some((r, c)) if r < BOARD_SIZE && c < BOARD_SIZE && board[r][c] == " " => (r, c),
                _ => {
                    println!("Invalid move. Try again.");
                    continue;
                }
            };

            board[row][col] = current_player;

            if check_win(&board, current_player) {
                display_board(&board);
                println!("Player {} wins!", current_player);
                break;
            }

            if check_draw(&board) {
                display_board(&board);
                println!("It's a draw!");
                break;
            }

            current_player = if current_player == "X" { "O" } else { "X" };
        }

        if !play_again() {
            break;
        }
    }
}

fn display_board(board: &[[&str; BOARD_SIZE]; BOARD_SIZE]) {
    for (i, row) in board.iter().enumerate() {
        println!("| {} | {} | {} |", row[0], row[1], row[2]);
        if i < BOARD_SIZE - 1 {
            println!("-------------");
        }
    }
}

fn get_player_move() -> Option<(usize, usize)> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let coords: Vec<usize> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if coords.len() == 2 {
        Some((coords[0] - 1, coords[1] - 1))
    } else {
        None
    }
}

fn check_win(board: &[[&str; BOARD_SIZE]; BOARD_SIZE], player: &str) -> bool {
    for row in board {
        if row.iter().all(|&cell| cell == player) {
            return true;
        }
    }

    for col in 0..BOARD_SIZE {
        if (0..BOARD_SIZE).all(|row| board[row][col] == player) {
            return true;
        }
    }

    if (0..BOARD_SIZE).all(|i| board[i][i] == player)
        || (0..BOARD_SIZE).all(|i| board[i][BOARD_SIZE - i - 1] == player)
    {
        return true;
    }

    false
}

fn check_draw(board: &[[&str; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    board.iter().all(|row| row.iter().all(|&cell| cell != " "))
}

fn play_again() -> bool {
    println!("Play again? (y[es] or n)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}
