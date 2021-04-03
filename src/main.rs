#![allow(dead_code)]

use std::thread::sleep;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WIN_WIDTH: u32 = 600;
const WIN_HEIGHT: u32 = 800;
const BLOCKSIZE: u32 = 32;
const MARGIN: u32 = 2;
const PLAYFIELD_WIDTH: u32 = 10;
const PLAYFIELD_VISIBLE_HEIGHT: u32 = 20;
const PLAYFIELD_BUFFFER_HEIGHT: u32 = 20;
const PLAYFIELD_TOTAL_HEIGHT: u32 = PLAYFIELD_VISIBLE_HEIGHT + PLAYFIELD_BUFFFER_HEIGHT;
const BORDER_THICKNESS: u32 = 8;

// Color presets
const BLACK: Color = Color::BLACK;
const WHITE: Color = Color::WHITE;
const GRAY: Color = Color::GREY;
const YELLOW: Color = Color::YELLOW;
const RED: Color = Color::RED;
const BLUE: Color = Color::BLUE;
const LIGHTBLUE: Color = Color::RGB(0, 192, 192);
const GREEN: Color = Color::GREEN;
const ORANGE: Color = Color::RGB(255, 153, 0);
const PURPLE: Color = Color::RGB(255, 0, 255);

// Tetrominoes colors
const O_COLOR: Color = YELLOW;
const I_COLOR: Color = LIGHTBLUE;
const S_COLOR: Color = GREEN;
const Z_COLOR: Color = RED;
const J_COLOR: Color = BLUE;
const L_COLOR: Color = ORANGE;
const T_COLOR: Color = PURPLE;

// Other colors
const PF_COLOR: Color = BLACK;
const BG_COLOR: Color = GRAY;
const BORDER_COLOR: Color = WHITE;

struct Piece {
    states: Vec<Vec<Vec<u8>>>,
    x: isize,
    y: usize,
    current_state: u8,
}

trait TetrominoGenerator {
    fn new() -> Piece;
}

struct TetrominoO;
struct TetrominoI;
struct TetrominoS;
struct TetrominoZ;
//struct TetrominoJ;
//struct TetrominoL;
//struct TetrominoT;

impl TetrominoGenerator for TetrominoO {
    fn new() -> Piece {
        Piece {
            states: vec![
                // State 0
                vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]],
                // State 1
                vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]],
                // State 2
                vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]],
                // State 3
                vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]],
            ],
            x: 0,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetrominoGenerator for TetrominoI {
    fn new() -> Piece {
        Piece {
            states: vec![
                // State 0
                vec![
                    vec![0, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                ],
                // State 1
                vec![
                    vec![0, 0, 1, 0],
                    vec![0, 0, 1, 0],
                    vec![0, 0, 1, 0],
                    vec![0, 0, 1, 0],
                ],
                // State 2
                vec![
                    vec![0, 0, 0, 0],
                    vec![0, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![0, 0, 0, 0],
                ],
                // State 3
                vec![
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 1, 0, 0],
                ],
            ],
            x: 0,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetrominoGenerator for TetrominoS {
    fn new() -> Piece {
        Piece {
            states: vec![
                // State 0
                vec![vec![0, 1, 1], vec![1, 1, 0], vec![0, 0, 0]],
                // State 1
                vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 0, 1]],
                // State 2
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 1, 0]],
                // State 3
                vec![vec![1, 0, 0], vec![1, 1, 0], vec![0, 1, 0]],
            ],
            x: 0,
            y: 0,
            current_state: 0,
        }
    }
}

impl TetrominoGenerator for TetrominoZ {
    fn new() -> Piece {
        Piece {
            states: vec![
                // State 0
                vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0]],
                // State 1
                vec![vec![0, 0, 1], vec![0, 1, 1], vec![0, 1, 0]],
                // State 2
                vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 1, 1]],
                // State 3
                vec![vec![0, 1, 0], vec![1, 1, 0], vec![1, 0, 0]],
            ],
            x: 0,
            y: 0,
            current_state: 0,
        }
    }
}

struct Tetrust {
    playfield: Vec<u8>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Piece>,
}

impl Tetrust {
    fn new() -> Tetrust {
        let playfield = vec![0; PLAYFIELD_WIDTH as usize * PLAYFIELD_TOTAL_HEIGHT as usize];
        Tetrust {
            playfield,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
        }
    }
}

struct S<T>(T, T, T, T);

fn main() {
    let a: &'static str = r#" 🦀 "#;
    let hello_html = include_str!("hello.html");
    // INIT
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Could not get SDL vidéo subsystem");

    let window = video_subsystem
        .window("Tetrust", 600, 800)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Could not get window's canvas");

    let game_state = Tetrust::new();

    let playfield_pixel_width = (BLOCKSIZE + MARGIN * 2) * PLAYFIELD_WIDTH;
    let playfield_pixel_height = (BLOCKSIZE + MARGIN * 2) * PLAYFIELD_VISIBLE_HEIGHT;
    let playfield_x = (WIN_WIDTH - playfield_pixel_width) / 2;
    let playfield_y = (WIN_HEIGHT - playfield_pixel_height) / 2;
    let playfield_rect = Rect::new(playfield_x as i32,
        playfield_y as i32,
        playfield_pixel_width,
        playfield_pixel_height
    );
    let border_rect = Rect::new(playfield_x as i32 - BORDER_THICKNESS as i32,
        playfield_y as i32 - BORDER_THICKNESS as i32,
        playfield_pixel_width + BORDER_THICKNESS * 2,
        playfield_pixel_height + BORDER_THICKNESS * 2
    );

    // GAME LOOP
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(BG_COLOR);
        canvas.clear();
        canvas.set_draw_color(BORDER_COLOR);
        canvas.fill_rect(border_rect).expect("Error drawing border");
        canvas.set_draw_color(PF_COLOR);
        canvas.fill_rect(playfield_rect).expect("Error drawing playfield");

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
