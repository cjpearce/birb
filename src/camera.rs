use crate::ray::Ray;
use nalgebra::{Point3, Vector3};
use rand;
use std::f64;

pub struct Camera {
    position: Point3<f64>,
    sensor: f64,
    focal_length: f64,
    object_distance: f64,
    fstop: f64,
    aperture: f64,
    image_distance: f64,
    vertical_angle: f64,
    horizontal_angle: f64,
}

impl Camera {
    pub fn new(
        position: Point3<f64>,
        sensor: f64,
        focal_length: f64,
        focus: f64,
        fstop: f64,
        horizontal_angle: f64,
        vertical_angle: f64,
    ) -> Self {
        Self {
            position: position,
            sensor: sensor,
            focal_length: focal_length,
            object_distance: -focus,
            fstop: fstop,
            aperture: focal_length / fstop,
            image_distance: 1.0 / (1.0 / focal_length - 1.0 / -focus),
            vertical_angle: vertical_angle,
            horizontal_angle: horizontal_angle,
        }
    }

    pub fn ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        let sensor_point = self.sensor_point(x, y, width, height);
        let focus_point = self.focus_point(sensor_point);
        let aperture_point = self.aperture_point();
        let direction = (focus_point - aperture_point).normalize();
        Ray {
            origin: self.position,
            direction: self.rotated(direction),
        }
    }

    fn rotated(&self, direction: Vector3<f64>) -> Vector3<f64> {
        let x_axis = Vector3::new(-1.0, 0.0, 0.0);
        let y_axis = Vector3::new(0.0, -1.0, 0.0);
        let direction1 = angle_axis(&direction, self.vertical_angle, &x_axis);
        angle_axis(&direction1, self.horizontal_angle, &y_axis)
    }

    fn focus_point(&self, sensor_point: Point3<f64>) -> Vector3<f64> {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let sensor_to_lens = origin - sensor_point;
        let lens_world_ray = Ray {
            origin,
            direction: sensor_to_lens.normalize(),
        };
        let focus_ratio = self.object_distance / lens_world_ray.direction.z;
        lens_world_ray.direction * focus_ratio
    }

    fn sensor_point(&self, x: usize, y: usize, width: usize, height: usize) -> Point3<f64> {
        let aspect = width as f64 / height as f64;
        let vx = ((x as f64 + rand::random::<f64>()) / width as f64 - 0.5) * aspect;
        let vy = (y as f64 + rand::random::<f64>()) / height as f64 - 0.5;
        let sensor_x = -vx * self.sensor;
        let sensor_y = vy * self.sensor;
        Point3::new(sensor_x, sensor_y, self.image_distance)
    }

    fn aperture_point(&self) -> Vector3<f64> {
        let r_max = self.aperture / 2.0;
        let r = (rand::random::<f64>() * r_max * r_max).sqrt();
        let angle = rand::random::<f64>() * f64::consts::PI * 2.0;
        let x = r * angle.cos();
        let y = r * angle.sin();
        Vector3::new(x, y, 0.0)
    }
}

fn angle_axis(direction: &Vector3<f64>, angle: f64, axis: &Vector3<f64>) -> Vector3<f64> {
    let k = axis;
    let theta = angle * f64::consts::PI / 180.0;
    let first = direction * theta.cos();
    let second = (k.cross(direction)) * (theta.sin());
    let third = k * (k.dot(direction)) * (1.0 - theta.cos());
    first + second + third
}
