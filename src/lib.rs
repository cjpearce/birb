use nalgebra::{Vector3, Point3};

use crate::sphere::Sphere;
use crate::material::Material;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::tracer::Tracer;
use crate::canvas_renderer::CanvasRenderer;
use wasm_bindgen::prelude::*;

mod sphere;
mod ray;
mod material;
mod camera;
mod scene;
mod tracer;
mod canvas_renderer;

#[wasm_bindgen(start)]
pub fn start() {
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
    let canvas_renderer = CanvasRenderer::new("canvas");

    let tracer = Tracer::new(
        scene,
        10,
        2.2,
        canvas_renderer.width(),
        canvas_renderer.height()
    );

    canvas_renderer.start(tracer);
}
