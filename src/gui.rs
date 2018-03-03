use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use board::Board;

use std::{thread, time};

enum TileColour {
    Blue,
    Red,
    White,
    Empty,
}

impl TileColour {
    fn value(&self) -> [u8; 3] {
        match *self {
            TileColour::Blue => [0, 102, 204],
            TileColour::Red => [255, 0, 0],
            TileColour::White => [224, 224, 224],
            TileColour::Empty => [47, 79, 79],
        }
    }
}

struct ThreesWindow {
    board: Board,
    window: PistonWindow,
}

impl ThreesWindow {
    fn new() -> ThreesWindow {
        WindowSettings::new("Hello Piston!", [800, 600])
        .exit_on_esc(true).build().unwrap();

        ThreesWindow {
            board: Board::new(),
            canvas,
        }
    }

    fn play(&mut self) {
        let mut event_pump = self.canvas.window().subsystem().sdl().event_pump().unwrap();
        'game: loop {
            if !self.board.has_moves() {
                break 'game;
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'game;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::W),
                        ..
                    } => {
                        self.board.move_up();
                        break;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        self.board.move_left();
                        break;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => {
                        self.board.move_down();
                        break;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => {
                        self.board.move_right();
                        break;
                    }
                    Event::AppDidEnterBackground { .. } => {
                        thread::sleep(time::Duration::from_millis(250));
                    }
                    _ => {}
                }
            }

            // Redraw the board onto the screen
            let state = self.board.get_board();
            for (row_num, row) in state.iter().enumerate() {
                for (col_num, col) in row.iter().enumerate() {
                    let colour = match *col {
                        0 => TileColour::Empty,
                        1 => TileColour::Blue,
                        2 => TileColour::Red,
                        _ => TileColour::White,
                    }.value();
                    self.canvas
                        .set_draw_color(Color::RGB(colour[0], colour[1], colour[2]));
                    self.canvas
                        .fill_rect(Rect::new(
                            10 + (100 * col_num) as i32,
                            10 + (100 * row_num) as i32,
                            100,
                            100,
                        ))
                        .expect("Failed to draw rect");
                }
            }

            self.canvas.present();
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

pub fn new_game() {
    let mut game = ThreesWindow::new();
    game.play();
}
