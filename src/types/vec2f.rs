use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy)]
pub struct Vec2f {
	pub x: f64,
	pub y: f64,
}

impl Vec2f {
	pub fn new(x: f64, y: f64) -> Vec2f {
		Vec2f { x, y }
	}
}

impl Clone for Vec2f {
	fn clone(&self) -> Self {
		*self
	}
}

impl Add for Vec2f {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Sub for Vec2f {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl Mul for Vec2f {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

impl Mul<f64> for Vec2f {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl Div for Vec2f {
	type Output = Self;
	fn div(self, rhs: Self) -> Self {
		Self {
			x: self.x / rhs.x,
			y: self.y / rhs.y,
		}
	}
}

impl Div<f64> for Vec2f {
	type Output = Self;
	fn div(self, rhs: f64) -> Self {
		Self {
			x: self.x / rhs,
			y: self.y / rhs,
		}
	}
}
