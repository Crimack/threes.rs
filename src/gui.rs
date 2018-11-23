use std::collections::HashMap;
use sdl2;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use board::Board;


use std::{thread, time};

struct ThreesWindow {
    board: Board,
    canvas: Canvas<Window>,
}

impl<'a> ThreesWindow {
    fn new() -> ThreesWindow {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Threes.rs", 420, 420)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .unwrap();

        canvas.clear();

        ThreesWindow {
            board: Board::new(),
            canvas,
        }
    }

    fn play(&mut self) {
        let texture_creator = self.canvas.texture_creator();

        let mut assets = HashMap::new();
        assets.insert(0, texture_creator.load_texture("./resources/zero.png").unwrap());
        assets.insert(1, texture_creator.load_texture("./resources/one.png").unwrap());
        assets.insert(2, texture_creator.load_texture("./resources/two.png").unwrap());
        assets.insert(3, texture_creator.load_texture("./resources/three.png").unwrap());
        assets.insert(6, texture_creator.load_texture("./resources/six.png").unwrap());
        assets.insert(12, texture_creator.load_texture("./resources/twelve.png").unwrap());
        assets.insert(24, texture_creator.load_texture("./resources/twenty_four.png").unwrap());
        assets.insert(48, texture_creator.load_texture("./resources/forty_eight.png").unwrap());
        assets.insert(96, texture_creator.load_texture("./resources/ninety_six.png").unwrap());
        assets.insert(192, texture_creator.load_texture("./resources/one_nine_two.png").unwrap());
        assets.insert(384, texture_creator.load_texture("./resources/three_eight_four.png").unwrap());
        assets.insert(762, texture_creator.load_texture("./resources/seven_six_two.png").unwrap());
        assets.insert(1524, texture_creator.load_texture("./resources/one_five_two_four.png").unwrap());
        assets.insert(3048, texture_creator.load_texture("./resources/three_zero_four_eight.png").unwrap());

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
                    self.canvas.copy(assets.get(col).unwrap(), None, Rect::new(
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
