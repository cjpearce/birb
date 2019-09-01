use nalgebra::{geometry::Reflection, Unit, Vector3};
use crate::ray::DirectionExt;
use rand;
use std::f64;

pub struct BSDF {
    pub direction: Vector3<f64>,
    pub signal: Vector3<f64>,
    pub reflected: bool
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

    pub fn emit(&self) -> Vector3<f64> {
        self.light
    }

    pub fn bsdf(
        &self,
        normal: &Vector3<f64>,
        direction: &Vector3<f64>,
        length: f64,
        u: f64,
        v: f64
    ) -> BSDF {
        let entering = direction.dot(&normal) < 0f64;
        if entering {
            let mut test = FilteredProbabilityTest::new();
            if test.or(self.schilck(&normal, &direction).component_average()) {
                self.reflected(*direction, &normal, u, v)
            } else if test.or(self.transparency) {
                self.refracted_entry(*direction, &normal)
            } else if test.or(self.metal) {
                self.dead()
            } else {
                self.diffused(&normal, u, v)
            }
        } else if let Some(exited) = direction.refraction(&-normal, self.refraction, 1.0) {
            self.refracted_exit(exited, length)
        } else {
            self.dead()
        }
    }

    fn dead(&self) -> BSDF {
        BSDF {
            direction: Vector3::new(0.0, 0.0, 0.0),
            signal: Vector3::new(0.0, 0.0, 0.0),
            reflected: false
        }
    }

    fn schilck(&self, incident: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        let cos_incident = (-incident).dot(&normal);
        self.frensel + ((Vector3::new(1.0, 1.0, 1.0) - self.frensel) * (1.0 - cos_incident).powf(5.0))
    }

    fn diffused(&self, normal: &Vector3<f64>, u: f64, v: f64) -> BSDF {
        let pdf = std::f64::consts::PI;
        BSDF {
            direction: Vector3::random_in_cos_hemisphere(normal, u, v),
            signal: self.color * (1.0 / pdf),
            reflected: false
        }
    }

    fn reflected(&self, mut direction: Vector3<f64>, normal: &Vector3<f64>, u: f64, v: f64) -> BSDF {
        Reflection::new(Unit::new_normalize(*normal), 0.0)
            .reflect(&mut direction);

        BSDF{
            direction: Vector3::random_in_cone(&direction, 1.0 - self.gloss, u, v),
            signal: Vector3::new(1.0, 1.0, 1.0).lerp(&self.frensel, self.metal),
            reflected: true
        }
    }

    fn refracted_entry(&self, direction: Vector3<f64>, normal: &Vector3<f64>) -> BSDF {
        BSDF{
            direction: direction.refraction(normal, 1.0, self.refraction).unwrap(),
            signal: Vector3::new(1.0, 1.0, 1.0),
            reflected: true
        }
    }

    fn refracted_exit(&self, exited: Vector3<f64>, length: f64) -> BSDF {
        let opacity = 1.0 - self.transparency;
        let volume = f64::min(opacity * length * length, 1.0);
        let tint = Vector3::new(1.0, 1.0, 1.0).lerp(&self.color, volume);
        BSDF {
            direction: exited,
            signal: tint,
            reflected: true
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
