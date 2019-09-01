use nalgebra::{Point3, Vector3};
use std::f64;
use crate::onb::OrthonormalBasis;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>
}

pub trait DirectionExt {
    fn from_spherical(theta: f64, phi: f64) -> Self;
    fn random_in_sphere() -> Self;
    fn random_in_cos_hemisphere(u: f64, v: f64) -> Self;
    fn random_in_cone(direction: &Self, width: f64, u: f64, v: f64) -> Self;
    fn component_average(&self) -> f64;
    fn greyscale(&self) -> Self;
    fn refraction(
        &self,
        normal: &Vector3<f64>,
        exterior_index: f64,
        interior_index: f64,
    ) -> Option<Vector3<f64>>;
}

impl DirectionExt for Vector3<f64> {
    fn from_spherical(theta: f64, phi: f64) -> Self {
        Vector3::new(theta.cos() * phi.cos(), phi.sin(), theta.sin() * phi.cos())
    }

    fn random_in_sphere() -> Self {
        Self::from_spherical(
            rand::random::<f64>() * f64::consts::PI * 2.0,
            (rand::random::<f64>() * 2.0 - 1.0).asin(),
        )
    }

    fn random_in_cos_hemisphere(u: f64, v: f64) -> Self {
        let phi = 2.0 * std::f64::consts::PI * u;
        Vector3::new(
            phi.cos() * 2.0 * v.sqrt(),
            phi.sin() * 2.0 * v.sqrt(),
            (1.0 - v).sqrt()
        )
    }

    fn random_in_cone(direction: &Self, width: f64, u: f64, v: f64) -> Self {
        // let u = rand::random::<f64>();
        // let v = rand::random::<f64>();
        let theta = width * 0.5 * f64::consts::PI * (1.0 - (2.0 * u.acos() / f64::consts::PI));
        let m1 = theta.sin();
        let m2 = theta.cos();
        let a = v * 2.0 * f64::consts::PI;
        let q = Self::random_in_sphere();
        let s = direction.cross(&q);
        let t = direction.cross(&s);
        let mut d = Vector3::new(0.0, 0.0, 0.0);
        d += s * (m1 * a.cos());
        d += t * (m1 * a.sin());
        d += direction * m2;
        d.normalize()
    }

    fn component_average(&self) -> f64 {
        (self.x + self.y + self.z) / 3.0
    }

    fn greyscale(&self) -> Self {
        Self::new(self.mean(), self.mean(), self.mean())
    }

    fn refraction(
        &self,
        normal: &Vector3<f64>,
        exterior_index: f64,
        interior_index: f64,
    ) -> Option<Vector3<f64>> {
        let ratio = exterior_index / interior_index;
        let n_dot_i = normal.dot(self);
        let k = 1.0 - ratio * ratio * (1.0 - n_dot_i * n_dot_i);
        if k < 0.0 {
            return None;
        } // total internal reflection

        let offset = normal * (ratio * n_dot_i + k.sqrt());
        Some(((self * ratio) - offset).normalize())
    }
}