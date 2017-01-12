mod labyrinth;
mod menu;
mod tickable;
mod state;
mod direction;
mod game;
mod play;
mod license;
mod screen;
mod geometry;
mod render;

#[macro_use]
extern crate glium;

use std::time::{Duration, Instant};
use std::thread;

use glium::{DisplayBuild, Surface};
use glium::glutin;

use glium::glutin::VirtualKeyCode;

use tickable::Input;
use screen::Screen;
use direction::{DIR_UP,DIR_DOWN,DIR_LEFT,DIR_RIGHT};

enum Action {
    Continue,
    Key(glutin::VirtualKeyCode),
    Stop
}

// Taken from glium.git/examples/support/mod.rs
fn start_loop<F>(scr: &mut Box<Screen>, mut callback: F)
    where F: FnMut(&geometry::World) -> Action {
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        loop {
            let mut key = None;
            match callback(&scr.scene()) {
                Action::Stop => break,
                Action::Key(k) => key = Some(k),
                Action::Continue => ()
            };

            let now = Instant::now();
            accumulator += now - previous_clock;
            previous_clock = now;

            let fixed_time_stamp = Duration::new(0, 16666667);
            while accumulator >= fixed_time_stamp {
                accumulator -= fixed_time_stamp;

                // Modifying the state of the game
                if let Some(b) = key {
                    let ev = match b {
                        VirtualKeyCode::H => Some(Input::Direction(DIR_LEFT)),
                        VirtualKeyCode::J => Some(Input::Direction(DIR_DOWN)),
                        VirtualKeyCode::K => Some(Input::Direction(DIR_UP)),
                        VirtualKeyCode::L => Some(Input::Direction(DIR_RIGHT)),
                        VirtualKeyCode::Z => Some(Input::Accept),
                        VirtualKeyCode::X => Some(Input::Cancel),
                        VirtualKeyCode::Colon => Some(Input::Menu),
                        _ => None
                    };
                    if let Some(f) = scr.tick(ev) {
                        *scr = f;
                    }
                    println!("{:?}", scr);
                }
            }

            thread::sleep(fixed_time_stamp - accumulator);
        }
}

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .build_glium()
        .unwrap();

    let pr = state::ProgramState::new();
    let mut scr : Box<Screen> = Box::new(menu::MenuScreen::new(pr));
    start_loop(&mut scr, |scene| {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return Action::Stop,
                glutin::Event::KeyboardInput(
                    glutin::ElementState::Pressed,
                    _,
                    Some(key)
                    ) => return Action::Key(key),
                _ => ()
            }
        }
        let mut canvas = render::Canvas::new(640, 480);
        canvas.render(scene);
        let pixels : Vec<Vec<(u8, u8, u8)>> = canvas.pixels().iter().map(
            |r| r.iter().map(
                |&geometry::RGB(r, g, b)| (r, g, b)).collect()).collect();
        let opengl_texture = glium::Texture2d::new(&display, pixels).unwrap();

        let target = display.draw();
        opengl_texture.as_surface().fill(
            &target, glium::uniforms::MagnifySamplerFilter::Linear);
        target.finish().unwrap();
        Action::Continue
    });

}

