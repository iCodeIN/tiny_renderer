use image::{imageops, Rgb, RgbImage};
use std::fs::File;
use std::path::Path;
use tiny_renderer::geometry::core::Vec3f;
use tiny_renderer::render;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let mut image = RgbImage::new(WIDTH as u32, HEIGHT as u32);

    render::new_triangle(
        &mut Vec3f([10.0, 10.0, 0.0]),
        &mut Vec3f([100.0, 30.0, 0.0]),
        &mut Vec3f([190.0, 160.0, 0.0]),
        &mut image,
        Rgb([255, 0, 0]),
    );

    imageops::flip_vertical_in_place(&mut image);

    image
        .save(Path::new("test_image.jpeg"))
        .expect("Could not save the file");
}
