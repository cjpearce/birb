use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::material::Material;
use crate::camera::Camera;
use nalgebra::{ Vector3, Point3 };


pub struct Intersection<'a> {
    hit: Point3<f32>,
    normal: Vector3<f32>,
    material: &'a Material,
    distance: f32,
}

struct Hit<'a> {
    object: &'a Sphere<'a>,
    distance: f32,
}

pub struct Scene<'a> {
    camera: Camera,
    objects: Vec<Sphere<'a>>
}

impl <'a> Scene<'a> {
    pub fn new(objects: Vec<Sphere<'a>>, camera: Camera) -> Scene<'a> {
        Scene{ objects, camera }
    }

    pub fn intersect(&'a self, ray: &Ray) -> Option<Intersection> {
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

    pub fn bg(ray: &Ray) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
