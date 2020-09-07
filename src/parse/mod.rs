use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct ObjVertex(pub [f64; 3]);

impl ObjVertex {
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

impl Add for ObjVertex {
    type Output = Self;

    fn add(self, other: ObjVertex) -> Self {
        ObjVertex([
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ])
    }
}

impl Sub for ObjVertex {
    type Output = Self;

    fn sub(self, other: ObjVertex) -> Self {
        ObjVertex([
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        ])
    }
}

impl Mul<f64> for ObjVertex {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        ObjVertex([self.x() * rhs, self.y() * rhs, self.z() * rhs])
    }
}

pub struct ObjFace(pub [usize; 3]);

impl ObjFace {
    pub fn get_vertex_index(&self, index: usize) -> usize {
        assert!(index < 3);

        self.0[index] - 1
    }
}

pub fn parse_obj(obj_file: &File) -> (VecDeque<ObjVertex>, VecDeque<ObjFace>) {
    let mut vertices = VecDeque::<ObjVertex>::new();
    let mut faces = VecDeque::<ObjFace>::new();

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

                vertices.push_back(ObjVertex([x, y, z]));
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

                faces.push_back(ObjFace([vertex1, vertex2, vertex3]));
            }
        }
    }

    (vertices, faces)
}
