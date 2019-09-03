use crate::camera::Camera;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use nalgebra::{Point3, Vector3};

#[derive(Copy, Clone)]
pub struct Intersection<'a> {
    pub hit: Point3<f64>,
    pub normal: Vector3<f64>,
    pub material: &'a Material,
    pub object: &'a Sphere,
    pub distance: f64,
}

struct Hit<'a> {
    object: &'a Sphere,
    distance: f64,
}

pub struct Scene {
    pub camera: Camera,
    objects: Vec<Sphere>,
    light: usize
}

impl Scene {
    pub fn new(objects: Vec<Sphere>, camera: Camera, light: usize) -> Scene {
        Scene { objects, camera, light }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        self.objects.iter().fold(None, |closest, object| {
            let distance = object.intersection_distance(ray);
            match closest {
                None => Some(Hit{object, distance}),
                Some(ref hit) if distance < hit.distance => Some(Hit{object, distance}),
                c => c
            }
        }).map(|hit| {
            let point = ray.origin + (ray.direction * hit.distance);
            let normal = (point - hit.object.center()).normalize();
            Intersection {
                hit: point,
                normal,
                material: hit.object.material(),
                distance: hit.distance,
                object: hit.object
            }
        })
    }

    pub fn bg(&self, ray: &Ray) -> Vector3<f64> {
        Vector3::new(1.0, 0.0, 0.0)
    }

    pub fn light(&self) -> Sphere {
        self.objects[self.light]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::camera::Camera;
    use crate::material::Material;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
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
            0.2,
        );

        let objects = vec![
            Sphere::new(Point3::new(-1005.0, 0.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(1005.0, 0.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, -1003.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, 1003.0, -8.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, 0.0, -1010.0), 1000.0, blue_plastic),
            Sphere::new(Point3::new(0.0, 13.0, -8.0), 10.5, blue_plastic),
            Sphere::new(Point3::new(1.0, -2.0, -7.0), 1.0, blue_plastic),
            Sphere::new(Point3::new(-0.75, -2.0, -5.0), 1.0, blue_plastic),
        ];

        let camera = Camera::new(
            Point3::new(0.0, 0.0, 7.0),
            0.024,
            0.040,
            15.0,
            1.4,
            0.0,
            0.0,
        );

        let scene = Scene::new(objects, camera);
        let ray = Ray {
            origin: Point3::new(0.0, 0.0, 7.0),
            direction: Vector3::new(-0.13133105101029943, 0.23858981742286559, -0.96219907195063),
        };

        let intersection = scene.intersect(&ray).unwrap();
        assert_eq!(
            intersection.normal,
            Vector3::new(
                -0.0016543758341001802,
                -0.999994486641428,
                0.002879188661149867
            )
        );
    }
}
