use nalgebra::{Vector3};
pub struct OrthonormalBasis(Vector3<f64>, Vector3<f64>, Vector3<f64>);

// TODO: Impl the from trait?
impl OrthonormalBasis {
    pub fn from_normal(n: Vector3<f64>) -> Self {
        let w = n.normalize();
        let w_orth = if n.x > 0.7 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };

        let v = w.cross(&w_orth).normalize();
        let u = w.cross(&v).normalize();

        Self(u, v, w)
    }

    fn u(&self) -> Vector3<f64> { self.0 }
    fn v(&self) -> Vector3<f64> { self.1 }
    fn w(&self) -> Vector3<f64> { self.2 }
    
    pub fn local(&self, a: Vector3<f64>) -> Vector3<f64> {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}