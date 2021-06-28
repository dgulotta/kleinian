use core::ops::Mul;
use derivative::Derivative;
use nalgebra::Matrix2;
use num_complex::Complex;
use ordered_float::NotNan;
use std::collections::BinaryHeap;

pub type Cpx = Complex<f64>;

/// Returns the adjoint of the matrix m.  We only call this function with matrices
/// of determinant 1, in which case the adjoint is the same as the inverse.
fn inv(m: &Matrix2<Cpx>) -> Matrix2<Cpx> {
    Matrix2::new(m[(1, 1)], -m[(0, 1)], -m[(1, 0)], m[(0, 0)])
}

fn inv_dagger(m: &Matrix2<Cpx>) -> Matrix2<Cpx> {
    Matrix2::new(
        m[(1, 1)].conj(),
        -m[(1, 0)].conj(),
        -m[(0, 1)].conj(),
        m[(0, 0)].conj(),
    )
}

fn dagger(m: &Matrix2<Cpx>) -> Matrix2<Cpx> {
    Matrix2::new(
        m[(0, 0)].conj(),
        m[(1, 0)].conj(),
        m[(0, 1)].conj(),
        m[(1, 1)].conj(),
    )
}

#[derive(Clone, Copy, Debug)]
pub struct Circle(Matrix2<Cpx>);

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

fn row_vector_for_nilpotent(u: &Matrix2<Cpx>) -> nalgebra::RowVector2<Cpx> {
    if u[(0, 1)].norm_sqr() >= u[(1, 0)].norm_sqr() {
        let s = (-u[(0, 1)]).sqrt();
        nalgebra::RowVector2::new(u[(0, 0)] / s, -s)
    } else {
        let s = u[(1, 0)].sqrt();
        nalgebra::RowVector2::new(s, u[(1, 1)] / s)
    }
}

/// Returns a circle C containing the fixed points of the parabolic
/// transforms u and v, such that if b is any transform satisfying bub^{-1} = v,
/// then bC is tangent to C.
fn circle_for_transforms(u: &Matrix2<Cpx>, v: &Matrix2<Cpx>) -> Circle {
    let un = u - Matrix2::from_diagonal_element(0.5 * u.trace());
    let vn = v - Matrix2::from_diagonal_element(0.5 * v.trace());
    let uv = row_vector_for_nilpotent(&un);
    let vv = row_vector_for_nilpotent(&vn);
    let m = vv.adjoint() * uv;
    let mh = m + dagger(&m);
    Circle(mh / (-mh.determinant()).sqrt())
}

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

struct CircleQueue {
    queue: BinaryHeap<QueueItem>,
    gens: [Generator; 4],
}

impl CircleQueue {
    fn item(&self, matrix: Matrix2<Cpx>, last: u8) -> QueueItem {
        let ri = (matrix * self.gens[last as usize].circle).radius_inv();
        QueueItem {
            matrix,
            last,
            priority: NotNan::new(-ri).unwrap(),
        }
    }
    pub fn new(gens: [Generator; 4]) -> Self {
        let mut q = CircleQueue {
            queue: BinaryHeap::new(),
            gens,
        };
        for i in 0..4 {
            q.queue.push(q.item(Matrix2::identity(), i));
        }
        q
    }
    pub fn advance(&mut self) {
        let item = self.queue.pop().unwrap();
        let matrix = item.matrix * self.gens[item.last as usize].matrix;
        for i in 3..6 {
            self.queue.push(self.item(matrix, (item.last + i) % 4));
        }
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn circles(self) -> impl Iterator<Item = Circle> {
        let (queue, gens) = (self.queue, self.gens);
        queue
            .into_iter()
            .map(move |i| i.matrix * gens[i.last as usize].circle)
    }
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct QueueItem {
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    matrix: Matrix2<Cpx>,
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    last: u8,
    priority: NotNan<f64>,
}

pub fn generate_points_from_traces(ta: Cpx, tb: Cpx, num_points: usize) -> Vec<Cpx> {
    let gens = generators(ta, tb);
    generate_points(gens, num_points)
}

#[derive(Clone, Copy)]
pub struct CoordTransform {
    scale: f64,
    xoff: f64,
    yoff: f64,
}

impl CoordTransform {
    pub fn apply(&self, pt: &Cpx) -> (usize, usize) {
        let x = (self.scale * (pt.re - self.xoff)) as usize;
        let y = (self.scale * (pt.im - self.yoff)) as usize;
        (x, y)
    }
}

pub fn window_transform(pts: &[Cpx], width: usize, height: usize) -> CoordTransform {
    let w = width as f64;
    let h = height as f64;
    let p_xmin = *pts
        .iter()
        .map(|z| NotNan::new(z.re).unwrap())
        .min()
        .unwrap();
    let p_xmax = *pts
        .iter()
        .map(|z| NotNan::new(z.re).unwrap())
        .max()
        .unwrap();
    let p_ymin = *pts
        .iter()
        .map(|z| NotNan::new(z.im).unwrap())
        .min()
        .unwrap();
    let p_ymax = *pts
        .iter()
        .map(|z| NotNan::new(z.im).unwrap())
        .max()
        .unwrap();
    let scale = f64::min(w / (p_xmax - p_xmin), h / (p_ymax - p_ymin)) * 0.999;
    let xoff = 0.5 * (p_xmin + p_xmax - w / scale);
    let yoff = 0.5 * (p_ymin + p_ymax - h / scale);
    CoordTransform { scale, xoff, yoff }
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
    let k1 = a * b * ai * bi;
    let k2 = ai * bi * a * b;
    let k3 = bi * a * b * ai;
    let k4 = b * ai * b * a;
    let ca = circle_for_transforms(&k1, &k3);
    let cb = circle_for_transforms(&k1, &k4);
    let cai = circle_for_transforms(&k2, &k4);
    let cbi = circle_for_transforms(&k2, &k3);
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
    let k1 = a * bi * a * b;
    let k2 = b * ai * bi * ai;
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
    let ab = a * b;
    let bai = b * ai;
    let aibi = ai * bi;
    let bia = bi * a;
    let ca = circle_for_transforms(&ab, &bai);
    let cb = circle_for_transforms(&bai, &aibi);
    let cai = circle_for_transforms(&aibi, &bia);
    let cbi = circle_for_transforms(&bia, &ab);
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
