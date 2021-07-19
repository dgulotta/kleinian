use crate::algebra::{inv, inv_dagger};
use crate::Cpx;
use core::ops::Mul;
use nalgebra::Matrix2;

#[derive(Clone, Copy, Debug)]
pub struct Circle(pub Matrix2<Cpx>);

impl Mul<Circle> for Matrix2<Cpx> {
    type Output = Circle;
    fn mul(self, c: Circle) -> Circle {
        Circle(inv_dagger(&self) * c.0 * inv(&self))
    }
}

impl Circle {
    pub fn radius_inv(&self) -> f64 {
        self.0[(0, 0)].re.abs()
    }
    pub fn center(&self) -> Cpx {
        -self.0[(0, 1)] / self.0[(0, 0)].re
    }
}
