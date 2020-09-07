use image::{imageops, Rgb, RgbImage};
use std::fs::File;
use std::path::Path;
use tiny_renderer::parse::{parse_obj, ObjVertex};
use tiny_renderer::render;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    render::triangle(
        &mut ObjVertex([10.0, 70.0, 0.0]),
        &mut ObjVertex([50.0, 160.0, 0.0]),
        &mut ObjVertex([70.0, 80.0, 0.0]),
        &mut image,
        Rgb([255, 0, 0]),
    );

    render::triangle(
        &mut ObjVertex([180.0, 50.0, 0.0]),
        &mut ObjVertex([150.0, 1.0, 0.0]),
        &mut ObjVertex([70.0, 180.0, 0.0]),
        &mut image,
        Rgb([255, 255, 255]),
    );

    render::triangle(
        &mut ObjVertex([180.0, 150.0, 0.0]),
        &mut ObjVertex([120.0, 160.0, 0.0]),
        &mut ObjVertex([130.0, 180.0, 0.0]),
        &mut image,
        Rgb([0, 255, 0]),
    );

    imageops::flip_vertical_in_place(&mut image);

    image.save(Path::new("test_image.jpeg")).expect("Could not save the file");
}
