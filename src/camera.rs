use nalgebra::{Vector3, Point3};
use rand;
use std::f32;
use crate::ray::Ray;
use web_sys::console;

pub struct Camera {
    position: Point3<f32>,
    sensor: f32,
    focal_length: f32,
    object_distance: f32,
    fstop: f32,
    aperture: f32,
    image_distance: f32,
    vertical_angle: f32,
    horizontal_angle: f32,
}

impl Camera {
    pub fn new(
        position: Point3<f32>,
        direction: Vector3<f32>,
        sensor: f32,
        focal_length: f32,
        focus: f32,
        fstop: f32,
        horizontal_angle: f32,
        vertical_angle: f32
    ) -> Self {
        Self{
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

    pub fn ray(&self, x: f32, y: f32, width: f32, height: f32) -> Ray {
        let sensor_point = self.sensor_point(x, y, width, height);
        let focus_point = self.focus_point(sensor_point);
        let aperture_point = self.aperture_point();

        let direction = (focus_point - aperture_point).normalize();
        Ray{
            origin: self.position,
            direction: self.rotated(direction)
        }
    }

    fn rotated(&self, direction: Vector3<f32>) -> Vector3<f32> {
        let x_axis = Vector3::new(-1.0, 0.0, 0.0);
        let y_axis = Vector3::new(0.0, -1.0, 0.0);
        let direction1 = angle_axis(&direction, self.vertical_angle, &x_axis);
        angle_axis(&direction1, self.horizontal_angle, &y_axis)
    }

    fn focus_point(&self, sensor_point: Point3<f32>) -> Vector3<f32> {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let sensor_to_lens = origin - sensor_point;
        let lens_world_ray = Ray{
            origin,
            direction: sensor_to_lens.normalize()
        };
        let focus_ratio = self.object_distance / lens_world_ray.direction.z;
        lens_world_ray.direction * focus_ratio
    }

    fn sensor_point(&self, x: f32, y: f32, width: f32, height: f32) -> Point3<f32> {
        let aspect = width / height;
        let vx = ((x + rand::random::<f32>()) / width - 0.5) * aspect;
        let vy = (y + rand::random::<f32>()) / height - 0.5;
        let sensor_x = -vx * self.sensor;
        let sensor_y = vy * self.sensor;
        Point3::new(sensor_x, sensor_y, self.image_distance)
    }

    fn aperture_point(&self) -> Vector3<f32> {
        let r_max = self.aperture / 2.0;
        let r = (rand::random::<f32>() * r_max * r_max).sqrt();
        let angle = rand::random::<f32>() * f32::consts::PI * 2.0;
        let x = r * angle.cos();
        let y = r * angle.sin();
        Vector3::new(x, y, 0.0)
    }
}

fn angle_axis(direction: &Vector3<f32>, angle: f32, axis: &Vector3<f32>) -> Vector3<f32> {
    let k = axis;
    let theta = angle * f32::consts::PI / 180.0;
    let first = direction*theta.cos();
    let second = (k.cross(direction))*(theta.sin());
    let third = k*(k.dot(direction))*(1.0 - theta.cos());
    first + second + third
}