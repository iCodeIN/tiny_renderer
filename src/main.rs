use image::{imageops, RgbaImage};
use std::fs::File;
use std::path::Path;
use tiny_renderer::geometry::core::Vec3f;
use tiny_renderer::parse;
use tiny_renderer::render;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let mut image = RgbaImage::new(WIDTH as u32, HEIGHT as u32);

    let obj_file = File::open("assets/test.obj").expect("Could not load file");

    let (vertices, faces) = parse::parse_obj(&obj_file);

    let mut light_dir = Vec3f([0.0, 0.0, -1.0]);
    light_dir.normalize();
    render::with_light(&mut image, &vertices, &faces, light_dir);

    imageops::flip_vertical_in_place(&mut image);

    image
        .save(Path::new("test_image.jpeg"))
        .expect("Could not save the file");
}
