extern crate piston_window;
extern crate image;
extern crate pretty_env_logger;
extern crate core;

#[macro_use] extern crate log;
#[macro_use] extern crate approx;

mod vector;

use piston_window::*;
use vector::Vector;
use image::ImageBuffer;

fn raytrace<T>(canvas: &mut T)
    where T: image::GenericImage<Pixel=image::Rgba<u8>>
{
    let height = canvas.height();
    let width = canvas.width();
    for y in 0..height-1 {
        for x in 0..width-1 {
            let r: f32 = x as f32 / width as f32;
            let g: f32 = 1.0 - (y as f32 / height as f32);
            let b: f32 = 0.2;
            let ir = (255.99 as f32 * r) as u8;
            let ig = (255.99 as f32 * g) as u8;
            let ib = (255.99 as f32 * b) as u8;
            canvas.put_pixel(x, y, image::Rgba([ir, ig, ib, 255]));
        }
    }
}

fn main() {
    pretty_env_logger::init();
    debug!("starting.");

    let opengl = OpenGL::V3_2;
    let (width, height) = (640, 480);
    let mut window: PistonWindow =
        WindowSettings::new("piston: paint", (width, height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    let mut canvas = ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    raytrace(&mut canvas);

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }
}