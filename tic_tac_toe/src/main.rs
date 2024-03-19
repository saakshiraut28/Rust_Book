use std::io;

fn print_board(board: &[[char; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("'{}' ", board[i][j]);
        }
        println!("\n")
    }
}

fn next_player(player: char) -> char {
    if player == 'X' {
        return '0';
    } else {
        return 'X';
    }
}

fn is_win(board: &[[char; 3]; 3], player: char) -> bool {
    // Check rows
    for i in 0..3 {
        let mut win = true;
        for j in 0..3 {
            if board[i][j] != player {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }
    // Check columns
    for j in 0..3 {
        let mut win = true;
        for i in 0..3 {
            if board[i][j] != player {
                win = false;
                break;
            }
        }
        if win {
            return true;
        }
    }

    // Check diagonals
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player)
        || (board[0][2] == player && board[1][1] == player && board[2][0] == player)
    {
        return true;
    }

    false
}

fn is_board_filled(board: &[[char; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == '-' {
                return false;
            }
        }
    }
    true
}

fn main() {
    println!("Tic Tac Toe Game");
    let mut board = [['-'; 3]; 3];
    let mut player = 'X';
    print_board(&board);
    loop {
        println!("{player}'s turn: ");
        println!("Enter the row and col: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input.");
        let mut coordinates = input.split_whitespace();
        let mut row: usize = coordinates.next().unwrap().parse().expect("Enter Integer.");
        row = row - 1;
        let mut col: usize = coordinates
            .next()
            .unwrap()
            .parse()
            .expect("Enter valid col");
        col = col - 1;
        // Check whether the given row or col is empty:
        if board[row][col] == '-' {
            board[row][col] = player;
        } else {
            println!("Overwriting the grid is not allowed.");
            break;
        }
        // Check wheter the player wins;
        if is_win(&board, player) {
            println!("{player}'x wins.");
            break;
        }
        if is_board_filled(&board) {
            println!("It's a draw");
            break;
        }
        print_board(&board);
        player = next_player(player);
    }
}
