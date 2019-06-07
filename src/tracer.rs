use crate::scene::Scene;
use nalgebra::Vector3;


#[derive(Clone)]
struct PixelInfo {
    color: Vector3<f32>,
    exposures: u32
}

pub struct Tracer<'a> {
    scene: Scene<'a>,
    bounces: u32,
    gamma: f32,
    width: usize,
    height: usize,
    exposures: Vec<PixelInfo>,
    pixels: Vec<u8>,
    index: u32,
    adaptive: f32,
}

impl <'a> Tracer<'a> {
    pub fn new(
        scene: Scene<'a>,
        bounces: u32,
        gamma: f32,
        width: usize,
        height: usize
    ) -> Tracer<'a> {
        Tracer{
            scene,
            bounces,
            gamma,
            width,
            height,
            exposures: vec![PixelInfo{color: Vector3::new(0.0, 0.0, 0.0), exposures: 0}; width*height],
            pixels: vec![0; width*height],
            index: 0,
            adaptive: 0.25
        }
    }
}

