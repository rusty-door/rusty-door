mod geometry;
mod render;
mod scene;

#[macro_use]
extern crate glium;

use std::time::{Duration, Instant};
use std::thread;

use glium::{DisplayBuild, Surface};
use glium::glutin;

use glium::glutin::VirtualKeyCode;

enum Action {
    Continue,
    Key(glutin::VirtualKeyCode),
    Stop
}

struct State {

}

// Taken from glium.git/examples/support/mod.rs
fn start_loop<F>(scr: &mut State, mut callback: F)
    where F: FnMut(&geometry::World) -> Action {
        let mut accumulator = Duration::new(0, 0);
        let mut previous_clock = Instant::now();

        loop {
            let mut key = None;
            match callback(&scene::scene()) {
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
            }

            thread::sleep(fixed_time_stamp - accumulator);
        }
}

fn main() {
    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(640, 480)
        .build_glium()
        .unwrap();

    let mut scr = State {
        };

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

