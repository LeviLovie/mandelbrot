use num_complex::Complex;
use raylib::prelude::*;

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;
pub const TITLE: &str = "Mandelbrot Set";
pub const MAX_ITER: i32 = 64;
pub type Pixels = Vec<u8>;

fn mandelbrot(c: Complex<f64>) -> f64 {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..MAX_ITER {
        if z.norm() > 2.0 {
            let log_zn = (z.norm() as f64).ln().ln();
            return i as f64 + 1.0 - log_zn / 2.0;
        }
        z = z * z + c;
    }
    MAX_ITER as f64
}

fn render_mandelbrot(
    pixels: &mut Vec<u8>,
    width: i32,
    height: i32,
    zoom: f64,
    center: Complex<f64>,
) {
    pixels.resize((width * height) as usize, 0);

    let scale_x = 3.0 / zoom;
    let scale_y = 2.0 / zoom;

    for x in 0..width {
        for y in 0..height {
            let cx = scale_x * (x as f64 - width as f64 / 2.0) / (width as f64) + center.re;
            let cy = scale_y * (y as f64 - height as f64 / 2.0) / (height as f64) + center.im;
            let c = Complex::new(cx, cy);
            let i = mandelbrot(c) as i32;

            let color = if i == MAX_ITER {
                0
            } else {
                255 - i * 255 / MAX_ITER
            };
            let index = (x + y * width) as usize;
            if index < pixels.len() {
                pixels[index] = color as u8;
            }
        }
    }
}

fn draw_mandelbrot(
    pixels: &Pixels,
    d: &mut RaylibTextureMode<'_, RaylibDrawHandle<'_>>,
    width: i32,
    height: i32,
) {
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) as usize;
            let color = Color::new(pixels[index], pixels[index], pixels[index], 255);
            d.draw_pixel(x, y, color);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title(TITLE)
        .resizable()
        .build();

    let mut rerender = true;
    let mut changed = false;
    let mut width = WIDTH;
    let mut height = HEIGHT;
    let mut zoom = 1.0;
    let mut center = Complex::new(-0.5, 0.0);

    let mut texture = rl
        .load_render_texture(&thread, width as u32, height as u32)
        .expect("Failed to load render texture");

    let mut pixels = Pixels::new();

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            width = rl.get_screen_width();
            height = rl.get_screen_height();
            changed = true;
        }

        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_R) {
            rerender = true;
        }

        let mouse_position = rl.get_mouse_position();
        if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
            let x = mouse_position.x as f64;
            let y = mouse_position.y as f64;
            let scale_x = 3.0 / zoom;
            let scale_y = 2.0 / zoom;
            let cx = scale_x * (x - width as f64 / 2.0) / (width as f64) + center.re;
            let cy = scale_y * (y - height as f64 / 2.0) / (height as f64) + center.im;
            center = Complex::new(cx, cy);
            zoom *= 2.0;
            changed = true;
            rerender = true;
        } else if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_RIGHT) {
            let x = mouse_position.x as f64;
            let y = mouse_position.y as f64;
            let scale_x = 3.0 / zoom;
            let scale_y = 2.0 / zoom;
            let cx = scale_x * (x - width as f64 / 2.0) / (width as f64) + center.re;
            let cy = scale_y * (y - height as f64 / 2.0) / (height as f64) + center.im;
            center = Complex::new(cx, cy);
            zoom /= 2.0;
            changed = true;
            rerender = true;
        }

        if rerender {
            texture = rl
                .load_render_texture(&thread, width as u32, height as u32)
                .expect("Failed to load render texture");
        }

        let mut d = rl.begin_drawing(&thread);

        if rerender {
            println!("Rendering Mandelbrot Set...");
            let mut d = d.begin_texture_mode(&thread, &mut texture);
            render_mandelbrot(&mut pixels, width, height, zoom, center);
            draw_mandelbrot(&pixels, &mut d, width, height);
            println!("Mandelbrot Set rendered!");
            changed = false;
            rerender = false;
        }

        d.clear_background(Color::WHITE);
        d.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, texture.width() as f32, -texture.height() as f32),
            Rectangle::new(0.0, 0.0, width as f32, height as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE,
        );

        d.draw_rectangle_lines(
            mouse_position.x as i32 - width / 4,
            mouse_position.y as i32 - height / 4,
            width / 2,
            height / 2,
            Color::RED,
        );

        if changed {
            d.draw_text(
                "Press 'R' to rerender the Mandelbrot Set",
                10,
                10,
                20,
                Color::BLACK,
            );
        }
    }
}
