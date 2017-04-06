
use rand::distributions::{IndependentSample, Range};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Board {
    state: [[u32; 4]; 4],
    high_card: u32,
    next_card: u32,
    basic_cards: Vec<u32>,
    bonus_cards: Vec<u32>,
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

        Board { state: starting_state,
                high_card: 3, // Can't be anything higher at this point
                next_card: basic_stack.pop().unwrap(), // Next card is guaranteed to be basic
                basic_cards: basic_stack,
                bonus_cards: Vec::new(), // Guaranteed to be empty
                has_moves: true // Guaranteed to be playable
                } 
    }

    pub fn move_up(&mut self) {
        // Resolve by column from left to right
        for col in 0..4 {
            // Resolve from top to bottom, skipping final row
            for row in 0..3 {
                let new_value = handle_collisions(self.state[row][col], self.state[row + 1][col]);
                match new_value {
                    Some(x) => {
                        self.state[row + 1][col] = 0;
                        self.state[row][col] = x;
                        self.update_high_card(x);
                    },
                    _ => {},
                };
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
        let y = possible_locations[between.ind_sample(&mut rng)];
        self.spawn_next_tile(3, y);
    }

    pub fn move_down(&mut self) {
        // Resolve by column from left to right
        for col in 0..4 {
            // Resolve from bottom to top, skipping first row
            for row in (1..4).rev() {
                let new_value = handle_collisions(self.state[row][col], self.state[row - 1][col]);
                match new_value {
                    Some(x) => {
                        self.state[row - 1][col] = 0;
                        self.state[row][col] = x;
                        self.update_high_card(x);                
                    },
                    _ => {},
                };
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
        let y = possible_locations[between.ind_sample(&mut rng)];
        self.spawn_next_tile(0, y);
    }

    pub fn move_left(&mut self) {
        // Resolve by column from top to bottom
        for row in 0..4 {
            // Resolve from left to right, skipping final column
            for col in 0..3 {
                let new_value = handle_collisions(self.state[row][col], self.state[row][col + 1]);
                match new_value {
                    Some(x) => {
                    self.state[row][col + 1] = 0;
                    self.state[row][col] = x;
                        self.update_high_card(x);                
                    },
                    _ => {},
                };
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
        let x = possible_locations[between.ind_sample(&mut rng)];
        self.spawn_next_tile(x, 3);    
    }

    pub fn move_right(&mut self) {
        // Resolve by column from top to bottom
        for row in 0..4 {
            // Resolve from right to left, skipping first column
            for col in (1..4).rev() {
                let new_value = handle_collisions(self.state[row][col], self.state[row][col - 1]);
                match new_value {
                    Some(x) => {
                        self.state[row][col - 1] = 0;
                        self.state[row][col] = x;
                        self.update_high_card(x);                
                    },
                    _ => {},
                };
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
        let x = possible_locations[between.ind_sample(&mut rng)];
        self.spawn_next_tile(x, 0);
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
        let new_tile = if self.high_card > 3 && between.ind_sample(&mut rng) == 7 {
            self.bonus_cards = generate_bonus_stack(self.high_card);
            self.bonus_cards.pop()
        } else {
            if self.basic_cards.len() == 0 {
                self.basic_cards = generate_basic_stack();
            }
            self.basic_cards.pop()
        };
        self.next_card = new_tile.unwrap();
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
        print!("\n");
        println!("Next card: {}", self.next_card);
        print!("\n\n\n")
    }


}

fn handle_collisions(x: u32, y: u32) -> Option<u32> {
    if x == 0 {
        Some(y)
    } else if (x==1 && y==2) | (y==1 && x==2) {
        Some(3)
    } else if x == y && x > 2 && y > 2 {
        Some(x*2)
    } else {
        None
    }
}

fn generate_basic_stack() -> Vec<u32> {
    let mut stack = vec![1,1,1,1,2,2,2,2,3,3,3,3];
    let mut rng = thread_rng();
    rng.shuffle(&mut stack);
    stack
}

fn generate_bonus_stack(high_card: u32) -> Vec<u32> {
    let mut stack: Vec<u32> = Vec::new();
    let mut next_value = high_card / 8;
    while next_value > 3 {
        stack.push(next_value);
        next_value = next_value / 2;
        println!("{:?}", stack);
    }
    let mut rng = thread_rng();
    rng.shuffle(&mut stack);
    stack
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


