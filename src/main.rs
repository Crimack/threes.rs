extern crate rand;

use rand::distributions::{IndependentSample, Range};

use std::process::Command;
use std::io;


#[derive(Debug)]
struct Board {
    state: [[u32; 4]; 4]
}

impl Board {
    fn new() -> Board {
        let mut starting_state: [[u32;4];4] = [
                                                [0; 4],
                                                [0; 4],
                                                [0; 4],
                                                [0; 4]
                                             ];
        let between = Range::new(0, 4);
        let mut rng = rand::thread_rng();

        // The starting board starts with 3 1s, 3 2s and 3 3s at seemingly random places
        for tile_type in 1..4 {
            for tile_number in 1..4 {
                let mut valid_place: bool = false;
                while !valid_place {
                    let x = between.ind_sample(&mut rng);
                    let y = between.ind_sample(&mut rng);
                    if starting_state[x][y] == 0 {
                        starting_state[x][y] = tile_type;
                        valid_place = true;
                    }
                }
            }
        }
        Board { state: starting_state } 
    }

    fn has_moves(&self) -> bool {
        true
    }

    fn print(&self) {
        for row in &self.state {
            print!("\n");
            for num in row {
                if *num == 0 {
                    print!("X\t")
                } else {
                    print!("{}\t", num)
                }
            }
        }
        print!("\n")
    }
}

fn clear()
{
    if !cfg!(target_os = "windows") {
        let output = Command::new("clear")
                    .output()
                    .expect("failure to clear window");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn validate_input(input: &str) -> bool {
    println!("{}", input);
    match input.to_uppercase().as_ref() {
        "W" => true,
        "A" => true,
        "S" => true,
        "D" => true,
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
            io::stdin().read_line(&mut input);
            println!("You entered: {}", input.trim());
            valid_input = validate_input(&input.trim());
        }
        clear();
    }
}
