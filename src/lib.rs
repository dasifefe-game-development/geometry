use core::f32::EPSILON;
use core::ops::*;

pub struct Point {
    pub array: [f32; 3],
}

impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Point {}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            array: [
                self.array[0] + other.array[0],
                self.array[1] + other.array[1],
                self.array[2] + other.array[2],
            ],
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            array: [
                self.array[0] - other.array[0],
                self.array[1] - other.array[1],
                self.array[2] - other.array[2],
            ],
        }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            array: [
                self.array[0] * other.array[0],
                self.array[1] * other.array[1],
                self.array[2] * other.array[2],
            ],
        }
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            array: [
                self.array[0] / other.array[0],
                self.array[1] / other.array[1],
                self.array[2] / other.array[2],
            ],
        }
    }
}

impl Point {
    pub fn new(v0: f32, v1: f32, v2: f32) -> Point {
        Point {
            array: [v0, v1, v2],
        }
    }
    pub fn initialize(&mut self) {
        self.array[0] = 0.0;
        self.array[1] = 0.0;
        self.array[2] = 0.0;
    }
    pub fn translate(&mut self, translocation: Point) {
        self.array[0] = self.array[0] + translocation.array[0];
        self.array[1] = self.array[1] + translocation.array[1];
        self.array[2] = self.array[2] + translocation.array[2];
    }
    pub fn translated(&self, translocation: Point) -> Point {
        Point {
            array: [
                self.array[0] + translocation.array[0],
                self.array[1] + translocation.array[1],
                self.array[2] + translocation.array[2],
            ],
        }
    }
    pub fn divide(&mut self, value: f32) {
        self.array[0] = self.array[0] / value;
        self.array[1] = self.array[1] / value;
        self.array[2] = self.array[2] / value;
    }
    pub fn divided(&self, value: f32) -> Point {
        Point {
            array: [
                self.array[0] / value,
                self.array[1] / value,
                self.array[2] / value,
            ],
        }
    }
    pub fn multiply(&mut self, value: f32) {
        self.array[0] = self.array[0] * value;
        self.array[1] = self.array[1] * value;
        self.array[2] = self.array[2] * value;
    }
    pub fn multiplied(&self, value: f32) -> Point {
        Point {
            array: [
                self.array[0] * value,
                self.array[1] * value,
                self.array[2] * value,
            ],
        }
    }
    pub fn normalize(&mut self) {
        let mut maximum: f32 = self.array[0];
        if maximum < self.array[1] {
            maximum = self.array[1];
        }
        if maximum < self.array[2] {
            maximum = self.array[2];
        }
        if maximum > 0.0 {
            self.divide(maximum);
        }
    }
    pub fn cross(&self, other: Point) -> Point {
        Point {
            array: [
                self.array[1] * other.array[2] - self.array[2] * other.array[1],
                self.array[2] * other.array[0] - self.array[0] * other.array[2],
                self.array[0] * other.array[1] - self.array[1] * other.array[0],
            ],
        }
    }
    pub fn dot(&self, other: Point) -> f32 {
        self.array[0] * other.array[0]
            + self.array[1] * other.array[1]
            + self.array[2] * other.array[2]
    }
}

pub struct Sphere {
    pub position: Point,
    pub radius: f32,
}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Sphere {}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            position: Point::new(0.0, 0.0, 0.0),
            radius: 0.0,
        }
    }
    pub fn initialize(&mut self) {
        self.position.initialize();
    }
}

pub struct Ray {
    pub position: Point,
    pub direction: Point,
}

impl Clone for Ray {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Ray {}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            position: Point::new(0.0, 0.0, 0.0),
            direction: Point::new(0.0, 0.0, 0.0),
        }
    }
    pub fn initialize(&mut self) {
        self.position.initialize();
        self.direction.initialize();
    }
}

pub struct Triangle {
    pub array: [Point; 3],
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Triangle {}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            array: [
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 0.0),
                Point::new(0.0, 0.0, 0.0),
            ],
        }
    }
    pub fn initialize(&mut self) {
        self.array[0].initialize();
        self.array[1].initialize();
        self.array[2].initialize();
    }
    pub fn translate(&mut self, translocation: Point) {
        self.array[0].translate(translocation);
        self.array[1].translate(translocation);
        self.array[2].translate(translocation);
    }
}

pub fn contact_ray_triangle(ray: Ray, triangle: Triangle) -> Option<Point> {
    let edge_1: Point = triangle.array[1] - triangle.array[0];
    let edge_2: Point = triangle.array[2] - triangle.array[0];
    let h: Point = ray.direction.cross(edge_2);
    let a: f32 = edge_1.dot(h);
    if a > -EPSILON && a < EPSILON {
        // Ray parallel to triangle.
        return Option::None;
    }
    let f: f32 = 1.0 / a;
    let s: Point = ray.position - triangle.array[0];
    let u: f32 = f * s.dot(h);
    if u < 0.0 || u > 1.0 {
        return Option::None;
    }
    let q: Point = s.cross(edge_1);
    let v: f32 = f * ray.direction.dot(q);
    if v < 0.0 || u + v > 1.0 {
        return Option::None;
    }
    // Find the point of intersection.
    let t: f32 = f * edge_2.dot(q);
    if t > EPSILON {
        return Option::Some(ray.direction.multiplied(t).translated(ray.position));
    }
    // There is a line intersection, but not a ray intersection.
    Option::None
}
