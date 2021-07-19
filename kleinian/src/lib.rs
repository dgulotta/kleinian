mod algebra;
mod circle;
mod queue;
pub mod window;

use crate::algebra::{circle_for_transforms, inv};
use crate::circle::Circle;
use crate::queue::CircleQueue;
use nalgebra::Matrix2;
use num_complex::Complex;

pub type Cpx = Complex<f64>;

/// A generator of a Kleinian group, along with a circle that approximates
/// a boundary of a fundamental domain.
///
/// Usually, there is no fundamental domain with circular boundaries, so our
/// circles are not tangent to each other.  But this heuristic seems to work
/// well in practice.
pub struct Generator {
    pub matrix: Matrix2<Cpx>,
    pub circle: Circle,
}

pub fn generate_points(gens: [Generator; 4], num_points: usize) -> Vec<Cpx> {
    let mut queue = CircleQueue::new(gens);
    while queue.len() < num_points {
        queue.advance()
    }
    queue.circles().map(|c| c.center()).collect()
}

pub fn generate_points_from_traces(ta: Cpx, tb: Cpx, num_points: usize) -> Vec<Cpx> {
    let gens = generators(ta, tb);
    generate_points(gens, num_points)
}

/// Returns a quadruple of matrices [a,b,a^{-1},b^{-1}] such that
/// tr a = `ta`, tr b = `tb`, and tr aba^{-1}b^{-1} = -2.  The formula
/// is taken from p229 of Indra's Pearls by Mumford, Series, and
/// Wright.
pub fn generators(ta: Cpx, tb: Cpx) -> [Generator; 4] {
    let c0 = ta * ta + tb * tb;
    let c1 = ta * tb;
    let tab = 0.5 * (c1 - (c1 * c1 - 4.0 * c0).sqrt());
    let i = Cpx::i();
    let z0 = (tab - 2.0) * tb / (tb * tab - 2.0 * ta + 2.0 * i * tab);
    let htb = 0.5 * tb;
    let htab = 0.5 * tab;
    let b = Matrix2::new(htb - i, htb, htb, htb + i);
    let ab = Matrix2::new(htab, (htab - 1.0) / z0, (htab + 1.0) * z0, htab);
    let bi = inv(&b);
    let a = ab * bi;
    let ai = inv(&a);
    let k1 = bi * a * b * ai;
    let k2 = a * b * ai * bi;
    let k3 = b * ai * bi * a;
    let k4 = ai * bi * a * b;
    let ca = circle_for_transforms(&k1, &k2);
    let cb = circle_for_transforms(&k2, &k3);
    let cai = circle_for_transforms(&k3, &k4);
    let cbi = circle_for_transforms(&k4, &k1);
    [
        Generator {
            matrix: a,
            circle: ca,
        },
        Generator {
            matrix: b,
            circle: cb,
        },
        Generator {
            matrix: ai,
            circle: cai,
        },
        Generator {
            matrix: bi,
            circle: cbi,
        },
    ]
}

/// Returns a quadruple of matrices [a,b,a^{-1},b^{-1}] such that
/// tr a = `ta`, tr b = `tb`, and tr abab^{-1} = -2.
pub fn generators_xx(ta: Cpx, tb: Cpx) -> [Generator; 4] {
    let hta = 0.5 * ta;
    let htb = 0.5 * tb;
    let hta21 = hta * hta - 1.0;
    let htb2 = htb * htb;
    let c0 = hta21 * (htb2 + 1.0) + 2.0;
    let c1 = ta * (hta21 * htb2 + 1.0).sqrt();
    let b1 = (c0 + c1).sqrt();
    let a = Matrix2::new(hta, hta * hta - 1.0, (1.0).into(), hta);
    let b = Matrix2::new(htb, b1, (htb * htb - 1.0) / b1, htb);
    let ai = inv(&a);
    let bi = inv(&b);
    let k1 = bi * ai * b * ai;
    let k2 = a * b * a * bi;
    let k3 = b * a * bi * a;
    let k4 = ai * bi * ai * b;
    let ca = circle_for_transforms(&k1, &k2);
    let cb = circle_for_transforms(&k2, &k3);
    let cai = circle_for_transforms(&k3, &k4);
    let cbi = circle_for_transforms(&k4, &k1);
    [
        Generator {
            matrix: a,
            circle: ca,
        },
        Generator {
            matrix: b,
            circle: cb,
        },
        Generator {
            matrix: ai,
            circle: cai,
        },
        Generator {
            matrix: bi,
            circle: cbi,
        },
    ]
}

/// Returns a quadruple of matrices [a,b,a^{-1},b^{-1}] such that
/// tr a = `ta`, tr ab = tr ab^{-1} = 2.
pub fn generators_x(ta: Cpx) -> [Generator; 4] {
    let ah = 0.5 * ta;
    let bh = 1.0 / ah;
    let a = Matrix2::new(ah, (1.0).into(), ah * ah - 1.0, ah);
    let b = Matrix2::new(bh, -bh, ah - bh, bh);
    let ai = inv(&a);
    let bi = inv(&b);
    let k1 = bi * ai;
    let k2 = a * bi;
    let k3 = b * a;
    let k4 = ai * b;
    let ca = circle_for_transforms(&k1, &k2);
    let cb = circle_for_transforms(&k2, &k3);
    let cai = circle_for_transforms(&k3, &k4);
    let cbi = circle_for_transforms(&k4, &k1);
    [
        Generator {
            matrix: a,
            circle: ca,
        },
        Generator {
            matrix: b,
            circle: cb,
        },
        Generator {
            matrix: ai,
            circle: cai,
        },
        Generator {
            matrix: bi,
            circle: cbi,
        },
    ]
}
