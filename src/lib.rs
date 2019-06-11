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

    let emitter_material = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
        0.0,
        Vector3::new(3000.0, 3000.0, 3000.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        0.0
    );

    let receiver_material = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.4, 0.4, 0.4),
        0.0,
        0.0
    );

    let objects = vec![
        Sphere::new(Point3::new(-1.0, 0.0, -10.0), 1.0, emitter_material),
        Sphere::new(Point3::new(1.0, 0.0, -10.0), 1.0, receiver_material)
    ];

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
        0.024,
        0.055,
        15.0,
        1.4,
        0.0,
        0.0
    );

    let scene = Scene::new(objects, camera);
    let tracer = Rc::new(RefCell::new(Tracer::new(scene, 10, 2.2, width as usize, height as usize)));
    let tracer_clone = tracer.clone();
    let tracer_clone_reader = tracer.clone();

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

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut tracer = tracer_clone.borrow_mut();
        tracer.update();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut tracer = tracer_clone_reader.borrow_mut();

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut tracer.pixels()), width, height).unwrap();

        context.put_image_data(&image_data, 0.0, 0.0).expect("should have a value");

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
