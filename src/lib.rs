use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
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

    let material = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        0.0,
        Vector3::new(3000.0, 3000.0, 3000.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        0.0
    );

    let objects = vec![
        Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, &material)
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
    let tracer = Tracer::new(scene, 10, 2.2, width as usize, height as usize);

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

    let mut data = vec![0; (width*height*4) as usize];

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    let pixel_count = ( width*height ) as usize;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i >= pixel_count {
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        data[i*4 + 0] = 255;
        data[i*4 + 1] = 0;
        data[i*4 + 2] = 0;
        data[i*4 + 3] = 255;

        let image_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height).unwrap();
        context.put_image_data(&image_data, 0.0, 0.0);

        i += 1;

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}
