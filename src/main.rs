extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 1200;

mod color_map;
mod complex;
mod mandelbrot;

mod test_color_map;

use mandelbrot::MandelbrotSet;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut mandelbrot = MandelbrotSet::new(WIDTH, HEIGHT, 100);
    mandelbrot.calculate();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Q) => {
                        mandelbrot.zoom_out(2.0);
                        mandelbrot.calculate();
                    }
                    Some(Keycode::E) => {
                        mandelbrot.zoom_in(2.0);
                        mandelbrot.calculate();
                    }
                    Some(Keycode::W) => {
                        mandelbrot.move_y(-0.1);
                        mandelbrot.calculate();
                    }
                    Some(Keycode::S) => {
                        mandelbrot.move_y(0.1);
                        mandelbrot.calculate();
                    }
                    Some(Keycode::A) => {
                        mandelbrot.move_x(-0.1);
                        mandelbrot.calculate();
                    }
                    Some(Keycode::D) => {
                        mandelbrot.move_x(0.1);
                        mandelbrot.calculate();
                    }
                    _ => {}
                },
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    mandelbrot.move_to(x as f64, y as f64);
                    mandelbrot.calculate();
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for y in 0..mandelbrot.height {
            for x in 0..mandelbrot.width {
                canvas.set_draw_color(
                    mandelbrot.val_to_color(mandelbrot.canvas[y as usize][x as usize]),
                );
                canvas.draw_point((x as i32, y as i32)).unwrap();
            }
        }

        canvas.present();
    }
}
