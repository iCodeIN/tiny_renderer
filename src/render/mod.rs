use crate::geometry::core::{Face, Vec3f};
use crate::geometry::ops::barycentric;
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

pub fn new_triangle(
    v0: &mut Vec3f,
    v1: &mut Vec3f,
    v2: &mut Vec3f,
    image: &mut RgbImage,
    color: Rgb<u8>,
) {
    let x_min = 0;
    let x_max = image.width() - 1;
    let y_min = 0;
    let y_max = image.height() - 1;

    for x in (x_min as usize)..=(x_max as usize) {
        for y in (y_min as usize)..=(y_max as usize) {
            let bc_coordinate = barycentric(vec![v0, v1, v2], &Vec3f([x as f64, y as f64, 0.0]));
            if bc_coordinate.iter().all(|num| *num >= 0.0) {
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn triangle(
    v0: &mut Vec3f,
    v1: &mut Vec3f,
    v2: &mut Vec3f,
    image: &mut RgbImage,
    color: Rgb<u8>,
) {
    if v0.y() == v1.y() && v0.y() == v2.y() {
        return;
    }

    if v0.y() > v1.y() {
        std::mem::swap(v0, v1)
    }
    if v0.y() > v2.y() {
        std::mem::swap(v0, v2)
    }
    if v1.y() > v2.y() {
        std::mem::swap(v1, v2)
    }

    let total_height = v2.y() - v0.y();
    for i in 0..=(total_height as u32) {
        let second_half = i > (v1.y() - v0.y()) as u32 || v1.y() == v0.y();
        let segment_height = if second_half {
            v2.y() - v1.y()
        } else {
            v1.y() - v0.y()
        };

        let alpha = i as f64 / total_height;
        let beta = (i as f64 - if second_half { v1.y() - v0.y() } else { 0.0 }) / segment_height;

        let mut a = *v0 + (*v2 - *v0) * alpha;
        let mut b = if second_half {
            *v1 + (*v2 - *v1) * beta
        } else {
            *v0 + (*v1 - *v0) * beta
        };

        if a.x() > b.x() {
            std::mem::swap(&mut a, &mut b)
        };

        for x in (a.x() as u32)..=(b.x() as u32) {
            image.put_pixel(x, (i as f64 + v0.y()) as u32, color);
        }
    }
}

pub fn wire_frame(
    mut image: &mut RgbImage,
    color: Rgb<u8>,
    vertices: &VecDeque<Vec3f>,
    faces: &VecDeque<Face>,
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
