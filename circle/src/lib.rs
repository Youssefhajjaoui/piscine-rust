use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, rdc: f64) -> Self {
        return Self {
            center: Point(x, y),
            radius: rdc,
        };
    }

    pub fn area(&self) -> f64 {
        return PI * (self.radius * self.radius);
    }

    pub fn diameter(&self) -> f64 {
        return 2.0 * self.radius;
    }

    pub fn intersect(&self, crcl: Self) -> bool {
        let distance = self.center.distance(crcl.center);
        if distance > self.radius + crcl.radius {
            return false;
        }
        return true;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Self) -> f64 {
        return ((self.0 - other.0).powf(2.0) + (self.1 - other.1).powf(2.0)).sqrt();
    }
}
