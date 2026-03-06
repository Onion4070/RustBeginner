use proconio::input;
use std::io::{self, Write};

fn main() {
    let mut black = 0;
    let mut white = 0;
    let mut total = 0;
    let mut turn  = 1;

    let mut board = [[0; 3]; 3];

    while total < 9 {
        display(&board);
        print!("> ");
        io::stdout().flush().unwrap();
        
        input! {
            y: usize, 
            x: usize
        }

        println!();
        
        if board[y][x] != 0
        || x > 3 || y > 3 {
            println!("Invalid :(");
            continue;
        }

        board[y][x] = turn;

        if turn == 1 {black += 1} else {white += 1}
        total = black + white;

        if judge(&board) {
            break;
        }

        turn *= -1;
    }

    display(&board);
    println!("Game!");
}

fn display(board: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            let mut symbol = '-';
            if board[i][j] ==  1 {symbol = 'o';}
            if board[i][j] == -1 {symbol = 'x';}
            print!("{} ", symbol);
        }
        println!();
    }
}

fn judge(board: &[[i32; 3]; 3]) -> bool {
    // 横
    for i in 0..3 {
        if board[i][0] != 0
        && board[i][0] == board[i][1]
        && board[i][1] == board[i][2] {
            return true;
        }
    }

    // 縦
    for i in 0..3 {
        if board[0][i] != 0
        && board[0][i] == board[1][i]
        && board[1][i] == board[2][i] {
            return true;
        }
    }

    // 右下
    if board[0][0] != 0
    && board[0][0] == board[1][1]
    && board[1][1] == board[2][2] {
        return true;
    }

    // 左下
    if board[0][2] != 0
    && board[0][2] == board[1][1]
    && board[1][1] == board[2][0] {
        return true;
    }

    false
}