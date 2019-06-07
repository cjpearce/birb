use nalgebra::{Vector3, Unit, geometry::Reflection};
use std::f32;
use rand;

pub struct BSDF {
    direction: Vector3<f32>,
    signal: Vector3<f32>,
}

pub struct Material {
    color: Vector3<f32>,
    refraction: f32,
    transparency: f32,
    light: Vector3<f32>,
    frensel: Vector3<f32>,
    metal: f32,
    gloss: f32,
}

impl Material {
    pub fn new(
        color: Vector3<f32>,
        refraction: f32,
        transparency: f32,
        light: Vector3<f32>,
        frensel: Vector3<f32>,
        metal: f32,
        gloss: f32,
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

    fn emit(&self, normal: &Vector3<f32>, direction: &Vector3<f32>) -> Option<Vector3<f32>> {
        if self.color.max() == 0f32 {
            return None;
        }

        let inverted = -direction;
        let cos = f32::max(normal.dot(&inverted), 0f32);
        Some(self.light * cos)
    }

    fn bsdf(&self, normal: Vector3<f32>, direction: Vector3<f32>, length: f32) -> Option<BSDF> {
        let entering = direction.dot(&normal) < 0f32;
        if entering {
            let reflect = self.schilck(&normal, &direction);
            let roughness = 1.0 - self.gloss;

            if rand::random::<f32>() <= ave(reflect) {
                let reflection = Reflection::new(Unit::new_normalize(normal), 0.0);
                let mut reflected = direction;
                reflection.reflect(&mut reflected);
                reflected += random_in_cone(&normal, roughness);
                let tint = Vector3::new(1.0, 1.0, 1.0).lerp(&self.frensel, self.metal);
                return Some(BSDF{
                    direction: reflected,
                    signal: tint,
                })
            }

            if rand::random::<f32>() <= self.transparency {
                let transmitted = direction.refraction(&normal, 1.0, self.refraction).unwrap();
                return Some(BSDF{
                    direction: transmitted,
                    signal: Vector3::new(1.0, 1.0, 1.0)
                })
            }

            if rand::random::<f32>() <= self.metal {
                return None
            }

            let diffused = random_in_cos_hemisphere(direction);
            let pdf = std::f32::consts::PI;

            Some(BSDF{
                direction: diffused,
                signal: self.color * (1.0 / pdf)
            })
        } else {
            let exited = direction.refraction(
                &-normal, self.refraction, 1.0
            );
            
            if !exited.is_some() { return None }

            let opacity = 1.0 - self.transparency;
            let volume = f32::min(opacity * length * length, 1.0);
            let tint = Vector3::new(1.0, 1.0, 1.0).lerp(&self.color, volume);
            Some(BSDF{ direction: exited.unwrap(), signal: tint })
        }
    }

    fn schilck(&self, incident: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
        let cos_incident = (-incident).dot(&normal);
        self.frensel + (
            (
                Vector3::new(1.0, 1.0, 1.0) - self.frensel
                * (1.0 - cos_incident).powf(5.0)
            )
        )
    }
}

fn ave(v: Vector3<f32>) -> f32 {
    ( v.x + v.y + v.z ) / 3.0
}

fn random_in_cone(direction: &Vector3<f32>, width: f32) -> Vector3<f32> {
    let u = rand::random::<f32>();
    let v = rand::random::<f32>();
    let theta = width * 0.5 * f32::consts::PI * (1.0 - (2.0 * u.acos() / f32::consts::PI));
    let m1 = theta.sin();
    let m2 = theta.cos();
    let a = v * 2.0 * f32::consts::PI;
    let q = random_in_sphere();
    let s = direction.cross(&q);
    let t = direction.cross(&s);
    let mut d = Vector3::new(0.0, 0.0, 0.0);
    d += s * (m1 * a.cos());
    d += t * (m1 * a.sin());
    d += direction * m2;
    d.normalize()
}

fn from_angles(theta: f32, phi: f32) -> Vector3<f32> {
    Vector3::new(theta.cos() * phi.cos(), phi.sin(), theta.sin() * phi.cos())
}

fn random_in_sphere() -> Vector3<f32> {
    from_angles(
        rand::random::<f32>() * f32::consts::PI * 2.0,
        (rand::random::<f32>() * 2.0 - 1.0).asin()
    )
}

fn random_in_cos_hemisphere(normal: Vector3<f32>) -> Vector3<f32> {
    let u = rand::random::<f32>();
    let v = rand::random::<f32>();
    let r = u.sqrt();
    let theta = 2.0 * f32::consts::PI * v;
    let sphere_dir = random_in_sphere();
    let s = normal.cross(&sphere_dir).normalize();
    let t = normal.cross(&s);
    let mut d = Vector3::new(0.0, 0.0, 0.0);
    d += s * (r * theta.cos());
    d += t * (r * theta.sin());
    d + normal * (1.0 - u).sqrt();
    d
}

trait LightDirection {
    fn refraction(&self, normal: &Vector3<f32>, exterior_index: f32, interior_index: f32) -> Option<Vector3<f32>>;
}

impl LightDirection for Vector3<f32> {
    fn refraction(&self, normal: &Vector3<f32>, exterior_index: f32, interior_index: f32) -> Option<Vector3<f32>> {
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
