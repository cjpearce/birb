use crate::ray::Ray;
use crate::scene::Scene;
use nalgebra::Point2;
use nalgebra::Vector3;
use web_sys::console;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[derive(Clone)]
struct PixelInfo {
    color: Vector3<f64>,
    exposures: u32,
}

pub struct Tracer {
    scene: Scene,
    bounces: u32,
    reciprocal_gamma: f64,
    width: usize,
    height: usize,
    exposures: Vec<PixelInfo>,
    index: usize,
    tick_ms: f64,
    traces: usize,
    performance: web_sys::Performance,
}

impl Tracer {
    pub fn new(scene: Scene, bounces: u32, gamma: f64, width: usize, height: usize) -> Tracer {
        let performance = window()
            .performance()
            .expect("performance should be available");
        Tracer {
            scene,
            bounces,
            reciprocal_gamma: 1.0 / gamma,
            width,
            height,
            exposures: vec![
                PixelInfo {
                    color: Vector3::new(0.0, 0.0, 0.0),
                    exposures: 0
                };
                width * height
            ],
            index: 0,
            tick_ms: 50.0,
            traces: 0,
            performance: performance,
        }
    }

    pub fn update(&mut self, pixels: &mut [u8]) {
        let start = self.performance.now();
        let end = start + self.tick_ms;

        loop {
            let limit = (self.index / (self.width * self.height)) + 1;
            self.expose(limit, pixels);
            if self.performance.now() > end {
                break;
            }
        }
    }

    fn pixel_for_index(&self, index: usize) -> Point2<usize> {
        let wrapped = index % (self.width * self.height);
        Point2::new(wrapped % self.width, wrapped / self.width)
    }

    fn average_at(&self, pixel: &Point2<usize>) -> Option<Vector3<f64>> {
        if pixel.x >= self.width || pixel.y >= self.height {
            return None;
        }

        self.exposures
            .get(pixel.x + pixel.y * self.width)
            .map(|e| e.color * (1.0 / e.exposures as f64))
    }

    fn expose(&mut self, limit: usize, pixels: &mut [u8]) {
        let pixel = self.pixel_for_index(self.index);
        let rgba_index = pixel.x + pixel.y * self.width;

        for _ in 0..limit {
            let sample = self.trace(&pixel);
            self.exposures[rgba_index as usize].color += sample;
            self.exposures[rgba_index as usize].exposures += 1;
        }

        self.color_pixel(pixel, pixels);
        self.index += 1;
    }

    fn trace(&mut self, pixel: &Point2<usize>) -> Vector3<f64> {
        let mut ray = self
            .scene
            .camera
            .ray(pixel.x, pixel.y, self.width, self.height);

        let mut signal = Vector3::new(1.0, 1.0, 1.0);
        let mut energy = Vector3::new(0.0, 0.0, 0.0);

        for _ in 0..self.bounces {
            if let Some(intersect) = self.scene.intersect(&ray) {
                if let Some(light) = intersect.material.emit(&intersect.normal, &ray.direction) {
                    energy += light.component_mul(&signal);
                }

                let max = signal.norm();
                if dies(&mut signal, max) {
                    break;
                }

                if let Some(sample) =
                    intersect
                        .material
                        .bsdf(&intersect.normal, &ray.direction, intersect.distance)
                {
                    ray = Ray {
                        origin: intersect.hit,
                        direction: sample.direction,
                    };
                    signal = signal.component_mul(&sample.signal);
                } else {
                    break;
                }
            } else {
                energy += self.scene.bg(&ray).component_mul(&signal);
                break;
            }
        }

        return energy;
    }

    fn color_pixel(&mut self, pixel: Point2<usize>, pixels: &mut [u8]) {
        let index = (pixel.x + pixel.y * self.width) * 4;
        let average = self.average_at(&pixel);
        if let Some(average) = average {
            pixels[index] = self.apply_gamma(average.x);
            pixels[index + 1] = self.apply_gamma(average.y);
            pixels[index + 2] = self.apply_gamma(average.z);
            pixels[index + 3] = 255;
        }
    }

    fn apply_gamma(&self, brightness: f64) -> u8 {
        ((brightness / 255.0).powf(self.reciprocal_gamma) * 255.0).min(255.0) as u8
    }
}

fn dies(v: &mut Vector3<f64>, chance: f64) -> bool {
    if rand::random::<f64>() > chance {
        true
    } else {
        v.x /= chance;
        v.y /= chance;
        v.z /= chance;
        false
    }
}
