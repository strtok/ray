extern crate piston_window;
extern crate image;
extern crate pretty_env_logger;
extern crate core;

#[macro_use] extern crate log;
#[macro_use] extern crate approx;

mod vector;
mod ray;
mod rgb;

use piston_window::*;
use vector::Vector;
use ray::Ray;
use rgb::Rgb;
use image::ImageBuffer;

fn background(ray: &Ray) -> Rgb {
    let t = 0.5*(ray.direction.unit().y + 1.0);
    return
        (Rgb::new(1.0,1.0,1.0) * (1.0-t) + Rgb::new(0.5, 0.7, 1.0)*t)
    *   (Rgb::new(255.99, 255.99, 255.99));
}

fn raytrace<T>(canvas: &mut T)
    where T: image::GenericImage<Pixel=image::Rgba<u8>>
{
    let height = canvas.height();
    let width = canvas.width();

    let camera_origin = Vector::new(0.0, 0.0, 0.0);
    let viewport_origin = Vector::new(-2.0, -1.0, -1.0);
    let viewport_width = Vector::new(4.0, 0.0, 0.0);
    let viewport_height = Vector::new(0.0, 2.0, 0.0);

    for y in 0..height {
        for x in 0..width {
            let xp = x as f32 / width as f32;
            let yp = y as f32 / height as f32;
            let ray = Ray::new(camera_origin, viewport_origin + viewport_width*xp + viewport_height*yp);
            let color = background(&ray);
            canvas.put_pixel(x, height-y-1, image::Rgba([(color.r) as u8, (color.g) as u8, (color.b) as u8, 255]));
            //debug!("{},{} ({}. {}) -> {:?} -> {:?}", x, y, xp, yp, ray, color);
        }
    }
}

fn main() {
    pretty_env_logger::init();
    debug!("starting.");

    let opengl = OpenGL::V3_2;
    let (width, height) = (400, 200);
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