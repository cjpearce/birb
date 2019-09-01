use crate::canvas_renderer::CanvasRenderer;
use crate::tracer::Tracer;
use wasm_bindgen::prelude::*;

mod camera;
pub mod canvas_renderer;
mod material;
mod ray;
mod scene;
mod sphere;
pub mod scene_loader;
pub mod tracer;

#[wasm_bindgen(start)]
pub fn start() {
    let scene = scene_loader::load_scene("box").unwrap();
    let canvas_renderer = CanvasRenderer::new("canvas");

    let tracer = Tracer::new(
        scene,
        6,
        1.0,
        canvas_renderer.width(),
        canvas_renderer.height(),
    );

    canvas_renderer.start(tracer);
}
