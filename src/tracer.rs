use crate::ray::Ray;
use crate::scene::Scene;
use nalgebra::Point2;
use nalgebra::Vector3;

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
    index: usize
}

impl Tracer {
    pub fn new(scene: Scene, bounces: u32, gamma: f64, width: usize, height: usize) -> Tracer {
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
            index: 0
        }
    }

    pub fn update(&mut self, pixels: &mut [u8]) {
        let limit = (self.index / (self.width * self.height)) + 1;
        self.expose(limit, pixels);
    }

    fn pixel_for_index(&self, index: usize) -> Point2<usize> {
        let wrapped = index % (self.width * self.height);
        Point2::new(wrapped % self.width, wrapped / self.width)
    }

    fn average_at(&self, pixel: &Point2<usize>) -> Vector3<f64> {
        self.exposures
            .get(pixel.x + pixel.y * self.width)
            .map(|e| e.color * (1.0 / f64::from(e.exposures)))
            .expect("Invalid pixel position")
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

                let max = signal.norm();
                if dies(&mut signal, max) {
                    break;
                }
            } else {
                energy += self.scene.bg(&ray).component_mul(&signal);
                break;
            }
        }

        energy
    }

    fn color_pixel(&mut self, pixel: Point2<usize>, pixels: &mut [u8]) {
        let index = (pixel.x + pixel.y * self.width) * 4;
        let average = self.apply_gamma(self.average_at(&pixel));
        pixels[index] = average.x as u8;
        pixels[index + 1] = average.y as u8;
        pixels[index + 2] = average.z as u8;
        pixels[index + 3] = 255;
    }

    fn apply_gamma(&self, pixel: Vector3<f64>) -> Vector3<f64> {
        (pixel / 255.0)
            .apply_into(|v| v.powf(self.reciprocal_gamma).min(1.0))
            * 255.0
    }
}

fn dies(v: &mut Vector3<f64>, chance: f64) -> bool {
    if rand::random::<f64>() > chance {
        true
    } else {
        *v /= chance;
        false
    }
}
