extern crate getopts;
extern crate rand;
extern crate sdl2;

mod board;
mod gui;

use std::io;
use std::env;

use getopts::Options;

use board::Board;
use gui::new_game;

fn handle_input(input: &str, board: &mut Board) -> bool {
    match input.to_uppercase().as_ref() {
        "W" => {
            board.move_up();
            true
        }
        "A" => {
            board.move_left();
            true
        }
        "S" => {
            board.move_down();
            true
        }
        "D" => {
            board.move_right();
            true
        }
        _ => {
            println!("Enter either W, A, S or D you dummy");
            false
        }
    }
}

fn print_help(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
    println!(
        "How to play:
This is a pretty basic simulation of the popular mobile game, Threes.
Enter W, A, S or D to move tiles up, left, down or right respectively.

Rules:
- Making a move moves the whole board in that direction if possible
- A 1 tile can collide with a 2 tile to make a 3 tile
- Tiles with the same value join together to make a new tile with double the values
    , e.g. a collision between two 6 tiles makes one 12 tile
- A new semi-random value will then join the board

Have fun!
"
    );
}

fn terminal_game() {
    let mut game_board = Board::new();
    while game_board.has_moves() {
        game_board.print();
        println!("Next card: {}", game_board.get_next_card());
        let mut valid_input = false;
        while !valid_input {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect(
                "Failed to read line, something is bad",
            );
            valid_input = handle_input(input.trim(), &mut game_board);
        }
    }
    game_board.print();
    println!("Game over!");
    println!("You scored: {}", game_board.calculate_score());
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag(
        "t",
        "terminal",
        "run as a playable version in the terminal. It's probably best to play this\
in a fresh terminal, or one that you don't mind having repeatedly wiped.",
    );
    opts.optflag("h", "help", "print the help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_help(&program, &opts);
        return;
    }

    if matches.opt_present("t") {
        terminal_game();
    } else {
        new_game();
    }
}
