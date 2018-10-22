extern crate piston_window;
extern crate image;
extern crate pretty_env_logger;
extern crate core;

#[macro_use] extern crate log;
#[macro_use] extern crate approx;

mod vector;
mod ray;
mod rgb;
mod scene;
mod sphere;

use piston_window::*;
use image::ImageBuffer;
use ray::Ray;
use rgb::Rgb;
use scene::*;
use sphere::Sphere;
use std::cmp::Ordering;
use std::f32;
use vector::Vector;

fn raycast(ray: &Ray, scene: &Scene) -> Rgb {
    let bounds = (0.0, f32::MAX);

    // Intersection with closest object
    if let Some(intersection) = scene.objects.iter().filter_map(|obj| {
        match obj {
            Object::Sphere(sphere) => {
                return ray.intersects(sphere, bounds);
            }
        }
    }).min_by(|r1, r2| {
        r1.t.partial_cmp(&r2.t).unwrap()
    }) {
        let normal = (intersection.normal + Vector::new(1.0,1.0,1.0)) * 0.5;
        let rgb = Rgb::new(normal.x*255.0, normal.y*255.0, normal.z*255.0);
        return rgb;
    }

    // Otherwise background color
    let t = 0.5*(ray.direction.unit().y + 1.0);
    return
        (Rgb::new(1.0,1.0,1.0) * (1.0-t) + Rgb::new(0.5, 0.7, 1.0)*t)
    *   (Rgb::new(255.0, 255.0, 255.0));
}

fn render<T>(height: u32, width: u32, scene: &Scene, put_pixel: &mut T)
    where T: FnMut(u32, u32, f32, f32, f32)
{
    let camera_origin = Vector::new(0.0, 0.0, 0.0);
    let viewport_origin = Vector::new(-2.0, -1.0, -1.0);
    let viewport_width = Vector::new(4.0, 0.0, 0.0);
    let viewport_height = Vector::new(0.0, 2.0, 0.0);

    for y in 0..height {
        for x in 0..width {
            let xp = x as f32 / width as f32;
            let yp = y as f32 / height as f32;
            let ray = Ray::new(camera_origin, viewport_origin + viewport_width*xp + viewport_height*yp);
            let color = raycast(&ray, &scene);
            put_pixel(x, height-y-1, color.r, color.g, color.b);
        }
    }
}

fn main() {
    pretty_env_logger::init();
    debug!("starting.");

    let opengl = OpenGL::V3_2;
    let (width, height) = (400, 200);
    let mut window: PistonWindow =
        WindowSettings::new("ray", (width, height))
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


    let scene = Scene { objects: vec![
        Object::Sphere(Sphere::new(Vector::new(0.0,0.0,-1.0), 0.5)),
        Object::Sphere(Sphere::new(Vector::new(0.0,-100.5,-1.0), 100.00))
    ]};

    render(canvas.height(), canvas.width(), &scene,&mut |x, y, r, g, b| {
        canvas.put_pixel(x, y, image::Rgba([r as u8, g as u8, b as u8, 255]));
    });

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