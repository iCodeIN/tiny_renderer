use tiny_renderer::parse::parse_obj;
use std::fs::File;

// const WIDTH: usize = 800;
// const HEIGHT: usize = 800;

fn main() {
    let obj_file = File::open("assets/test.obj").expect("Could not load file");

    let (vertices, faces) = parse_obj(&obj_file);

    println!("#vertices: {}", vertices.len());
    println!("#faces: {}", faces.len());
}
