use std::ops::{Add, Div, Index, Mul, Sub};
use std::slice::Iter;

#[derive(Clone, Copy)]
pub struct Vec3f(pub [f64; 3]);

impl Index<usize> for Vec3f {
    type Output = f64;
    fn index(&self, _index: usize) -> &Self::Output {
        if _index > 2 {
            panic!(format!("Index out of bounds: {} out of [0,2]", _index));
        }

        &self.0[_index]
    }
}

impl Vec3f {
    pub fn iter(&self) -> Iter<'_, f64> {
        self.0.iter()
    }

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

impl Add for Vec3f {
    type Output = Self;

    fn add(self, other: Vec3f) -> Self {
        Vec3f([
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ])
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, other: Vec3f) -> Self {
        Vec3f([
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        ])
    }
}

impl Sub<f64> for Vec3f {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Vec3f([self.x() - other, self.y() - other, self.z() - other])
    }
}

impl Mul<f64> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3f([self.x() * rhs, self.y() * rhs, self.z() * rhs])
    }
}

impl Div<f64> for Vec3f {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        if rhs == 0.0 {
            panic!("Division by zero");
        }
        Vec3f([self.x() / rhs, self.y() / rhs, self.z() / rhs])
    }
}

pub struct Face(pub [usize; 3]);

impl Face {
    pub fn get_vertex_index(&self, index: usize) -> usize {
        assert!(index < 3);

        self.0[index] - 1
    }
}
