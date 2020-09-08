use crate::geometry::core::{Face, Vec3f};
use crate::geometry::ops::{barycentric, cross_product, dot_product};
use image::{Rgb, RgbImage, Rgba, RgbaImage};
use rand::Rng;
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

pub fn triangle(v0: &Vec3f, v1: &Vec3f, v2: &Vec3f, image: &mut RgbImage, color: Rgb<u8>) {
    // find bounding box
    let (x_min, x_max, y_min, y_max) =
        find_bounding_box(v0, v1, v2, image.width() as usize, image.height() as usize);

    // iterate over points in bounding box and paint ones which are in the triangle
    for x in (x_min as usize)..=(x_max as usize) {
        for y in (y_min as usize)..=(y_max as usize) {
            let bc_coordinate = barycentric(vec![v0, v1, v2], &Vec3f([x as f64, y as f64, 0.0]));
            if bc_coordinate.iter().all(|num| *num >= 0.0) {
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn triangle_with_gamma(
    v0: &Vec3f,
    v1: &Vec3f,
    v2: &Vec3f,
    image: &mut RgbaImage,
    color: Rgba<u8>,
) {
    // find bounding box
    let (x_min, x_max, y_min, y_max) =
        find_bounding_box(v0, v1, v2, image.width() as usize, image.height() as usize);

    // iterate over points in bounding box and paint ones which are in the triangle
    for x in (x_min as usize)..=(x_max as usize) {
        for y in (y_min as usize)..=(y_max as usize) {
            let bc_coordinate = barycentric(vec![v0, v1, v2], &Vec3f([x as f64, y as f64, 0.0]));
            if bc_coordinate.iter().all(|num| *num >= 0.0) {
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn triangle_with_gamma_smart(
    v0: &Vec3f,
    v1: &Vec3f,
    v2: &Vec3f,
    z_buffer: &mut Vec<f64>,
    image: &mut RgbaImage,
    color: Rgba<u8>,
) {
    // find bounding box
    let (x_min, x_max, y_min, y_max) =
        find_bounding_box(v0, v1, v2, image.width() as usize, image.height() as usize);

    for x in (x_min as usize)..=(x_max as usize) {
        for y in (y_min as usize)..=(y_max as usize) {
            let bc_coordinate = barycentric(vec![v0, v1, v2], &Vec3f([x as f64, y as f64, 0.0]));
            if bc_coordinate.iter().any(|num| *num < 0.0) {
                continue;
            }
            let z =
                v0.z() * bc_coordinate[0] + v1.z() * bc_coordinate[1] + v2.z() * bc_coordinate[2];
            if z_buffer[x + y * (image.width()) as usize] < z {
                z_buffer[x + y * (image.width()) as usize] = z;
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

fn find_bounding_box(
    v0: &Vec3f,
    v1: &Vec3f,
    v2: &Vec3f,
    width: usize,
    height: usize,
) -> (usize, usize, usize, usize) {
    let mut x_values = vec![v0.x(), v1.x(), v2.x()];
    x_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut y_values = vec![v0.y(), v1.y(), v2.y()];
    y_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let x_min = std::cmp::max(0, x_values[0] as usize);
    let x_max = std::cmp::min((width - 1) as usize, x_values[2] as usize);

    let y_min = std::cmp::max(0, y_values[0] as usize);
    let y_max = std::cmp::min((height - 1) as usize, y_values[2] as usize);

    (x_min, x_max, y_min, y_max)
}

pub fn old_triangle(
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

pub fn with_light(
    image: &mut RgbaImage,
    vertices: &VecDeque<Vec3f>,
    faces: &VecDeque<Face>,
    light_dir: Vec3f,
) {
    let width = image.width();
    let height = image.height();

    for face in faces {
        let mut screen_vertices = Vec::with_capacity(3);
        let mut world_vertices = Vec::with_capacity(3);

        for i in 0..3 {
            let vertex: &Vec3f = &vertices[face.get_vertex_index(i)];

            let x0 = (vertex.x() + 1.0) * (width / 2) as f64;
            let y0 = (vertex.y() + 1.0) * (height / 2) as f64;

            screen_vertices.push(Vec3f([x0, y0, 0.0]));
            world_vertices.push(*vertex);
        }

        let side1 = world_vertices[2] - world_vertices[0];
        let side2 = world_vertices[1] - world_vertices[0];

        let mut n = cross_product(side1, side2);
        n.normalize();

        let intensity = dot_product(n, light_dir);

        if intensity > 0.0 {
            let color_intensity = (255.0 * intensity) as u8;

            triangle_with_gamma(
                &screen_vertices[0],
                &screen_vertices[1],
                &screen_vertices[2],
                image,
                Rgba([color_intensity, color_intensity, color_intensity, 255]),
            );
        }
    }
}

pub fn with_light_smart(
    image: &mut RgbaImage,
    vertices: &VecDeque<Vec3f>,
    faces: &VecDeque<Face>,
    light_dir: Vec3f,
) {
    let width = image.width();
    let height = image.height();

    let mut z_buffer: Vec<f64> = vec![f64::NEG_INFINITY; (image.width() * image.height()) as usize];

    for face in faces {
        let mut screen_vertices = Vec::with_capacity(3);
        let mut world_vertices = Vec::with_capacity(3);

        for i in 0..3 {
            let vertex: &Vec3f = &vertices[face.get_vertex_index(i)];

            let x0 = (vertex.x() + 1.0) * (width / 2) as f64;
            let y0 = (vertex.y() + 1.0) * (height / 2) as f64;

            screen_vertices.push(Vec3f([x0, y0, 0.0]));
            world_vertices.push(*vertex);
        }

        let side1 = world_vertices[2] - world_vertices[0];
        let side2 = world_vertices[1] - world_vertices[0];

        let mut n = cross_product(side1, side2);
        n.normalize();

        let intensity = dot_product(n, light_dir);

        if intensity > 0.0 {
            let color_intensity = (255.0 * intensity) as u8;

            triangle_with_gamma_smart(
                &screen_vertices[0],
                &screen_vertices[1],
                &screen_vertices[2],
                &mut z_buffer,
                image,
                Rgba([color_intensity, color_intensity, color_intensity, 255]),
            );
        }
    }
}

pub fn flat_shade(image: &mut RgbImage, vertices: &VecDeque<Vec3f>, faces: &VecDeque<Face>) {
    let width = image.width();
    let height = image.height();

    let mut rng = rand::thread_rng();

    for face in faces {
        let mut screen_vertices = Vec::with_capacity(3);

        for i in 0..3 {
            let vertex = &vertices[face.get_vertex_index(i)];

            let x0 = (vertex.x() + 1.0) * (width / 2) as f64;
            let y0 = (vertex.y() + 1.0) * (height / 2) as f64;

            screen_vertices.push(Vec3f([x0, y0, 0.0]));
        }

        triangle(
            &screen_vertices[0],
            &screen_vertices[1],
            &screen_vertices[2],
            image,
            Rgb([
                rng.gen_range(0, 254),
                rng.gen_range(0, 254),
                rng.gen_range(0, 254),
            ]),
        )
    }
}
