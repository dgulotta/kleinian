use crate::{Circle, Cpx};
use nalgebra::Matrix2;

/// Returns the adjoint of the matrix m.  We only call this function with matrices
/// of determinant 1, in which case the adjoint is the same as the inverse.
pub fn inv(m: &Matrix2<Cpx>) -> Matrix2<Cpx> {
    Matrix2::new(m[(1, 1)], -m[(0, 1)], -m[(1, 0)], m[(0, 0)])
}

pub fn inv_dagger(m: &Matrix2<Cpx>) -> Matrix2<Cpx> {
    Matrix2::new(
        m[(1, 1)].conj(),
        -m[(1, 0)].conj(),
        -m[(0, 1)].conj(),
        m[(0, 0)].conj(),
    )
}

pub fn dagger(m: &Matrix2<Cpx>) -> Matrix2<Cpx> {
    Matrix2::new(
        m[(0, 0)].conj(),
        m[(1, 0)].conj(),
        m[(0, 1)].conj(),
        m[(1, 1)].conj(),
    )
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
pub fn circle_for_transforms(u: &Matrix2<Cpx>, v: &Matrix2<Cpx>) -> Circle {
    let un = u - Matrix2::from_diagonal_element(0.5 * u.trace());
    let vn = v - Matrix2::from_diagonal_element(0.5 * v.trace());
    let uv = row_vector_for_nilpotent(&un);
    let vv = row_vector_for_nilpotent(&vn);
    let m = vv.adjoint() * uv;
    let mh = m + dagger(&m);
    Circle(mh / (-mh.determinant()).sqrt())
}
