extern crate cgmath;
extern crate minifb;
extern crate rayon;
extern crate rgb;

use helpers::*;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use rayon::prelude::*;

mod helpers;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const MAX_ITERATION: f64 = 1000.0;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rgb_buffer: Vec<(Col)> = vec![Col::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut x_offset = 0.0;
    let mut y_offset = 0.0;
    let mut zoom = 1.0;
    let mut bw = false;
    let mut hue = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                match t {
                    Key::Enter => (bw = !bw),
                    Key::Backspace => {
                        hue = !hue;
                        bw = false
                    }
                    _ => (),
                };
            }
        });

        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => (y_offset -= 0.15 * zoom),
                    Key::S => (y_offset += 0.15 * zoom),
                    Key::A => (x_offset -= 0.15 * zoom),
                    Key::D => (x_offset += 0.15 * zoom),
                    Key::Space => (zoom *= 1.5),
                    Key::LeftShift => (zoom *= 0.9),
                    _ => (),
                };
            }
        });

        rgb_buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, pixel)| {
                let uv = uv(i);
                let x0 = uv.x / HEIGHT as f64 * 3.0 * zoom - 2.0 * zoom + x_offset;
                let y0 = uv.y / HEIGHT as f64 * 3.0 * zoom - 1.5 * zoom + y_offset;

                let mut x = 0.0;
                let mut y = 0.0;
                let mut iteration = 0.0;

                while x * x + y * y <= 4.0 && iteration < MAX_ITERATION {
                    let xtemp = x * x - y * y + x0;
                    y = 2.0 * x * y + y0;
                    x = xtemp;

                    iteration += 1.0;
                }

                if bw {
                    if iteration < MAX_ITERATION {
                        let log_zn = (x * x + y * y).log(10.0) / 2.0;
                        let nu = (log_zn / (2 as f64).log(10.0)).log(10.0) / (2 as f64).log(10.0);
                        iteration = iteration + 1.0 - nu;
                    }
                    let col = Col::new(
                        (10.0 * iteration as f64 % MAX_ITERATION) / MAX_ITERATION,
                        (10.0 * iteration as f64 % MAX_ITERATION) / MAX_ITERATION,
                        (10.0 * iteration as f64 % MAX_ITERATION) / MAX_ITERATION,
                    );
                    pixel.r = col.r;
                    pixel.g = col.g;
                    pixel.b = col.b;
                } else if hue {
                    if iteration < MAX_ITERATION {
                        let log_zn = (x * x + y * y).log(10.0) / 2.0;
                        let nu = (log_zn / (2 as f64).log(10.0)).log(10.0) / (2 as f64).log(10.0);
                        iteration = iteration + 1.0 - nu;

                        let col = hsv((10.0 * iteration / MAX_ITERATION) % 1.0, 1.0, 1.0);
                        pixel.r = col.r;
                        pixel.g = col.g;
                        pixel.b = col.b;
                    } else {
                        let col = Col::new(0.0, 0.0, 0.0);
                        pixel.r = col.r;
                        pixel.g = col.g;
                        pixel.b = col.b;
                    }
                } else {
                    if iteration < MAX_ITERATION {
                        let log_zn = (x * x + y * y).log(10.0) / 2.0;
                        let nu = (log_zn / (2 as f64).log(10.0)).log(10.0) / (2 as f64).log(10.0);
                        iteration = iteration + 1.0 - nu;

                        let col = color_ramp((10.0 * iteration / MAX_ITERATION) % 1.0);
                        pixel.r = col.r;
                        pixel.g = col.g;
                        pixel.b = col.b;
                    } else {
                        let col = Col::new(0.0, 0.0, 0.0);
                        pixel.r = col.r;
                        pixel.g = col.g;
                        pixel.b = col.b;
                    }
                }
            });

        for (col_1, col_2) in rgb_buffer.iter().zip(buffer.iter_mut()) {
            *col_2 = col_to_rgb_u32(*col_1);
        }

        window.update_with_buffer(&buffer).unwrap();
    }
}
