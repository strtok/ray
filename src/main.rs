extern crate piston_window;
extern crate image;
extern crate pretty_env_logger;
extern crate core;

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
use ray::Ray;
use rgb::Rgb;
use scene::*;
use sphere::Sphere;
use std::f32;
use vector::Vector;

fn raycast(ray: &Ray, scene: &Scene) -> Rgb {
    let bounds = (0.0, f32::MAX);

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
    let camera = Camera::new();

    for yp in 0..height {
        for xp in 0..width {
            let u = xp as f32 / width as f32;
            let v = yp as f32 / height as f32;
            let ray = camera.ray(u, v);
            let color = raycast(&ray, &scene);
            put_pixel(xp, height-yp-1, color.r, color.g, color.b);
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