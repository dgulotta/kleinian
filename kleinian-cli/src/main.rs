use clap::{value_t, App, Arg};
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use kleinian::Cpx;
use std::fs::File;

fn main() {
    let matches = App::new("kleinian")
        .arg(Arg::with_name("width").required(true))
        .arg(Arg::with_name("height").required(true))
        .arg(
            Arg::with_name("re1")
                .required(true)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("im1")
                .required(true)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("re2")
                .required(true)
                .allow_hyphen_values(true),
        )
        .arg(
            Arg::with_name("im2")
                .required(true)
                .allow_hyphen_values(true),
        )
        .arg(Arg::with_name("iters").required(true))
        .arg(Arg::with_name("outfile").required(true))
        .get_matches();
    let width = value_t!(matches.value_of("width"), usize).unwrap_or_else(|e| e.exit());
    let height = value_t!(matches.value_of("height"), usize).unwrap_or_else(|e| e.exit());
    let re1 = value_t!(matches.value_of("re1"), f64).unwrap_or_else(|e| e.exit());
    let im1 = value_t!(matches.value_of("im1"), f64).unwrap_or_else(|e| e.exit());
    let re2 = value_t!(matches.value_of("re2"), f64).unwrap_or_else(|e| e.exit());
    let im2 = value_t!(matches.value_of("im2"), f64).unwrap_or_else(|e| e.exit());
    let iters = value_t!(matches.value_of("iters"), usize).unwrap_or_else(|e| e.exit());
    let filename = value_t!(matches.value_of("outfile"), String).unwrap_or_else(|e| e.exit());
    let p1 = Cpx::new(re1, im1);
    let p2 = Cpx::new(re2, im2);
    let pts = kleinian::generate_points_from_traces(p1, p2, iters as usize);
    let trans = kleinian::window::window_transform(&pts, width, height);
    let mut pixel_data = Vec::new();
    pixel_data.resize(width * height, 255);
    for pt in pts {
        let (x, y) = trans.apply(&pt);
        let idx = x * height + y;
        pixel_data[idx] = 0;
    }
    let f = File::create(filename.as_str()).unwrap_or_else(|e| clap::Error::from(e).exit());
    PngEncoder::new(f)
        .write_image(&pixel_data, width as u32, height as u32, ExtendedColorType::L8)
        .unwrap();
}
