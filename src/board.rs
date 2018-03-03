use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Board {
    state: [[u32; 4]; 4],
    high_card: u32,
    next_card: u32,
    basic_cards: Vec<u32>,
    bonus_cards: Vec<u32>,
}

impl Board {
    pub fn new() -> Board {
        let mut starting_state: [[u32; 4]; 4] = [[0; 4], [0; 4], [0; 4], [0; 4]];

        let mut basic_stack = generate_basic_stack();
        let between = Range::new(0, 4);
        let mut rng = thread_rng();

        // The starting board starts 9 cards off the basic stack at random places
        for _ in 0..9 {
            let mut valid_place: bool = false;
            while !valid_place {
                let x = between.ind_sample(&mut rng);
                let y = between.ind_sample(&mut rng);
                if starting_state[x][y] == 0 {
                    starting_state[x][y] = basic_stack.pop().unwrap();
                    valid_place = true;
                }
            }
        }

        Board {
            state: starting_state,
            high_card: 3, // Can't be anything higher at this point
            next_card: basic_stack.pop().unwrap(), // Next card is guaranteed to be basic
            basic_cards: basic_stack,
            bonus_cards: Vec::new(), // Guaranteed to be empty
        }
    }

    pub fn move_up(&mut self) {
        let mut moved = false;
        // Resolve by column from left to right
        for col in 0..4 {
            // Resolve from top to bottom, skipping final row
            for row in 0..3 {
                if let Some(x) = handle_collisions(self.state[row][col], self.state[row + 1][col]) {
                    self.state[row + 1][col] = 0;
                    self.state[row][col] = x;
                    self.update_high_card(x);
                    moved = true;
                };
            }
        }

        if moved {
            // Spawn new tile in bottom row
            let mut possible_locations = vec![];
            for col in 0..4 {
                if self.state[3][col] == 0 {
                    possible_locations.push(col);
                }
            }
            let between = Range::new(0, possible_locations.len());
            let mut rng = thread_rng();
            let y = possible_locations[between.ind_sample(&mut rng)];
            self.spawn_next_tile(3, y);
        } else {
            println!("Invalid move");
        }
    }

    pub fn move_down(&mut self) {
        let mut moved = false;
        // Resolve by column from left to right
        for col in 0..4 {
            // Resolve from bottom to top, skipping first row
            for row in (1..4).rev() {
                if let Some(x) = handle_collisions(self.state[row][col], self.state[row - 1][col]) {
                    self.state[row - 1][col] = 0;
                    self.state[row][col] = x;
                    self.update_high_card(x);
                    moved = true;
                };
            }
        }

        if moved {
            // Spawn new tile in top row
            let mut possible_locations = vec![];
            for col in 0..4 {
                if self.state[0][col] == 0 {
                    possible_locations.push(col);
                }
            }
            let between = Range::new(0, possible_locations.len());
            let mut rng = thread_rng();
            let y = possible_locations[between.ind_sample(&mut rng)];
            self.spawn_next_tile(0, y);
        } else {
            println!("Invalid move");
        }
    }

    pub fn move_left(&mut self) {
        let mut moved = false;
        // Resolve by column from top to bottom
        for row in 0..4 {
            // Resolve from left to right, skipping final column
            for col in 0..3 {
                if let Some(x) = handle_collisions(self.state[row][col], self.state[row][col + 1]) {
                    self.state[row][col + 1] = 0;
                    self.state[row][col] = x;
                    self.update_high_card(x);
                    moved = true;
                };
            }
        }

        if moved {
            // Spawn new tile in right column
            let mut possible_locations = vec![];
            for row in 0..4 {
                if self.state[row][3] == 0 {
                    possible_locations.push(row);
                }
            }
            let between = Range::new(0, possible_locations.len());
            let mut rng = thread_rng();
            let x = possible_locations[between.ind_sample(&mut rng)];
            self.spawn_next_tile(x, 3);
        } else {
            println!("Invalid move");
        }
    }

    pub fn move_right(&mut self) {
        let mut moved = false;
        // Resolve by column from top to bottom
        for row in 0..4 {
            // Resolve from right to left, skipping first column
            for col in (1..4).rev() {
                if let Some(x) = handle_collisions(self.state[row][col], self.state[row][col - 1]) {
                    self.state[row][col - 1] = 0;
                    self.state[row][col] = x;
                    self.update_high_card(x);
                    moved = true;
                };
            }
        }

        if moved {
            // Spawn new tile in left column
            let mut possible_locations = vec![];
            for row in 0..4 {
                if self.state[row][0] == 0 {
                    possible_locations.push(row);
                }
            }
            let between = Range::new(0, possible_locations.len());
            let mut rng = thread_rng();
            let x = possible_locations[between.ind_sample(&mut rng)];
            self.spawn_next_tile(x, 0);
        } else {
            println!("Invalid move");
        }
    }

    fn update_high_card(&mut self, new_card: u32) {
        if new_card > self.high_card {
            self.high_card = new_card;
        }
    }

    fn spawn_next_tile(&mut self, x: usize, y: usize) {
        self.state[x][y] = self.next_card;

        let between = Range::new(0, 21);
        let mut rng = thread_rng();
        let new_tile = if self.high_card >= 48 && between.ind_sample(&mut rng) == 7 {
            self.bonus_cards = generate_bonus_stack(self.high_card);
            self.bonus_cards.pop()
        } else {
            if self.basic_cards.is_empty() {
                self.basic_cards = generate_basic_stack();
            }
            self.basic_cards.pop()
        };
        self.next_card = new_tile.unwrap();
    }

    pub fn get_board(&self) -> &[[u32; 4]] {
        &self.state
    }

    pub fn get_next_card(&self) -> u32 {
        self.next_card
    }

    pub fn print(&self) {
        for row in &self.state {
            println!();
            for num in row {
                if *num == 0 {
                    print!("X\t")
                } else {
                    print!("{}\t", num)
                }
            }
        }
        println!();
        println!("\n\n")
    }

    pub fn has_moves(&self) -> bool {
        // Check for vertical moves
        for col in 0..4 {
            for row in 0..3 {
                if handle_collisions(self.state[row][col], self.state[row + 1][col]).is_some() {
                    return true;
                }
            }
        }
        // Check for horizontal moves
        for row in 0..4 {
            for col in 0..3 {
                if handle_collisions(self.state[row][col], self.state[row][col + 1]).is_some() {
                    return true;
                }
            }
        }
        false
    }

    pub fn calculate_score(&self) -> u64 {
        let mut score = 0;
        for row in self.get_board().iter() {
            for tile in row.iter() {
                // 1s and 2s are not worth points
                if *tile == 1 || *tile == 2 {
                    continue;
                }
                score += 3u64.pow(calculate_coefficient(*tile));
            }
        }
        score
    }
}

fn calculate_coefficient(x: u32) -> u32 {
    // There's probably a much more elegant way of doing this
    let mut y = x;
    let mut coefficient = 1;
    while y > 3 {
        y /= 2;
        coefficient += 1;
    }
    coefficient
}

fn handle_collisions(x: u32, y: u32) -> Option<u32> {
    if x == 0 {
        Some(y)
    } else if (x == 1 && y == 2) | (y == 1 && x == 2) {
        Some(3)
    } else if x == y && x > 2 && y > 2 {
        Some(x * 2)
    } else {
        None
    }
}

fn generate_basic_stack() -> Vec<u32> {
    let mut stack = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];
    let mut rng = thread_rng();
    rng.shuffle(&mut stack);
    stack
}

fn generate_bonus_stack(high_card: u32) -> Vec<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut next_value = high_card / 8;
    while next_value > 3 {
        stack.push(next_value);
        next_value /= 2;
    }
    let mut rng = thread_rng();
    rng.shuffle(&mut stack);
    stack
}

#[test]
fn test_new_board_has_moves() {
    let board = Board::new();
    assert_eq!(true, board.has_moves());
}

#[test]
fn test_half_played_has_moves() {
    let state: [[u32; 4]; 4] = [
        [192, 384, 1, 2],
        [6, 3, 1, 3],
        [192, 3, 1, 12],
        [6, 12, 24, 6],
    ];
    let board = Board {
        state: state,
        high_card: 384,
        next_card: 3,
        basic_cards: vec![],
        bonus_cards: vec![],
    };
    assert_eq!(true, board.has_moves());
}

#[test]
fn test_full_board_no_moves() {
    let state: [[u32; 4]; 4] = [
        [192, 384, 1, 1],
        [6, 3, 1, 3],
        [192, 48, 1, 12],
        [6, 12, 24, 6],
    ];
    let board = Board {
        state: state,
        high_card: 384,
        next_card: 3,
        basic_cards: vec![],
        bonus_cards: vec![],
    };
    assert_eq!(false, board.has_moves());
}

#[test]
fn test_calculate_score_zero() {
    let state: [[u32; 4]; 4] = [[1, 2, 1, 1], [2, 2, 1, 1], [2, 2, 1, 2], [2, 1, 2, 1]];
    let board = Board {
        state: state,
        high_card: 2,
        next_card: 3,
        basic_cards: vec![],
        bonus_cards: vec![],
    };
    assert_eq!(0, board.calculate_score());
}

#[test]
fn test_calculate_score_low() {
    let state: [[u32; 4]; 4] = [[3, 6, 3, 2], [2, 3, 6, 3], [3, 12, 1, 6], [12, 48, 6, 3]];
    let board = Board {
        state: state,
        high_card: 384,
        next_card: 3,
        basic_cards: vec![],
        bonus_cards: vec![],
    };
    assert_eq!(351, board.calculate_score());
}

#[test]
fn test_calculate_score_mid() {
    let state: [[u32; 4]; 4] = [
        [1, 3, 48, 1],
        [6, 2, 12, 24],
        [3, 6, 24, 2],
        [768, 384, 96, 3],
    ];
    let board = Board {
        state: state,
        high_card: 384,
        next_card: 3,
        basic_cards: vec![],
        bonus_cards: vec![],
    };
    assert_eq!(27432, board.calculate_score());
}

#[test]
fn test_calculate_score_high() {
    let state: [[u32; 4]; 4] = [
        [2, 3, 96, 3],
        [12, 6, 48, 2],
        [6, 48, 24, 6],
        [1536, 768, 384, 192],
    ];
    let board = Board {
        state: state,
        high_card: 1536,
        next_card: 3,
        basic_cards: vec![],
        bonus_cards: vec![],
    };
    assert_eq!(88836, board.calculate_score());
}

#[test]
fn test_collision_x_zero() {
    let (x, y) = (0, 3);
    let result = handle_collisions(x, y);
    assert_eq!(result, Some(y));
}

#[test]
fn test_collision_one_two() {
    let (x, y) = (1, 2);
    assert_eq!(handle_collisions(x, y), Some(3));
    assert_eq!(handle_collisions(y, x), Some(3));
}

#[test]
fn test_collision_of_like_values() {
    assert_eq!(handle_collisions(3, 3), Some(6));
    assert_eq!(handle_collisions(6, 6), Some(12));
    assert_eq!(handle_collisions(12, 12), Some(24));
    assert_eq!(handle_collisions(24, 24), Some(48));
    assert_eq!(handle_collisions(48, 48), Some(96));
    assert_eq!(handle_collisions(96, 96), Some(192));
    assert_eq!(handle_collisions(192, 192), Some(384));
}

#[test]
fn test_collision_fails() {
    assert_eq!(handle_collisions(1, 1), None);
    assert_eq!(handle_collisions(2, 2), None);
    assert_eq!(handle_collisions(3, 1), None);
    assert_eq!(handle_collisions(3, 2), None);
    assert_eq!(handle_collisions(1, 3), None);
    assert_eq!(handle_collisions(2, 3), None);
}

#[test]
fn test_basic_stack() {
    let mut stack = generate_basic_stack();
    assert_eq!(12, stack.len());
    stack.sort();
    assert_eq!(stack[0], 1);
    assert_eq!(stack[4], 2);
    assert_eq!(stack[8], 3);
}

#[test]
fn test_bonus_stack_empty() {
    let stack = generate_bonus_stack(24);
    assert_eq!(0, stack.len());
}

#[test]
fn test_bonus_stack_96() {
    let mut stack = generate_bonus_stack(96);
    assert_eq!(2, stack.len());
    stack.sort();
    assert_eq!(stack[0], 6);
    assert_eq!(stack[1], 12);
}

#[test]
fn test_bonus_stack_192() {
    let mut stack = generate_bonus_stack(192);
    assert_eq!(3, stack.len());
    stack.sort();
    assert_eq!(stack[0], 6);
    assert_eq!(stack[1], 12);
    assert_eq!(stack[2], 24);
}

#[test]
fn test_bonus_stack_384() {
    let mut stack = generate_bonus_stack(384);
    assert_eq!(4, stack.len());
    stack.sort();
    assert_eq!(stack[0], 6);
    assert_eq!(stack[1], 12);
    assert_eq!(stack[2], 24);
    assert_eq!(stack[3], 48);
}
