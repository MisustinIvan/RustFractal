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
mod render_text;
mod test;

use mandelbrot::MandelbrotSet;
use render_text::render_text;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_subsystem = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let window = video_subsystem
        .window("SDL2", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut help_toggled = false;

    let font_size = 32;
    let font_path = "./ttf/pcsenior.ttf";
    let font = ttf_subsystem
        .load_font(font_path, font_size)
        .map_err(|e| e.to_string())
        .unwrap();

    let color_map = color_map::ColorMap::new_default();
    //    let color_map = color_map::ColorMap::new(
    //        vec![
    //            Color::RGB(255, 0, 0),
    //            Color::RGB(0, 255, 0),
    //            Color::RGB(0, 0, 255),
    //        ],
    //        vec![0.0, 0.2, 1.0],
    //    );

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut mandelbrot = MandelbrotSet::new(WIDTH, HEIGHT, 100);
    //mandelbrot.seed = Complex::new(-0.5, 0.0);
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
                    Some(Keycode::R) => {
                        if mandelbrot.max_iter > 100 {
                            mandelbrot.max_iter -= 100;
                            mandelbrot.calculate();
                        }
                    }
                    Some(Keycode::T) => {
                        mandelbrot.max_iter += 100;
                        mandelbrot.calculate();
                    }
                    Some(Keycode::H) => {
                        help_toggled = !help_toggled;
                        println!("Help toggled: {}", help_toggled);
                    }
                    _ => {}
                },
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => {
                        mandelbrot.move_to(x as f64, y as f64);
                        mandelbrot.calculate();
                    }

                    MouseButton::Right => {
                        println!(
                            "Value at cursor: {}",
                            mandelbrot.canvas[y as usize][x as usize]
                        )
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for y in 0..mandelbrot.height {
            for x in 0..mandelbrot.width {
                //canvas.set_draw_color(
                //    mandelbrot.val_to_color(mandelbrot.canvas[y as usize][x as usize]),
                //);
                canvas.set_draw_color(color_map.get_color(
                    mandelbrot.canvas[y as usize][x as usize] as f64,
                    mandelbrot.max_iter as f64,
                ));
                canvas.draw_point((x as i32, y as i32)).unwrap();
            }
        }

        render_text(
            &mut canvas,
            &font,
            &format!("Zoom: {:.2}", mandelbrot.zoom()),
            16,
            1 * 16 + 0 * font_size as i32,
            Color::RGB(255, 255, 255),
        );

        render_text(
            &mut canvas,
            &font,
            &format!("Max Iter: {}", mandelbrot.max_iter),
            16,
            2 * 16 + 1 * font_size as i32,
            Color::RGB(255, 255, 255),
        );

        let center = mandelbrot.center();
        render_text(
            &mut canvas,
            &font,
            &format!("Center x: {:.4}", center.0),
            16,
            3 * 16 + 2 * font_size as i32,
            Color::RGB(255, 255, 255),
        );
        render_text(
            &mut canvas,
            &font,
            &format!("Center y: {:.4}", center.1),
            16,
            4 * 16 + 3 * font_size as i32,
            Color::RGB(255, 255, 255),
        );

        render_text(
            &mut canvas,
            &font,
            &format!("Press H for help"),
            WIDTH as i32 - 16 * (32 + 1),
            16,
            Color::RGB(255, 255, 255),
        );

        if help_toggled {
            render_text(
                &mut canvas,
                &font,
                "WSAD to move around",
                16,
                HEIGHT as i32 - (5 * 48),
                Color::RGB(255, 255, 255),
            );
            render_text(
                &mut canvas,
                &font,
                "Left click to center on cursor",
                16,
                HEIGHT as i32 - (4 * 48),
                Color::RGB(255, 255, 255),
            );
            render_text(
                &mut canvas,
                &font,
                "Q/E to zoom out/in",
                16,
                HEIGHT as i32 - (3 * 48),
                Color::RGB(255, 255, 255),
            );
            render_text(
                &mut canvas,
                &font,
                "R/T to decrease/increase max iterations",
                16,
                HEIGHT as i32 - (2 * 48),
                Color::RGB(255, 255, 255),
            );
            render_text(
                &mut canvas,
                &font,
                "Right click to print iterations to escape at cursor",
                16,
                HEIGHT as i32 - (1 * 48),
                Color::RGB(255, 255, 255),
            );
        }

        canvas.present();
    }
}
