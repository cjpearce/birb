use nalgebra::{Vector3, Point3};

use crate::sphere::Sphere;
use crate::material::Material;
use crate::scene::Scene;
use crate::camera::Camera;

pub fn load_scene(name: &str) -> Option<Scene> {
  match name {
    "box" => Some(load_box_scene()),
    "spheres" => Some(load_spheres_scene()),
    _ => None
  }
}

fn load_spheres_scene() -> Scene {
  let bright_light = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        1.0,
        Vector3::new(700.0, 700.0, 700.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        0.0
    );

    let white_lambert = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.03, 0.03, 0.03),
        0.0,
        0.0
    );

    let blue_plastic = Material::new(
        Vector3::new(0.1, 0.1, 1.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.2
    );

    let red_plastic = Material::new(
        Vector3::new(1.0, 0.0, 0.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.2
    );

    let silver = Material::new(
        Vector3::new(0.972, 0.960, 0.915),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.972, 0.960, 0.915),
        0.9,
        1.0
    );

    let gold = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.022, 0.782, 0.344),
        1.0,
        0.7
    );

    let glass = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.6,
        1.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.0
    );

    let green_glass = Material::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.52,
        0.95,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.05, 0.05, 0.05),
        0.0,
        1.0
    );

    let objects = vec![
        Sphere::new(0, Point3::new(-3.3, 1.0, -4.3), 1.0, gold),
        Sphere::new(1, Point3::new(-1.1, 1.0, -5.0), 1.0, blue_plastic),
        Sphere::new(2, Point3::new(1.0, 1.0, -5.0), 1.0, silver),
        Sphere::new(3, Point3::new(3.2, 1.0, -4.6), 1.0, green_glass),
        Sphere::new(4, Point3::new(0.5, -1000.0, -8.0), 1000.0, white_lambert),
        Sphere::new(5, Point3::new(-8.0, 3.0, -1.0), 2.0, bright_light)
    ];

    let camera = Camera::new(
        Point3::new(0.0, 6.0, 8.0),
        0.024,
        0.055,
        14.0,
        1.4,
        0.0,
        25.0
    );

    Scene::new(objects, camera, 5)
}

fn load_box_scene() -> Scene {
  let bright_light = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        1.0,
        Vector3::new(355.0, 355.0, 355.0),
        Vector3::new(0.0, 0.0, 0.0),
        0.0,
        0.0
    );

    let white_lambert = Material::new(
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.03, 0.03, 0.03),
        0.0,
        0.0
    );

    let blue_plastic = Material::new(
        Vector3::new(0.1, 0.1, 1.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.2
    );

    let red_plastic = Material::new(
        Vector3::new(1.0, 0.0, 0.0),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.2
    );

    let silver = Material::new(
        Vector3::new(0.972, 0.960, 0.915),
        1.0,
        0.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.972, 0.960, 0.915),
        0.9,
        1.0
    );

    let glass = Material::new(
        Vector3::new(0.0, 0.0, 0.0),
        1.6,
        1.0,
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.04, 0.04, 0.04),
        0.0,
        0.0
    );

    let objects = vec![
        Sphere::new(0, Point3::new(-1005.0, 0.0, -8.0), 1000.0, blue_plastic),
        Sphere::new(1, Point3::new(1005.0, 0.0, -8.0), 1000.0, red_plastic),
        Sphere::new(2, Point3::new(0.0, -1003.0, -8.0), 1000.0, white_lambert),
        Sphere::new(3, Point3::new(0.0, 1003.0, -8.0), 1000.0, white_lambert),
        Sphere::new(4, Point3::new(0.0, 0.0, -1010.0), 1000.0, white_lambert),
        Sphere::new(5, Point3::new(0.0, 13.0, -8.0), 10.5, bright_light),
        Sphere::new(6, Point3::new(1.0, -2.0, -7.0), 1.0, silver),
        Sphere::new(7, Point3::new(-0.75, -2.0, -5.0), 1.0, glass)
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

    Scene::new(objects, camera, 5)
}
