use crate::canvas_renderer::CanvasRenderer;
use crate::tracer::Tracer;
use wasm_bindgen::prelude::*;

mod camera;
mod canvas_renderer;
mod material;
mod ray;
mod scene;
mod scene_loader;
mod sphere;
mod tracer;

#[wasm_bindgen(start)]
pub fn start() {
    let scene = scene_loader::load_scene("box").unwrap();
    let canvas_renderer = CanvasRenderer::new("canvas");

    let tracer = Tracer::new(
        scene,
        10,
        2.2,
        canvas_renderer.width(),
        canvas_renderer.height(),
    );

    canvas_renderer.start(tracer);
}
