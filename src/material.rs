use nalgebra::{geometry::Reflection, Unit, Vector3};
use rand;
use std::f64;

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
        Self {
            color,
            refraction,
            transparency,
            light,
            frensel,
            metal,
            gloss,
        }
    }

    pub fn emit(&self, normal: &Vector3<f64>, direction: &Vector3<f64>) -> Option<Vector3<f64>> {
        if self.light.max() == 0f64 {
            None
        } else {
            Some(self.light * f64::max(normal.dot(&-direction), 0f64))
        }
    }

    pub fn bsdf(
        &self,
        normal: &Vector3<f64>,
        direction: &Vector3<f64>,
        length: f64,
    ) -> Option<BSDF> {
        let entering = direction.dot(&normal) < 0f64;
        if entering {
            let mut test = FilteredProbabilityTest::new();
            if test.or(self.schilck(&normal, &direction).average()) {
                Some(self.reflected(*direction, &normal))
            } else if test.or(self.transparency) {
                Some(self.refracted_entry(*direction, &normal))
            } else if test.or(self.metal) {
                None
            } else {
                Some(self.diffused(&normal))
            }
        } else if let Some(exited) = direction.refraction(&-normal, self.refraction, 1.0) {
            Some(self.refracted_exit(exited, length))
        } else {
            None
        }
    }

    fn schilck(&self, incident: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        let cos_incident = (-incident).dot(&normal);
        self.frensel + ((Vector3::new(1.0, 1.0, 1.0) - self.frensel) * (1.0 - cos_incident).powf(5.0))
    }

    fn diffused(&self, normal: &Vector3<f64>) -> BSDF {
        let pdf = std::f64::consts::PI;
        BSDF {
            direction: random_in_cos_hemisphere(normal),
            signal: self.color * (1.0 / pdf),
        }
    }

    fn reflected(&self, mut direction: Vector3<f64>, normal: &Vector3<f64>) -> BSDF {
        Reflection::new(Unit::new_normalize(*normal), 0.0)
            .reflect(&mut direction);

        BSDF{
            direction: random_in_cone(&direction, 1.0 - self.gloss),
            signal: Vector3::new(1.0, 1.0, 1.0).lerp(&self.frensel, self.metal)
        }
    }

    fn refracted_entry(&self, direction: Vector3<f64>, normal: &Vector3<f64>) -> BSDF {
        BSDF{
            direction: direction.refraction(normal, 1.0, self.refraction).unwrap(),
            signal: Vector3::new(1.0, 1.0, 1.0)
        }
    }

    fn refracted_exit(&self, exited: Vector3<f64>, length: f64) -> BSDF {
        let opacity = 1.0 - self.transparency;
        let volume = f64::min(opacity * length * length, 1.0);
        let tint = Vector3::new(1.0, 1.0, 1.0).lerp(&self.color, volume);
        BSDF {
            direction: exited,
            signal: tint,
        }
    }
}

struct FilteredProbabilityTest {
    r: f64,
    p: f64
}

impl FilteredProbabilityTest {
    fn new() -> Self {
        Self{r: rand::random::<f64>(), p: 0.0}
    }

    fn or(&mut self, p: f64) -> bool {
        self.p = (1.0 - self.p) * p;
        self.r <= self.p
    }
}

trait Averageable {
    fn average(&self) -> f64;
}

impl Averageable for Vector3<f64> {
    fn average(&self) -> f64 {
        (self.x + self.y + self.z) / 3.0
    }
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
        (rand::random::<f64>() * 2.0 - 1.0).asin(),
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

trait Ray {
    fn refraction(
        &self,
        normal: &Vector3<f64>,
        exterior_index: f64,
        interior_index: f64,
    ) -> Option<Vector3<f64>>;
}

impl Ray for Vector3<f64> {
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

#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::Vector3;

    #[test]
    fn schilck_is_correct() {

        let incident = Vector3::new(
            0.9999877074290066,
            0.002070457097031252,
            0.004505352182583419,
        );
        let normal = Vector3::new(
            -0.42430229364657923,
            0.17526903761586785,
            -0.8883964925974548,
        );
        
        let material = Material::new(
            Vector3::new(0.1, 0.1, 1.0),
            1.0,
            0.0,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.04, 0.04, 0.04),
            0.0,
            0.2
        );

        assert_eq!(
            material.schilck(&incident, &normal),
            Vector3::new(
                0.09881546766725074,
                0.09881546766725074,
                0.09881546766725074
            )
        )
    }
}
