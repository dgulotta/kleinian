use crate::Cpx;
use ordered_float::NotNan;

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
