use crate::geometry::core::{Face, Vec3f};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn parse_obj(obj_file: &File) -> (VecDeque<Vec3f>, VecDeque<Face>) {
    let mut vertices = VecDeque::<Vec3f>::new();
    let mut faces = VecDeque::<Face>::new();

    let reader = BufReader::new(obj_file);

    for line in reader.lines() {
        if let Ok(line_text) = line {
            let segments: Vec<&str> = line_text.split(" ").collect();
            if segments[0] == "v" {
                // if it's a vertex

                // extract vertex coordinates
                let x = segments[1].parse::<f64>().unwrap();
                let y = segments[2].parse::<f64>().unwrap();
                let z = segments[3].parse::<f64>().unwrap();

                vertices.push_back(Vec3f([x, y, z]));
            } else if segments[0] == "f" {
                // if it's a face

                // extract vertex indices
                let vertex1 = segments[1]
                    .split("/")
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let vertex2 = segments[2]
                    .split("/")
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let vertex3 = segments[3]
                    .split("/")
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                faces.push_back(Face([vertex1, vertex2, vertex3]));
            }
        }
    }

    (vertices, faces)
}
