use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector3 {
    e: [f64; 3],
}

enum Vector3Dim {
    X,
    Y,
    Z,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self[Vector3Dim::X]
    }

    pub fn y(&self) -> f64 {
        self[Vector3Dim::Y]
    }

    pub fn z(&self) -> f64 {
        self[Vector3Dim::Z]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            -(self.x() * rhs.z() - self.z() * rhs.x()),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

// trait implementation
// i.e., implementing the `Index` trait for the `Vector3` type
// seems like a bad idea, especially since we already have the accessors above but i'll just implement it to
// be spec-compliant
impl Index<Vector3Dim> for Vector3 {
    // defines the associated type of `Output` for the `Index` trait
    // allows the trait to be generic and reusable
    type Output = f64;

    fn index(&self, index: Vector3Dim) -> &Self::Output {
        match index {
            Vector3Dim::X => &self.e[0],
            Vector3Dim::Y => &self.e[1],
            Vector3Dim::Z => &self.e[2],
        }
    }
}

impl IndexMut<Vector3Dim> for Vector3 {
    fn index_mut(&mut self, index: Vector3Dim) -> &mut Self::Output {
        match index {
            Vector3Dim::X => &mut self.e[0],
            Vector3Dim::Y => &mut self.e[1],
            Vector3Dim::Z => &mut self.e[2],
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self[Vector3Dim::X] += rhs.x();
        self[Vector3Dim::Y] += rhs.y();
        self[Vector3Dim::Z] += rhs.z();
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[Vector3Dim::X] *= rhs;
        self[Vector3Dim::Y] *= rhs;
        self[Vector3Dim::Z] *= rhs;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

// makes multiplication commutative
impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}
