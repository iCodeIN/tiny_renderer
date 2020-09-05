use crate::parse::{ObjFace, ObjVertex};
use image::{Rgb, RgbImage};
use std::collections::VecDeque;

pub fn line(
    mut x0: i64,
    mut y0: i64,
    mut x1: i64,
    mut y1: i64,
    image: &mut RgbImage,
    color: Rgb<u8>,
) {
    let steep = (x1 - x0).abs() < (y1 - y0).abs();
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let d_error = dy.abs() * 2;
    let mut error = 0;
    let mut y = y0;
    for x in x0..=x1 {
        if steep {
            image.put_pixel(y as u32, x as u32, color);
        } else {
            image.put_pixel(x as u32, y as u32, color);
        }

        error += d_error;
        if error > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error -= dx * 2;
        }
    }
}

pub fn wire_frame(
    mut image: &mut RgbImage,
    color: Rgb<u8>,
    vertices: &VecDeque<ObjVertex>,
    faces: &VecDeque<ObjFace>,
) {
    let width = image.width();
    let height = image.height();

    for face in faces {
        for i in 0..3 {
            let v0 = &vertices[face.get_vertex_index(i)];
            let v1 = &vertices[face.get_vertex_index((i + 1) % 3)];

            let x0 = ((v0.x() + 1.0) * (width / 2) as f64) as i64;
            let y0 = ((v0.y() + 1.0) * (height / 2) as f64) as i64;

            let x1 = ((v1.x() + 1.0) * (width / 2) as f64) as i64;
            let y1 = ((v1.y() + 1.0) * (height / 2) as f64) as i64;

            line(x0, y0, x1, y1, &mut image, color);
        }
    }
}
