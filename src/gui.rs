
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use board::Board;

enum TileColour {
    Blue,
    Red,
    White,
}

impl TileColour {
    fn value(&self) -> [u8; 3] {
        match *self {
            TileColour::Blue => [0, 102, 204],
            TileColour::Red => [255, 0, 0],
            TileColour::White => [224, 224, 224],
        }
    }
}

struct ThreesWindow {
    board: Board,
    canvas: Canvas<Window>,
}

impl ThreesWindow {
    fn new() -> ThreesWindow {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Threes.rs", 800, 600)
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
        let mut event_pump = self.canvas.window().subsystem().sdl().event_pump().unwrap();
        'game: loop {
            if !self.board.has_moves() {
                break 'game;
            }
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'game;
                    }
                    Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                        self.board.move_up();
                    }
                    Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                        self.board.move_left();
                    }
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        self.board.move_down();
                    }
                    Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                        self.board.move_right();
                    }
                    _ => {}
                }
            }

            // Redraw the board onto the screen
            let state = self.board.get_board();
            for row in state.iter() {
                for col in row.iter() {
                    let colour = match *col {
                        1 => TileColour::Blue,
                        2 => TileColour::Red,
                        _ => TileColour::White,
                    }.value();
                    self.canvas.set_draw_color(
                        Color::RGB(colour[0], colour[1], colour[2]),
                    );
                    self.canvas.fill_rect(Rect::new(10, 10, 200, 200));
                }
            }

            self.canvas.present();
        }
    }
}

pub fn new_game() {
    let mut game = ThreesWindow::new();
    game.play();
}
