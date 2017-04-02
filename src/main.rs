extern crate rand;
mod board;

use std::process::Command;
use std::io;

use board::Board;


fn clear()
{
    if !cfg!(target_os = "windows") {
        let output = Command::new("clear")
                    .output()
                    .expect("failure to clear window");
    }
}

fn handle_input(input: &str, board: &mut Board) -> bool {
    println!("{}", input);
    match input.to_uppercase().as_ref() {
        "W" => {
            board.move_up();
            true
        },
        "A" => {
            board.move_left();
            true
        },
        "S" => {
            board.move_down();
            true
        },
        "D" => {
            board.move_right();
            true
        },
        _  => {
            println!("Enter either W, A, S or D you dummy");
            false
        }
    }
}


fn main() {
    let mut game_board = Board::new();
    println!("How to play:
This is a pretty basic simulation of the popular mobile game, Threes. Enter W, A, S or D to move tiles up, left, down or right respectively.

It's probably best to play this in a fresh terminal, or one that you don't mind having repeatedly wiped.

Rules:
- Making a move moves the whole board in that direction if possible
- A 1 tile can collide with a 2 tile to make a 3 tile
- Tiles with the same value join together to make a new tile with double the values, e.g. a collision between two 6 tiles makes one 12 tile
- A new semi-random value will then join the board

Have fun!
");

    while game_board.has_moves() {
        game_board.print();
        let mut valid_input = false;
        while !valid_input {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line, something is bad");
            valid_input = handle_input(&input.trim(), &mut game_board);
        }
        clear();
    }
}
