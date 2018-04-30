use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn new(arr: [f64; 3]) -> Vec3 {
        Vec3 { e: arr }
    }

    pub fn apply_per_element<F>(&self, other: Vec3, op: F) -> Vec3
    where
        F: Fn(f64, f64) -> f64,
    {
        Vec3::new([
            op(self.e[0], other.e[0]),
            op(self.e[1], other.e[1]),
            op(self.e[2], other.e[2]),
        ])
    }

    pub fn map<F>(&self, op: F) -> Vec3
    where
        F: Fn(f64) -> f64,
    {
        Vec3::new([op(self.e[0]), op(self.e[1]), op(self.e[2])])
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new([
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            -(self.e[0] * other.e[2] - self.e[2] * other.e[0]),
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        ])
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn unit_vector(&self) -> Vec3 {
        let l = self.length();
        *self / l
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        self.apply_per_element(other, |a, b| a + b)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, k: f64) -> Vec3 {
        self.map(|i| i + k)
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        v.map(|i| i + self)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        self.apply_per_element(other, |a, b| a - b)
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, k: f64) -> Vec3 {
        self.map(|i| i - k)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        self.apply_per_element(other, |a, b| a * b)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, k: f64) -> Vec3 {
        self.map(|i| i * k)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v.map(|i| i * self)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        self.apply_per_element(other, |a, b| a / b)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, k: f64) -> Vec3 {
        self.map(|i| i / k)
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }

    pub fn origin(&self) -> Vec3 {
        self.a
    }

    pub fn direction(&self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + self.b * t
    }
}
