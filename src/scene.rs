use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::material::Material;
use crate::camera::Camera;
use nalgebra::{ Vector3, Point3 };


pub struct Intersection<'a> {
    pub hit: Point3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Material,
    pub distance: f32,
}

struct Hit<'a> {
    object: &'a Sphere,
    distance: f32,
}

pub struct Scene {
    pub camera: Camera,
    objects: Vec<Sphere>
}

impl Scene {
    pub fn new(objects: Vec<Sphere>, camera: Camera) -> Scene {
        Scene{ objects, camera }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest = None;
        for obj in self.objects.iter() {
            let dist = obj.intersection_distance(ray);
            if closest.is_none() {
                closest = Some(Hit{object: obj, distance: dist});
            } else if let Some(Hit{distance: close_dist, ..}) = closest {
                if dist < close_dist {
                    closest = Some(Hit{object: obj, distance: dist});
                }
            }
        }

        if let Some(Hit{object: obj, distance: dist}) = closest {
            let point = ray.origin + (ray.direction * dist);
            let normal = (point - obj.center()).normalize();
            Some(Intersection{
                hit: point,
                normal,
                material: obj.material(),
                distance: dist
            })
        } else {
            None
        }
    }

    pub fn bg(&self, ray: &Ray) -> Vector3<f32> {
        Vector3::new(1.0, 0.0, 0.0)
    }
}
