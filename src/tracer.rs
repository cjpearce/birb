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
        let limit = (self.index / (self.width * self.height)) + 1;
        let pixel = self.pixel_for_index(self.index);
        self.expose(pixel, limit);

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

    fn expose(&mut self, pixel: Point2<usize>, limit: usize) {
        let rgba_index = pixel.x + pixel.y * self.width;
        for _ in 0..limit {
            let ray = self
                .scene
                .camera
                .ray(pixel.x, pixel.y, self.width, self.height);
            let sample = self.trace(ray, 4, 0, true);
            self.exposures.add_sample(rgba_index as usize, sample);
        }
    }

    fn sample_lights(&self, intersection: &Intersection, direction: Vector3<f64>) -> Vector3<f64> {
        let light = self.scene.light();

        // get bounding sphere center and radius
        let center = light.center();
        let radius = light.radius();

        // get random point in disk
        let point = loop {
            let x = rand::random::<f64>() * 2.0 - 1.0;
            let y = rand::random::<f64>() * 2.0 - 1.0;
            if x*x + y*y <= 1.0 {
                let l = (center - intersection.hit).normalize();
                let u = l.cross(&Vector3::random_in_sphere()).normalize();
                let v = l.cross(&u);

                break center + (u * x * radius) + (v * y * radius);
            }
        };

        // construct ray toward light point
        let ray = Ray{
            origin: intersection.hit,
            direction: (point - intersection.hit).normalize()
        };

        // check for light visibility
        let hit = self.scene.intersect(&ray);
        if hit.is_none() || hit.unwrap().object != light {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        // compute solid angle (hemisphere coverage)
        let hyp = (center - intersection.hit).norm();
        let opp = radius;
        let theta = (opp / hyp).asin();
        let adj = opp / theta.tan();
        let d = theta.cos() * adj;
        let r = theta.sin() * adj;

        let coverage = if hyp < opp {
            1.0
        } else {
            f64::min((r * r) / (d * d), 1.0)
        };

        light.material().emit() * coverage
    }

    fn trace(&mut self, ray: Ray, samples: u32, depth: u32, emmission: bool) -> Vector3<f64> {

        if depth == self.bounces {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(intersect) = self.scene.intersect(&ray) {
            let mut energy = Vector3::new(0.0, 0.0, 0.0);
            let n = f64::from(samples).sqrt() as u32;

            if intersect.material.emit().norm() > 0.1 && !emmission {
                return Vector3::new(0.0, 0.0, 0.0);
            }

            energy += intersect.material.emit() * f64::from(n*n);

            for u in 0..n {
                for v in 0..n {
                    let fu = (f64::from(u) + rand::random::<f64>()) / f64::from(n);
                    let fv = (f64::from(v) + rand::random::<f64>()) / f64::from(n);

                    let sample = intersect
                        .material
                        .bsdf(&intersect.normal, &ray.direction, intersect.distance, fu, fv);

                    let ray = Ray{origin: intersect.hit, direction: sample.direction};
                    let indirect = self.trace(ray, 1, depth + 1, sample.reflected);

                    let direct = self.sample_lights(&intersect, ray.direction);
                    if !sample.reflected {
                        energy += direct.component_mul(&sample.signal); 
                    }
                    energy += indirect.component_mul(&sample.signal);
                }
            }

            energy / f64::from(n*n)
        } else {
            self.scene.bg(&ray)
        }
    }
}