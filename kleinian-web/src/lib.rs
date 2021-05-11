use kleinian::Cpx;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    re1: f64,
    im1: f64,
    re2: f64,
    im2: f64,
    iters: u32,
) -> Result<(), JsValue> {
    let pts = kleinian::generate_points_from_traces(
        Cpx::new(re1, im1),
        Cpx::new(re2, im2),
        iters as usize,
    );
    let w = width as usize;
    let h = height as usize;
    let trans = kleinian::window_transform(&pts, w, h);
    let mut pixel_data = Vec::new();
    pixel_data.resize(w * h * 4, 255);
    for pt in pts {
        let (x, y) = trans.apply(&pt);
        let idx = x * h + y;
        pixel_data[4 * idx] = 0;
        pixel_data[4 * idx + 1] = 0;
        pixel_data[4 * idx + 2] = 0;
    }
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&pixel_data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}
