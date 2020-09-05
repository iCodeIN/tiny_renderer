use image::{imageops, Rgb, RgbImage};
use std::fs::File;
use std::path::Path;
use tiny_renderer::parse::parse_obj;
use tiny_renderer::render;

const WIDTH: usize = 801;
const HEIGHT: usize = 801;

fn main() {
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    let obj_file = File::open("assets/test.obj").expect("Could not load file");

    let (vertices, faces) = parse_obj(&obj_file);

    render::wire_frame(&mut image, Rgb([255, 255, 255]), &vertices, &faces);

    imageops::flip_vertical_in_place(&mut image);

    image
        .save(Path::new("assets/test_image.jpeg"))
        .expect("Could not save the result");
}
