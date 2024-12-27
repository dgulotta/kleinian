use clap::{Arg, Command};
use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use kleinian::Cpx;
use std::fs::File;

fn main() {
    let matches = Command::new("kleinian")
        .arg(
            Arg::new("width")
                .required(true)
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("height")
                .required(true)
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("re1")
                .required(true)
                .value_parser(clap::value_parser!(f64))
                .allow_negative_numbers(true),
        )
        .arg(
            Arg::new("im1")
                .required(true)
                .value_parser(clap::value_parser!(f64))
                .allow_negative_numbers(true),
        )
        .arg(
            Arg::new("re2")
                .required(true)
                .value_parser(clap::value_parser!(f64))
                .allow_negative_numbers(true),
        )
        .arg(
            Arg::new("im2")
                .required(true)
                .value_parser(clap::value_parser!(f64))
                .allow_negative_numbers(true),
        )
        .arg(
            Arg::new("iters")
                .required(true)
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(Arg::new("outfile").required(true))
        .get_matches();
    let width: usize = *matches.get_one("width").unwrap();
    let height: usize = *matches.get_one("height").unwrap();
    let re1: f64 = *matches.get_one("re1").unwrap();
    let im1: f64 = *matches.get_one("im1").unwrap();
    let re2: f64 = *matches.get_one("re2").unwrap();
    let im2: f64 = *matches.get_one("im2").unwrap();
    let iters: usize = *matches.get_one("iters").unwrap();
    let filename: &String = matches.get_one("outfile").unwrap();
    let p1 = Cpx::new(re1, im1);
    let p2 = Cpx::new(re2, im2);
    let pts = kleinian::generate_points_from_traces(p1, p2, iters);
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
        .write_image(
            &pixel_data,
            width as u32,
            height as u32,
            ExtendedColorType::L8,
        )
        .unwrap();
}
