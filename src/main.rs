extern crate piston_window;
extern crate image;
extern crate pretty_env_logger;
extern crate core;
extern crate rand;
extern crate threadpool;

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
use std::sync::Arc;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
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

fn render_scene(width: u32, height: u32, camera: &Camera, scene: &Scene) -> Vec<Rgb>
{
    let mut image = vec![Rgb::new(0.0, 0.0, 0.0); (width * height) as usize];
    let mut rng = rand::thread_rng();

    for yp in 0..height {
        for xp in 0..width {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();
            let u = (xp as f32 + r1) / width as f32;
            let v = (yp as f32 + r2) / height as f32;
            let ray = camera.ray(u, v);
            let i = ((height - yp - 1) * width) + xp;
            image[i as usize] = raycast(&ray, &scene);
        }
    }
    image
}

fn render(width: u32, height: u32, scene: Arc<Scene>) -> Vec<Rgb>
{
    const NTHREADS: usize = 4;
    const NSAMPLES: usize = 100;

    let camera = Arc::new(Camera::new());
    let pool = ThreadPool::new(NTHREADS);
    let (tx, rx) = channel();

    // Distribute work to the thread pool. Each thread renders the
    // full scene one by one, sending the rendered buffer back to
    // the main thread to be merged into a final image.
    for _ in 0..NSAMPLES {
        let tx = tx.clone();
        let scene = Arc::clone(&scene);
        let camera = Arc::clone(&camera);

        pool.execute(move|| {
            tx.send(render_scene(width, height, &camera, &scene)).unwrap();
        });
    }

    // Average out the images produced by the worker threads to produce a final image
    let mut final_image = vec![Rgb::new(0.0, 0.0, 0.0); (width * height) as usize];
    for _ in 0..NSAMPLES {
        let image = rx.recv().unwrap();
        for (i, it) in image.iter().enumerate() {
            final_image[i] = &final_image[i] + it;
        }
    }
    for it in final_image.iter_mut() {
        it.r = it.r / NSAMPLES as f32;
        it.g = it.g / NSAMPLES as f32;
        it.b = it.b / NSAMPLES as f32;
    }

    final_image
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

    let scene = Arc::new(Scene {
        objects: vec![
            Object::Sphere(Sphere::new(Vector::new(0.0, 0.0, -1.0), 0.5)),
            Object::Sphere(Sphere::new(Vector::new(0.0, -100.5, -1.0), 100.00))
        ]
    });

    let image_buffer = render(width, height, scene);

    let mut canvas = ImageBuffer::new(width, height);
    for (i, it) in image_buffer.iter().enumerate() {
        let x = i as u32 % width;
        let y = i as u32 / width;
        canvas.put_pixel(x, y, image::Rgba([image_buffer[i].r as u8, image_buffer[i].g as u8, image_buffer[i].g as u8, 255]));
    }

    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

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