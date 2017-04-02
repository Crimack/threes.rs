
use rand::distributions::{IndependentSample, Range};
use rand::thread_rng;

enum SpawnPosition {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Debug)]
pub struct Board {
    state: [[u32; 4]; 4]
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
        Board { state: starting_state } 
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
        
    }

    pub fn has_moves(&self) -> bool {
        true
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
    if (x == 0) {
        (true, Some(y))
    } else if (x==1 && y==2) | (y==1 && x==2) {
        (true, Some(3))
    } else if x == y && x > 2 && y > 2 {
        (true, Some(x*2))
    } else {
        (false, None)
    }
}

// fn spawn_new_tile(board: Board, position: SpawnPosition)