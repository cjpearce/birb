use nalgebra::{geometry::Reflection, Unit, Vector3, Point3};
use crate::ray::{DirectionExt, Ray};
use crate::scene::{Scene, Intersection};
use crate::onb::{OrthonormalBasis};
use rand;
use std::f64;


pub struct BSDF {
    pub direction: Vector3<f64>,
    pub signal: Vector3<f64>
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
        v: f64,
        scene: &Scene,
        intersect: &Intersection
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
                self.diffused(&normal, u, v, scene, intersect)
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
            signal: Vector3::new(0.0, 0.0, 0.0)
        }
    }

    fn schilck(&self, incident: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        let cos_incident = (-incident).dot(&normal);
        self.frensel + ((Vector3::new(1.0, 1.0, 1.0) - self.frensel) * (1.0 - cos_incident).powf(5.0))
    }

    fn diffused(&self, normal: &Vector3<f64>, u: f64, v: f64, scene: &Scene,
        intersect: &Intersection) -> BSDF {
        let a = CosWeightedDiffuse::new(*normal, u, v);
        let b = LightWeightedDiffuse::new(intersect.hit, *normal, scene);
        let mix = MixturePdf::new(1.0, &a, &b);
        let direction = mix.gen();
        BSDF {
            direction,
            signal: self.color * mix.p(direction)
        }
    }

    fn reflected(&self, mut direction: Vector3<f64>, normal: &Vector3<f64>, u: f64, v: f64) -> BSDF {
        Reflection::new(Unit::new_normalize(*normal), 0.0)
            .reflect(&mut direction);

        BSDF{
            direction: Vector3::random_in_cone(&direction, 1.0 - self.gloss, u, v),
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
            signal: tint
        }
    }
}

trait Pdf {
    fn p(&self, v: Vector3<f64>) -> f64;
    fn gen(&self) -> Vector3<f64>;
}

struct MixturePdf<'a, 'b> {
    mix: f64,
    a: &'a Pdf,
    b: &'b Pdf
}

impl <'a, 'b> MixturePdf<'a, 'b> {
    fn new(mix: f64, a: &'a Pdf, b: &'b Pdf) -> Self {
        Self{mix, a, b}
    }
}

impl <'a, 'b> Pdf for MixturePdf<'a, 'b> {
    fn p(&self, v: Vector3<f64>) -> f64 {
        (1.0 - self.mix) * self.a.p(v) + self.mix * self.b.p(v)
    }

    fn gen(&self) -> Vector3<f64> {
        if rand::random::<f64>() <= self.mix {
            self.b.gen()
        } else {
            self.a.gen()
        }
    }
}


struct CosWeightedDiffuse {
    u: f64,
    v: f64,
    normal: Vector3<f64>
}

impl CosWeightedDiffuse {
    fn new(normal: Vector3<f64>, u: f64, v: f64) -> Self {
        Self{u, v, normal}
    }
}

impl Pdf for CosWeightedDiffuse {
    fn p(&self, v: Vector3<f64>) -> f64 {
        let cosine = v.dot(&self.normal);
        if cosine > 0.0 { cosine / std::f64::consts::PI } else { 0.0 }
    }

    fn gen(&self) -> Vector3<f64> {
        let vec = Vector3::random_in_cos_hemisphere(self.u, self.v);
        let onb = OrthonormalBasis::from_normal(self.normal);
        onb.local(vec)
    }
}

struct LightWeightedDiffuse<'a> {
    point: Point3<f64>,
    normal: Vector3<f64>,
    scene: &'a Scene
}

impl <'a> LightWeightedDiffuse<'a> {
    fn new(point: Point3<f64>, normal: Vector3<f64>, scene: &'a Scene) -> Self {
        Self{point, normal, scene}
    }
}

impl <'a> Pdf for LightWeightedDiffuse<'a> {
    fn p(&self, v: Vector3<f64>) -> f64 {
        let light = self.scene.light();

        // get bounding sphere center and radius
        let center = light.center();
        let radius = light.radius();

        let cos_angle = v.dot(&self.normal);
        if cos_angle <= 0.0 {
            return 0.0;
        }

        // compute solid angle (hemisphere coverage)
        let hyp = (center - self.point).norm();
        let opp = radius;
        let theta = (opp / hyp).asin();
        let adj = opp / theta.tan();
        let d = theta.cos() * adj;
        let r = theta.sin() * adj;

        let coverage = if hyp < opp {
            1.0
        } else {
            f64::min((r * r) / (d * d), 1.0)
        };

        let a = (center - self.point).angle(&v);

        if a < radius {
            coverage / std::f64::consts::PI
        } else {
            0.0
        }
    }

    fn gen(&self) -> Vector3<f64> {
        let light = self.scene.light();

        // get bounding sphere center and radius
        let center = light.center();
        let radius = light.radius();

        // get random point in disk
        let point = loop {
            let x = rand::random::<f64>() * 2.0 - 1.0;
            let y = rand::random::<f64>() * 2.0 - 1.0;
            if x*x + y*y <= 1.0 {
                let l = (center - self.point).normalize();
                let u = l.cross(&Vector3::random_in_sphere()).normalize();
                let v = l.cross(&u);

                break center + (u * x * radius) + (v * y * radius);
            }
        };

        // construct ray toward light point
        (point - self.point).normalize()
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
