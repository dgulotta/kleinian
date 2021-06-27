use derivative::Derivative;
use nalgebra::{Matrix2, Vector2};
use num_complex::Complex;
use ordered_float::NotNan;
use std::collections::BinaryHeap;

pub type Cpx = Complex<f64>;

fn coordinate(v: &Vector2<Cpx>) -> Cpx {
    v[0] / v[1]
}
fn height(v: &Vector2<Cpx>) -> f64 {
    v[1].norm_sqr()
}

/// Returns the adjoint of the matrix m.  We only call this function with matrices
/// of determinant 1, in which case the adjoint is the same as the inverse.
fn inv(m: &Matrix2<Cpx>) -> Matrix2<Cpx> {
    Matrix2::new(m[(1, 1)], -m[(0, 1)], -m[(1, 0)], m[(0, 0)])
}

fn fixed_points(m: &Matrix2<Cpx>) -> [Vector2<Cpx>; 2] {
    let t = m.trace();
    let d = (t * t - 4.0 * m.determinant()).sqrt();
    let l1 = 0.5 * (t + d);
    let l2 = 0.5 * (t - d);
    let v1a = Vector2::new(m[(0, 0)] - l1, m[(1, 0)]);
    let v1b = Vector2::new(m[(0, 1)], m[(1, 1)] - l1);
    let v1 = if v1a.norm_squared() > v1b.norm_squared() {
        v1a
    } else {
        v1b
    };
    let v2a = Vector2::new(m[(0, 0)] - l2, m[(1, 0)]);
    let v2b = Vector2::new(m[(0, 1)], m[(1, 1)] - l2);
    let v2 = if v2a.norm_squared() > v2b.norm_squared() {
        v2a
    } else {
        v2b
    };
    [v1.normalize(), v2.normalize()]
}

#[derive(Default)]
struct PointQueue {
    points: Vec<Cpx>,
    queue: BinaryHeap<PointIterItem>,
}

impl PointQueue {
    fn new() -> Self {
        Default::default()
    }
    fn push(&mut self, item: PointIterItem) {
        self.points.push(coordinate(&item.point));
        self.queue.push(item);
    }
    fn pop(&mut self) -> Option<PointIterItem> {
        self.queue.pop()
    }
}

pub fn generate_points(gens: [Matrix2<Cpx>; 4], num_points: usize) -> Vec<Cpx> {
    let mut queue = PointQueue::new();
    let fp1 = fixed_points(&gens[0]);
    let fp2 = fixed_points(&gens[1]);
    queue.push(PointIterItem::new(gens[1] * fp1[0], 1));
    queue.push(PointIterItem::new(gens[3] * fp1[0], 3));
    queue.push(PointIterItem::new(gens[1] * fp1[1], 1));
    queue.push(PointIterItem::new(gens[3] * fp1[1], 3));
    queue.push(PointIterItem::new(gens[0] * fp2[0], 0));
    queue.push(PointIterItem::new(gens[2] * fp2[0], 2));
    queue.push(PointIterItem::new(gens[0] * fp2[1], 0));
    queue.push(PointIterItem::new(gens[2] * fp2[1], 2));
    while queue.points.len() < num_points {
        let item = queue.pop().unwrap();
        for i in 3..6 {
            let last = (item.last + i) % 4;
            let point = gens[last as usize] * item.point;
            queue.push(PointIterItem::new(point, last));
        }
    }
    queue.points
}

pub fn generate_points_from_traces(ta: Cpx, tb: Cpx, num_points: usize) -> Vec<Cpx> {
    let gens = generators(ta, tb);
    generate_points(gens, num_points)
}

#[derive(Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
struct PointIterItem {
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    point: Vector2<Cpx>,
    #[derivative(PartialEq = "ignore")]
    #[derivative(PartialOrd = "ignore")]
    #[derivative(Ord = "ignore")]
    last: u8,
    priority: NotNan<f64>,
}

impl PointIterItem {
    fn new(point: Vector2<Cpx>, last: u8) -> Self {
        let priority = NotNan::new(-height(&point)).unwrap();
        Self {
            point,
            last,
            priority,
        }
    }
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
pub fn generators(ta: Cpx, tb: Cpx) -> [Matrix2<Cpx>; 4] {
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
    [a, b, inv(&a), bi]
}

/// Returns a quadruple of matrices [a,b,a^{-1},b^{-1}] such that
/// tr a = `ta`, tr b = `tb`, and tr abab^{-1} = -2.
pub fn generators_xx(ta: Cpx, tb: Cpx) -> [Matrix2<Cpx>; 4] {
    let hta = 0.5 * ta;
    let htb = 0.5 * tb;
    let hta21 = hta * hta - 1.0;
    let htb2 = htb * htb;
    let c0 = hta21 * (htb2 + 1.0) + 2.0;
    let c1 = ta * (hta21 * htb2 + 1.0).sqrt();
    let b1 = (c0 + c1).sqrt();
    let a = Matrix2::new(hta, hta * hta - 1.0, (1.0).into(), hta);
    let b = Matrix2::new(htb, b1, (htb * htb - 1.0) / b1, htb);
    [a, b, inv(&a), inv(&b)]
}

/// Returns a quadruple of matrices [a,b,a^{-1},b^{-1}] such that
/// tr a = `ta`, tr ab = tr ab^{-1} = 2.
pub fn generators_x(ta: Cpx) -> [Matrix2<Cpx>; 4] {
    let ah = 0.5 * ta;
    let bh = 1.0 / ah;
    let a = Matrix2::new(ah, ah * ah - 1.0, (1.0).into(), ah).transpose();
    let b = Matrix2::new(bh, ah - bh, -bh, bh).transpose();
    [a, b, inv(&a), inv(&b)]
}
