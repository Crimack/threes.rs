use sdl2;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::collections::HashMap;

use board::Board;

use std::{thread, time};

struct ThreesWindow {
    board: Board,
    canvas: Canvas<Window>,
}

enum PostGameOption {
    Quit,
    Restart,
    DisplayScore,
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
        let ttf_context = sdl2::ttf::init().unwrap();
        let font = ttf_context
            .load_font("./resources/font/Raleway-Black.ttf", 64)
            .unwrap();

        // TODO: Work out how to move this into a method without fighting the lifetime
        let mut assets = HashMap::new();
        assets.insert(
            0,
            texture_creator
                .load_texture("./resources/zero.png")
                .unwrap(),
        );
        assets.insert(
            1,
            texture_creator.load_texture("./resources/one.png").unwrap(),
        );
        assets.insert(
            2,
            texture_creator.load_texture("./resources/two.png").unwrap(),
        );
        assets.insert(
            3,
            texture_creator
                .load_texture("./resources/three.png")
                .unwrap(),
        );
        assets.insert(
            6,
            texture_creator.load_texture("./resources/six.png").unwrap(),
        );
        assets.insert(
            12,
            texture_creator
                .load_texture("./resources/twelve.png")
                .unwrap(),
        );
        assets.insert(
            24,
            texture_creator
                .load_texture("./resources/twenty_four.png")
                .unwrap(),
        );
        assets.insert(
            48,
            texture_creator
                .load_texture("./resources/forty_eight.png")
                .unwrap(),
        );
        assets.insert(
            96,
            texture_creator
                .load_texture("./resources/ninety_six.png")
                .unwrap(),
        );
        assets.insert(
            192,
            texture_creator
                .load_texture("./resources/one_nine_two.png")
                .unwrap(),
        );
        assets.insert(
            384,
            texture_creator
                .load_texture("./resources/three_eight_four.png")
                .unwrap(),
        );
        assets.insert(
            762,
            texture_creator
                .load_texture("./resources/seven_six_two.png")
                .unwrap(),
        );
        assets.insert(
            1524,
            texture_creator
                .load_texture("./resources/one_five_two_four.png")
                .unwrap(),
        );
        assets.insert(
            3048,
            texture_creator
                .load_texture("./resources/three_zero_four_eight.png")
                .unwrap(),
        );

        let mut event_pump = self.canvas.window().subsystem().sdl().event_pump().unwrap();
        'game: loop {
            if !self.board.has_moves() || !self.handle_input(&mut event_pump) {
                // if true {
                self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                self.canvas.clear();

                let score_str = format!("Score: {}", self.board.calculate_score());
                let score_surface = font
                    .render(&score_str)
                    .blended_wrapped(Color::RGB(0, 0, 0), 500)
                    .unwrap();
                let score_texture = texture_creator
                    .create_texture_from_surface(&score_surface)
                    .unwrap();

                let description_str = "Press r to start a new game, or q/ESC to quit";
                let desc_surface = font
                    .render(&description_str)
                    .blended_wrapped(Color::RGB(0, 0, 0), 500)
                    .unwrap();
                let desc_texture = texture_creator
                    .create_texture_from_surface(&desc_surface)
                    .unwrap();

                'end: loop {
                    match self.handle_end_input(&mut event_pump) {
                        PostGameOption::Quit => {
                            break 'game;
                        }
                        PostGameOption::DisplayScore => {}
                        PostGameOption::Restart => {
                            self.board = Board::new();
                            break 'end;
                        }
                    }
                    self.canvas
                        .copy(&score_texture, None, Rect::new(50, 50, 350, 100))
                        .unwrap();
                    self.canvas
                        .copy(&desc_texture, None, Rect::new(50, 200, 350, 200))
                        .unwrap();
                    self.canvas.present();

                    thread::sleep(time::Duration::from_millis(10));
                }
            }
            // Redraw the board onto the screen
            let state = self.board.get_board();
            for (row_num, row) in state.iter().enumerate() {
                for (col_num, col) in row.iter().enumerate() {
                    self.canvas
                        .copy(
                            assets.get(col).unwrap(),
                            None,
                            Rect::new(
                                10 + (100 * col_num) as i32,
                                10 + (100 * row_num) as i32,
                                100,
                                100,
                            ),
                        )
                        .expect("Failed to draw rect");
                }
            }

            self.canvas.present();
            thread::sleep(time::Duration::from_millis(10));
        }
    }

    fn handle_input(&mut self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    return false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    self.board.move_up();
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    self.board.move_left();
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    self.board.move_down();
                    return true;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    self.board.move_right();
                    return true;
                }
                Event::AppDidEnterBackground { .. } => {
                    thread::sleep(time::Duration::from_millis(250));
                }
                _ => {
                    return true;
                }
            }
        }
        true
    }

    fn handle_end_input(&mut self, event_pump: &mut EventPump) -> PostGameOption {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    return PostGameOption::Quit;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    return PostGameOption::Restart;
                }
                Event::AppDidEnterBackground { .. } => {
                    thread::sleep(time::Duration::from_millis(250));
                }
                _ => {
                    return PostGameOption::DisplayScore;
                }
            }
        }
        PostGameOption::DisplayScore
    }
}

pub fn new_game() {
    let mut game = ThreesWindow::new();
    game.play();
}
