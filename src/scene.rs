use crate::sphere::Sphere;
use crate::ray::Ray;
use crate::material::Material;
use crate::camera::Camera;
use nalgebra::{ Vector3, Point3 };

pub struct Intersection<'a> {
    pub hit: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a Material,
    pub distance: f64,
}

struct Hit<'a> {
    object: &'a Sphere,
    distance: f64,
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

    pub fn bg(&self, ray: &Ray) -> Vector3<f64> {
        Vector3::new(1.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sphere::Sphere;
    use crate::camera::Camera;
    use crate::material::Material;
    use crate::ray::Ray;
    use nalgebra::{Point3, Vector3};

    #[test]
    fn intersection_returns_correct_result() {
        let blue_plastic = Material::new(
            Vector3::new(0.1, 0.1, 1.0),
            1.0,
            0.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.04, 0.04, 0.04),
            0.0,
            0.2
        );

        let objects = vec![
            Sphere::new(Point3::new(-1005.0, 0.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(1005.0, 0.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, -1003.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, 1003.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, 0.0, -1010.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, 13.0, -8.0), 10.5, blue_plastic),
            Sphere::new(Point3::new(1.0, -2.0, -7.0), 1.0, blue_plastic),
            Sphere::new(Point3::new(-0.75, -2.0, -5.0), 1.0, blue_plastic)
        ];

        let camera = Camera::new(
            Point3::new(0.0, 0.0, 7.0),
            0.024,
            0.040,
            15.0,
            1.4,
            0.0,
            0.0
        );

        let scene = Scene::new(objects, camera);
        let ray = Ray{
            origin: Point3::new(0.0, 0.0, 7.0),
            direction: Vector3::new( -0.13133105101029943, 0.23858981742286559, -0.96219907195063)
        };

        let intersection = scene.intersect(&ray).unwrap();
        assert_eq!(intersection.normal, Vector3::new(-0.0016543758341001802, -0.999994486641428, 0.002879188661149867));
    }
}