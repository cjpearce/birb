use crate::ray::{Ray, DirectionExt};
use crate::scene::{Scene, Intersection};
use nalgebra::Point2;
use nalgebra::Vector3;

#[derive(Clone)]
struct PixelInfo {
    color: Vector3<f64>,
    exposures: u32,
}

impl PixelInfo {
    fn color(&self, reciprocal_gamma: f64) -> Vector3<u8> {
        let color = self.color * (1.0 / f64::from(self.exposures));
        let color = (color / 255.0)
            .apply_into(|v| v.powf(reciprocal_gamma).min(1.0))
            * 255.0;
        Vector3::new(color.x as u8, color.y as u8, color.z as u8)
    }
}

struct Exposures(Vec<PixelInfo>, f64);

impl Exposures {
    fn new(length: usize, reciprocal_gamma: f64) -> Self {
        let default = PixelInfo {
            color: Vector3::new(0.0, 0.0, 0.0),
            exposures: 0
        };
        Self(vec![default; length], reciprocal_gamma)
    }

    fn add_sample(&mut self, position: usize, sample: Vector3<f64>) {
        self.0[position].color += sample;
        self.0[position].exposures += 1;
    }

    fn color_at(&self, position: usize) -> Vector3<u8> {
        self.0[position].color(self.1)
    }
}

pub struct Tracer {
    scene: Scene,
    bounces: u32,
    width: usize,
    height: usize,
    exposures: Exposures,
    index: usize
}

impl Tracer {
    pub fn new(scene: Scene, bounces: u32, gamma: f64, width: usize, height: usize) -> Tracer {
        Tracer {
            scene,
            bounces,
            width,
            height,
            exposures: Exposures::new(width*height, 1.0/gamma),
            index: 0
        }
    }

    pub fn update(&mut self, pixels: &mut [u8]) {
        let pixel = self.pixel_for_index(self.index);
        self.expose(pixel);

        let color = self.exposures.color_at(pixel.x + pixel.y * self.width);

        let index = (pixel.x + pixel.y * self.width) * 4;
        pixels[index] = color.x;
        pixels[index + 1] = color.y;
        pixels[index + 2] = color.z;
        pixels[index + 3] = 255;

        self.index += 1;
    }

    fn pixel_for_index(&self, index: usize) -> Point2<usize> {
        let wrapped = index % (self.width * self.height);
        Point2::new(wrapped % self.width, wrapped / self.width)
    }

    fn expose(&mut self, pixel: Point2<usize>) {
        let rgba_index = pixel.x + pixel.y * self.width;
        let ray = self
            .scene
            .camera
            .ray(pixel.x, pixel.y, self.width, self.height);
        let sample = self.trace(ray, 1);
        self.exposures.add_sample(rgba_index as usize, sample);
    }

    fn trace(&mut self, mut ray: Ray, samples: u32) -> Vector3<f64> {
        let mut total_energy = Vector3::new(0.0, 0.0, 0.0);
        let n = f64::from(samples).sqrt() as u32;
        for u in 0..n {
            for v in 0..n {
                let mut energy = Vector3::new(0.0, 0.0, 0.0);
                let mut signal = Vector3::new(1.0, 1.0, 1.0);

                for bounce in 0..self.bounces {
                    let (fu, fv) = if bounce == 0 {
                        ((f64::from(u) + rand::random::<f64>()) / f64::from(n),
                        (f64::from(v) + rand::random::<f64>()) / f64::from(n))
                    } else {
                        (rand::random::<f64>(), rand::random::<f64>())
                    };
            
                    if let Some(intersect) = self.scene.intersect(&ray) {
                        let sample = intersect
                            .material
                            .bsdf(&intersect.normal, &ray.direction, intersect.distance, fu, fv, &self.scene, &intersect);

                        ray.origin = intersect.hit;
                        ray.direction = sample.direction;
                        energy += intersect.material.emit().component_mul(&signal);
                        signal = signal.component_mul(&sample.signal);
                    } else {
                        energy += self.scene.bg(&ray).component_mul(&signal);
                        break;
                    }
                }

                total_energy += energy;
            }
        }

        total_energy / f64::from(n*n)
    }
}