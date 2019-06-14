use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console};
use nalgebra::{Vector3, Point3};

use crate::sphere::Sphere;
use crate::material::Material;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::tracer::Tracer;

mod sphere;
mod ray;
mod material;
mod camera;
mod scene;
mod tracer;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;

    let bright_light = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        1.0,
        Vector3::new(3000.0, 3000.0, 3000.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        0.0
    );

    let white_lambert = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.03, 0.03, 0.03),
        0.0,
        0.0
    );

    let blue_plastic = Material::new(
        Vector3::new(0.1, 0.1, 1.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.2
    );

    let red_plastic = Material::new(
        Vector3::new(1.0, 0.0, 0.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.2
    );

    let silver = Material::new(
        Vector3::new(0.972, 0.960, 0.915),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.972, 0.960, 0.915),
        0.9,
        1.0
    );

    let glass = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.6,
        1.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.0
    );

    let objects = vec![
        Sphere::new(Point3::new(-1005.0, 0.0, -8.0), 1000.0, blue_plastic),
        Sphere::new(Point3::new(1005.0, 0.0, -8.0), 1000.0, red_plastic),
        Sphere::new(Point3::new(0.0, -1003.0, -8.0), 1000.0, white_lambert),
        Sphere::new(Point3::new(0.0, 1003.0, -8.0), 1000.0, white_lambert),
        Sphere::new(Point3::new(0.0, 0.0, -1010.0), 1000.0, white_lambert),
        Sphere::new(Point3::new(0.0, 13.0, -8.0), 10.5, bright_light),
        Sphere::new(Point3::new(1.0, -2.0, -7.0), 1.0, silver),
        Sphere::new(Point3::new(-0.75, -2.0, -5.0), 1.0, glass)
    ];

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 7.0),
        0.024,
        0.040,
        15.0,
        1.4,
        0.0,
        0.0
    );

    let scene = Scene::new(objects, camera);
    let mut tracer = Tracer::new(scene, 10, 2.2, width as usize, height as usize);

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let mut data = vec![0u8; (width*height*4) as usize];
    
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut data),
            width,
            height
        ).unwrap();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        tracer.update(&mut data);
        context.put_image_data(&image_data, 0.0, 0.0)
            .expect("should have a value");

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
