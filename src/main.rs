extern crate piston_window;
extern crate image;
extern crate pretty_env_logger;
extern crate core;
extern crate rand;

#[macro_use] extern crate log;
#[macro_use] extern crate approx;

mod camera;
mod ray;
mod rgb;
mod scene;
mod sphere;
mod vector;

use camera::Camera;
use image::ImageBuffer;
use piston_window::*;
use rand::prelude::*;
use ray::Ray;
use rgb::Rgb;
use scene::*;
use sphere::Sphere;
use std::f32;
use vector::Vector;

fn raycast(ray: &Ray, scene: &Scene) -> Rgb {
    let bounds = (0.001, f32::MAX);

    // Intersection with closest object
    if let Some(intersection) = scene.objects.iter().filter_map(|obj| {
        match obj {
            Object::Sphere(sphere) => {
                ray.intersects(sphere, bounds)
            }
        }
    }).min_by(|r1, r2| {
        r1.t.partial_cmp(&r2.t).unwrap()
    }) {
        let target = intersection.point + intersection.normal + Vector::random_unit();
        return raycast(&Ray::new(intersection.point, target-intersection.point), scene) * 0.5;
    }

    // Otherwise background color
    let t = 0.5*(ray.direction.unit().y + 1.0);
    let color = Rgb::new(1.0, 1.0, 1.0) * (1.0 - t) + Rgb::new(0.5, 0.7, 1.0)*t;
    return Rgb::new(color.r.sqrt() * 255.0, color.g.sqrt() * 255.0, color.b.sqrt() * 255.0);
}

fn render<T>(height: u32, width: u32, scene: &Scene, put_pixel: &mut T)
    where T: FnMut(u32, u32, f32, f32, f32)
{
    let camera = Camera::new();
    let nsamples = 25;

    for yp in 0..height {
        for xp in 0..width {
            let mut color = Rgb::new(0.0, 0.0, 0.0);
            for sample in 0..nsamples {
                let u = (xp as f32 + random::<f32>()) / width as f32;
                let v = (yp as f32 + random::<f32>()) / height as f32;
                let ray = camera.ray(u, v);
                color = color + raycast(&ray, &scene);
            }
            color = color / nsamples as f32;
            put_pixel(xp, height - yp - 1, color.r, color.g, color.b);
        }
    }
}

fn main() {
    pretty_env_logger::init();
    debug!("starting.");

    let opengl = OpenGL::V3_2;
    let (width, height) = (1400, 700);
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