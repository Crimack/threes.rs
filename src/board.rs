
use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    state: [[u32; 4]; 4],
    has_moves: bool
}

impl Board {
    pub fn new() -> Board {
        let mut starting_state: [[u32;4];4] = [
                                                [0; 4],
                                                [0; 4],
                                                [0; 4],
                                                [0; 4]
                                             ];
        let between = Range::new(0, 4);
        let mut rng = thread_rng();

        // The starting board starts with 3 1s, 3 2s and 3 3s at seemingly random places
        for tile_type in 1..4 {
            // Spawn 3 of them
            for _ in 1..4 {
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
        Board { state: starting_state, has_moves: true } 
    }

    pub fn move_up(&mut self) {
        // Resolve by column from left to right
        for col in 0..4 {
            // Resolve from top to bottom, skipping final row
            for row in 0..3 {
                let (collides, new_value) = handle_collisions(self.state[row][col], self.state[row + 1][col]);
                if collides {
                    self.state[row + 1][col] = 0;
                    self.state[row][col] = new_value.unwrap();
                }
            }
        }

        // Spawn new tile in bottom row
        let mut possible_locations = vec![];
        for col in 0..4 {
            if self.state[3][col] == 0 {
                possible_locations.push(col);
            }
        }
        let between = Range::new(0, possible_locations.len());
        let mut rng = thread_rng();
        self.state[3][possible_locations[between.ind_sample(&mut rng)]] = generate_new_tile_value(&self);
    }

    pub fn move_down(&mut self) {
        // Resolve by column from left to right
        for col in 0..4 {
            // Resolve from bottom to top, skipping first row
            for row in (1..4).rev() {
                let (collides, new_value) = handle_collisions(self.state[row][col], self.state[row - 1][col]);
                if collides {
                    self.state[row - 1][col] = 0;
                    self.state[row][col] = new_value.unwrap();
                }
            }
        }
        // Spawn new tile in top row
        let mut possible_locations = vec![];
        for col in 0..4 {
            if self.state[0][col] == 0 {
                possible_locations.push(col);
            }
        }
        let between = Range::new(0, possible_locations.len());
        let mut rng = thread_rng();
        self.state[0][possible_locations[between.ind_sample(&mut rng)]] = generate_new_tile_value(&self);
    }

    pub fn move_left(&mut self) {
        // Resolve by column from top to bottom
        for row in 0..4 {
            // Resolve from left to right, skipping final column
            for col in 0..3 {
                let (collides, new_value) = handle_collisions(self.state[row][col], self.state[row][col + 1]);
                if collides {
                    self.state[row][col + 1] = 0;
                    self.state[row][col] = new_value.unwrap();
                }
            }
        }

        // Spawn new tile in right column
        let mut possible_locations = vec![];
        for row in 0..4 {
            if self.state[row][3] == 0 {
                possible_locations.push(row);
            }
        }
        let between = Range::new(0, possible_locations.len());
        let mut rng = thread_rng();
        self.state[possible_locations[between.ind_sample(&mut rng)]][3] = generate_new_tile_value(&self);
    }

    pub fn move_right(&mut self) {
        // Resolve by column from top to bottom
        for row in 0..4 {
            // Resolve from right to left, skipping first column
            for col in (1..4).rev() {
                let (collides, new_value) = handle_collisions(self.state[row][col], self.state[row][col - 1]);
                if collides {
                    self.state[row][col - 1] = 0;
                    self.state[row][col] = new_value.unwrap();
                }
            }
        }

        // Spawn new tile in left column
        let mut possible_locations = vec![];
        for row in 0..4 {
            if self.state[row][0] == 0 {
                possible_locations.push(row);
            }
        }
        let between = Range::new(0, possible_locations.len());
        let mut rng = thread_rng();
        self.state[possible_locations[between.ind_sample(&mut rng)]][0] = generate_new_tile_value(&self);    
    }

    pub fn get_value_at(&self, x: usize, y: usize) -> u32 {
        let result = self.state[x][y];
        result
    }

    pub fn has_moves(&self) -> bool {
        self.has_moves
    }

    pub fn print(&self) {
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

fn handle_collisions(x: u32, y: u32) -> (bool, Option<u32>) {
    if x == 0 {
        (true, Some(y))
    } else if (x==1 && y==2) | (y==1 && x==2) {
        (true, Some(3))
    } else if x == y && x > 2 && y > 2 {
        (true, Some(x*2))
    } else {
        (false, None)
    }
}

fn generate_new_tile_value(board: &Board) -> u32 {
    let mut tile_counter = HashMap::new();
    // Gather counts of the number of different tile values
    for row in 0..4 {
        for col in 0..4 {
            let counter = tile_counter
                            .entry(board.get_value_at(row, col))
                            .or_insert(0);
            *counter += 1;
        }
    }
    // Remove empty tiles
    tile_counter.remove(&0);

    // Pick the tile with the 3rd smallest number of entries in the board
    // We don't want the largest tile to constantly spawn
    let mut largest = (0, 0);
    let mut next_largest = (0, 0);
    let mut target = (0, 0);

    for (key, value) in tile_counter.iter() {
        if *value > largest.1 {
            target = next_largest;
            next_largest = largest;
            largest = (*key, *value);
        } else if (*value == largest.1) || (*value > next_largest.1) {
            // Should really roll a die here to see if we swap if equal
            target = next_largest;
            next_largest = (*key, *value);
        } else if (*value == next_largest.1) || (*value > target.1) {
                target = (*key, *value);
        }
    }
    target.0
    
}
