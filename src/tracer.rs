use crate::scene::Scene;
use crate::ray::Ray;
use nalgebra::Vector3;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use nalgebra::Point2;
use web_sys::console;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[derive(Clone)]
struct PixelInfo {
    color: Vector3<f32>,
    exposures: u32
}

pub struct Tracer {
    scene: Scene,
    bounces: u32,
    gamma: f32,
    width: usize,
    height: usize,
    exposures: Vec<PixelInfo>,
    pixels: Vec<u8>,
    index: u32,
    adaptive: f32,
    tick_ms: f64,
    traces: usize,
}

impl Tracer {
    pub fn new(
        scene: Scene,
        bounces: u32,
        gamma: f32,
        width: usize,
        height: usize
    ) -> Tracer {
        Tracer{
            scene,
            bounces,
            gamma,
            width,
            height,
            exposures: vec![PixelInfo{color: Vector3::new(0.0, 0.0, 0.0), exposures: 0}; width*height],
            pixels: vec![0; width*height*4],
            index: 0,
            adaptive: 0.25,
            tick_ms: 50.0,
            traces: 0
        }
    }

    pub fn update(&mut self) {
        let window = window();
        let performance = window.performance().expect("performance should be available");
        let start = performance.now();
        let end = start + self.tick_ms;

        loop {
            self.expose();
            if performance.now() > end {
                break;
            }
        }
    }

    fn pixel_for_index(&self, index: u32) -> Point2<usize> {
        let wrapped = index % (self.width * self.height) as u32;
        Point2::new( (wrapped as usize % self.width) as usize, (f32::floor(wrapped as f32 / self.width as f32)) as usize )
    }

    fn average_at(&self, pixel: Point2<usize>) -> Option<Vector3<f32>> {
        if pixel.x < 0 || pixel.x >= self.width || pixel.y < 0 || pixel.y >= self.height {
            return None;
        }
        
        let index = pixel.x + pixel.y * self.width;
        let exposure = self.exposures[index].clone();
        Some(exposure.color * (1.0 / exposure.exposures as f32))
    }

    fn expose(&mut self) {
        let pixel = self.pixel_for_index(self.index);
        let rgba_index = pixel.x + pixel.y * self.width;
        let limit = self.index as usize / (self.width * self.height) + 1;
        let mut last = self.average_at(pixel).unwrap();

        for _ in 0..limit {
            let light = self.trace(pixel);

            let noise = (light - last).norm() / (last.norm() + 1e-6);
            let average = ave(light);
            last = Vector3::new(average, average, average);

            self.exposures[rgba_index as usize].color += light;
            self.exposures[rgba_index as usize].exposures += 1;

            self.traces += 1;
            if noise < self.adaptive {
                break;
            }
        }

        self.color_pixel(pixel);
        self.index += 1;
    }

    fn trace(&mut self, pixel: Point2<usize>) -> Vector3<f32> {
        let mut ray = self.scene.camera.ray(
            pixel.x as f32,
            pixel.y as f32,
            self.width as f32,
            self.height as f32
        );
        let mut signal : Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);
        let mut energy = Vector3::new(0.0, 0.0, 0.0);

        for _ in 0..self.bounces {
            let intersection = self.scene.intersect(&ray);
            if intersection.is_none() {
                energy += self.scene.bg(&ray).component_mul(&signal);
                break;
            } else if let Some(intersect) = intersection {
                let light = intersect.material
                    .emit(&intersect.normal, &ray.direction);
                
                if let Some(light) = light {
                    energy += light.component_mul(&signal);
                }

                let max = signal.max();
                if dies(&mut signal, max) {
                    break;
                }

                let sample = intersect.material.bsdf(
                    intersect.normal,
                    ray.direction,
                    intersect.distance
                );

                if sample.is_none() {
                    break;
                } else if let Some(sample) = sample {
                    ray = Ray{origin: intersect.hit, direction: sample.direction};
                    signal = signal.component_mul(&sample.signal);
                }
            }
        }
        
        return energy
    }

    fn color_pixel(&mut self, pixel: Point2<usize>) {
        let index = (pixel.x + pixel.y * self.width) * 4;
        let average = self.average_at(pixel);
        if let Some(average) = average {
            self.pixels[index] = self.apply_gamma(average.x);
            self.pixels[index + 1] = self.apply_gamma(average.y);
            self.pixels[index + 2] = self.apply_gamma(average.z);
            self.pixels[index + 3] = 255;
        }
    }

    fn apply_gamma(&self, brightness: f32) -> u8 {
        ((brightness / 255.0).powf(1.0 / self.gamma) * 255.0).min(255.0) as u8
    }

    pub fn pixels(&mut self) -> &mut Vec<u8> {
        &mut self.pixels
    }
}

fn ave(v: Vector3<f32>) -> f32 {
    ( v.x + v.y + v.z ) / 3.0
}

fn dies(v: &mut Vector3<f32>, chance: f32) -> bool {
    if rand::random::<f32>() > chance {
        return true;
    }

    v.x /= chance;
    v.y /= chance;
    v.z /= chance;

    false
}