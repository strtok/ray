extern crate piston_window;
extern crate image as im;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use piston_window::*;

fn main() {
    pretty_env_logger::init();

    let opengl = OpenGL::V3_2;
    let (width, height) = (640, 480);
    let mut window: PistonWindow =
        WindowSettings::new("piston: paint", (width, height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    for y in (0..height-1).rev() {
        for x in 0..width-1 {
            let r: f32 = x as f32 / width as f32;
            let g: f32 = y as f32 / height as f32;
            let b: f32 = 0.2;
            let ir = (255.99 as f32 * r) as u8;
            let ig = (255.99 as f32 * g) as u8;
            let ib = (255.99 as f32 * b) as u8;
            canvas.put_pixel(x, y, im::Rgba([ir, ig, ib, 255]));
        }
    }

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
//        if draw {
//            if let Some(pos) = e.mouse_cursor_args() {
//                let (x, y) = (pos[0] as f32, pos[1] as f32);
//
//                if let Some(p) = last_pos {
//                    let (last_x, last_y) = (p[0] as f32, p[1] as f32);
//                    let distance = vec2_len(vec2_sub(p, pos)) as u32;
//
//                    for i in 0..distance {
//                        let diff_x = x - last_x;
//                        let diff_y = y - last_y;
//                        let delta = i as f32 / distance as f32;
//                        let new_x = (last_x + (diff_x * delta)) as u32;
//                        let new_y = (last_y + (diff_y * delta)) as u32;
//                        if new_x < width && new_y < height {
//                            canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
//                        };
//                    };
//                };
//
//                last_pos = Some(pos)
//            };
//
//        }
    }
}