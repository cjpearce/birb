use nalgebra::{Vector3, Unit, geometry::Reflection};
use std::f64;
use rand;

pub struct BSDF {
    pub direction: Vector3<f64>,
    pub signal: Vector3<f64>,
}

#[derive(Copy, Clone)]
pub struct Material {
    color: Vector3<f64>,
    refraction: f64,
    transparency: f64,
    light: Vector3<f64>,
    frensel: Vector3<f64>,
    metal: f64,
    gloss: f64,
}

impl Material {
    pub fn new(
        color: Vector3<f64>,
        refraction: f64,
        transparency: f64,
        light: Vector3<f64>,
        frensel: Vector3<f64>,
        metal: f64,
        gloss: f64,
    ) -> Self {
        Self{
            color: color,
            refraction: refraction,
            transparency: transparency,
            light: light,
            frensel: frensel,
            metal: metal,
            gloss: gloss
        }
    }

    pub fn emit(&self, normal: &Vector3<f64>, direction: &Vector3<f64>) -> Option<Vector3<f64>> {
        if self.light.max() == 0f64 {
            None
        } else {
            Some(self.light * f64::max(normal.dot(&-direction), 0f64))
        }
    }

    pub fn bsdf(&self, normal: &Vector3<f64>, direction: &Vector3<f64>, length: f64) -> Option<BSDF> {
        let entering = direction.dot(&normal) < 0f64;
        if entering {
            let reflect = schilck(&normal, &direction, &self.frensel);
            let roughness = 1.0 - self.gloss;

            if rand::random::<f64>() <= ave(reflect) {
                let reflection = Reflection::new(Unit::new_normalize(normal.clone()), 0.0);
                let mut reflected1 = direction.clone();
                reflection.reflect(&mut reflected1);
                let reflected = random_in_cone(&reflected1, roughness);
                let tint = Vector3::new(1.0, 1.0, 1.0).lerp(&self.frensel, self.metal);
                Some(BSDF{
                    direction: reflected,
                    signal: tint,
                })
            } else if rand::random::<f64>() <= self.transparency {
                let transmitted = direction.refraction(&normal, 1.0, self.refraction).unwrap();
                Some(BSDF{
                    direction: transmitted,
                    signal: Vector3::new(1.0, 1.0, 1.0)
                })
            } else if rand::random::<f64>() <= self.metal {
                None
            } else {
                let diffused = random_in_cos_hemisphere(normal);
                let pdf = std::f64::consts::PI;

                Some(BSDF{
                    direction: diffused,
                    signal: self.color * (1.0 / pdf)
                })
            }
        } else if let Some(exited) = direction.refraction(
            &-normal, self.refraction, 1.0
        ) {
            let opacity = 1.0 - self.transparency;
            let volume = f64::min(opacity * length * length, 1.0);
            let tint = Vector3::new(1.0, 1.0, 1.0).lerp(&self.color, volume);
            Some(BSDF{ direction: exited, signal: tint })
        } else {
            None
        }
    }
}

fn ave(v: Vector3<f64>) -> f64 {
    ( v.x + v.y + v.z ) / 3.0
}

fn schilck(incident: &Vector3<f64>, normal: &Vector3<f64>, frensel: &Vector3<f64>) -> Vector3<f64> {
        let cos_incident = (-incident).dot(&normal);
        frensel + (
            (
                (Vector3::new(1.0, 1.0, 1.0) - frensel)
                * (1.0 - cos_incident).powf(5.0)
            )
        )
    }

fn random_in_cone(direction: &Vector3<f64>, width: f64) -> Vector3<f64> {
    let u = rand::random::<f64>();
    let v = rand::random::<f64>();
    let theta = width * 0.5 * f64::consts::PI * (1.0 - (2.0 * u.acos() / f64::consts::PI));
    let m1 = theta.sin();
    let m2 = theta.cos();
    let a = v * 2.0 * f64::consts::PI;
    let q = random_in_sphere();
    let s = direction.cross(&q);
    let t = direction.cross(&s);
    let mut d = Vector3::new(0.0, 0.0, 0.0);
    d += s * (m1 * a.cos());
    d += t * (m1 * a.sin());
    d += direction * m2;
    d.normalize()
}

fn from_angles(theta: f64, phi: f64) -> Vector3<f64> {
    Vector3::new(theta.cos() * phi.cos(), phi.sin(), theta.sin() * phi.cos())
}

fn random_in_sphere() -> Vector3<f64> {
    from_angles(
        rand::random::<f64>() * f64::consts::PI * 2.0,
        (rand::random::<f64>() * 2.0 - 1.0).asin()
    )
}

fn random_in_cos_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let u = rand::random::<f64>();
    let v = rand::random::<f64>();
    let r = u.sqrt();
    let theta = 2.0 * f64::consts::PI * v;
    let sphere_dir = random_in_sphere();
    let s = normal.cross(&sphere_dir).normalize();
    let t = normal.cross(&s);
    let mut d = Vector3::new(0.0, 0.0, 0.0);
    d += s * (r * theta.cos());
    d += t * (r * theta.sin());
    d += normal * (1.0 - u).sqrt();
    d
}

trait LightDirection {
    fn refraction(&self, normal: &Vector3<f64>, exterior_index: f64, interior_index: f64) -> Option<Vector3<f64>>;
}

impl LightDirection for Vector3<f64> {
    fn refraction(&self, normal: &Vector3<f64>, exterior_index: f64, interior_index: f64) -> Option<Vector3<f64>> {
        let ratio = exterior_index / interior_index;
        let n_dot_i = normal.dot(self);
        let k = 1.0 - ratio * ratio * (1.0 - n_dot_i * n_dot_i);
        if k < 0.0 {
            return None;
        } // total internal reflection

        let offset = normal*(ratio * n_dot_i + k.sqrt());
        Some(((self * ratio) - offset).normalize())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::{Vector3};

    #[test]
    fn schilck_is_correct() {
        let incident = Vector3::new(0.9999877074290066, 0.002070457097031252, 0.004505352182583419);
        let normal = Vector3::new(-0.42430229364657923, 0.17526903761586785, -0.8883964925974548);
        let frensel = Vector3::new(0.04, 0.04, 0.04);
        assert_eq!(schilck(&incident, &normal, &frensel), Vector3::new(0.09881546766725074, 0.09881546766725074, 0.09881546766725074))
    }
}