use rand::prelude::*;
use std::ops::Index;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = thread_rng();

        loop {
            let a: f64 = rng.gen();
            let b: f64 = rng.gen();
            let c: f64 = rng.gen();
            let p = 2.0 * Vec3::new([a, b, c]) - vec3(1, 1, 1);

            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = thread_rng();

        loop {
            let a: f64 = rng.gen();
            let b: f64 = rng.gen();
            let p = 2.0 * vec3(a, b, 0.0) - vec3(1, 1, 0);

            if dot(p, p) < 1.0 {
                return p;
            }
        }
    }
}

pub fn vec3<T: Into<f64>>(x: T, y: T, z: T) -> Vec3 {
    Vec3::new([x.into(), y.into(), z.into()])
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn dot(v: Vec3, other: Vec3) -> f64 {
    v.e[0] * other.e[0] + v.e[1] * other.e[1] + v.e[2] * other.e[2]
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = dot(uv, n);
    let discriminant = 1.0 - (ni_over_nt * ni_over_nt) * (1.0 - (dt * dt));

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    #[inline]
    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
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

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self.map(|i| -i)
    }
}

#[derive(Debug, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3, ti: f64) -> Ray {
        Ray { a, b, time: ti }
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

    pub fn time(&self) -> f64 {
        self.time
    }
}
