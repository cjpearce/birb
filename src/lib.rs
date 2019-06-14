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
mod scene_loader;

#[wasm_bindgen(start)]
pub fn start() {
    let scene = scene_loader::load_scene("spheres").unwrap();
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
