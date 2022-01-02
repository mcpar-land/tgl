use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Pos<T = usize>
where
	T: Copy + PartialEq,
{
	pub x: T,
	pub y: T,
}

impl<T: Copy + PartialEq> Pos<T> {
	pub fn new(x: T, y: T) -> Self {
		Self { x, y }
	}
	pub fn both(v: T) -> Self {
		Self { x: v, y: v }
	}
	pub fn flip(self) -> Self {
		Self {
			x: self.y,
			y: self.x,
		}
	}
	pub fn into<B: Copy + PartialEq + From<T>>(self) -> Pos<B> {
		Pos {
			x: self.x.into(),
			y: self.y.into(),
		}
	}
	pub fn map<B: Copy + PartialEq, F: Fn(T) -> B>(self, f: F) -> Pos<B> {
		Pos {
			x: f(self.x),
			y: f(self.y),
		}
	}
}

impl<T> Add<T> for Pos<T>
where
	T: Add<Output = T> + Copy + PartialEq,
{
	type Output = Self;
	fn add(self, rhs: T) -> Self::Output {
		Pos {
			x: self.x + rhs,
			y: self.y + rhs,
		}
	}
}

impl<T> Add<Pos<T>> for Pos<T>
where
	T: Add<Output = T> + Copy + PartialEq,
{
	type Output = Self;
	fn add(self, rhs: Pos<T>) -> Self::Output {
		Pos {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> Sub<T> for Pos<T>
where
	T: Sub<Output = T> + Copy + PartialEq,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		Pos {
			x: self.x - rhs,
			y: self.y - rhs,
		}
	}
}

impl<T> Sub<Pos<T>> for Pos<T>
where
	T: Sub<Output = T> + Copy + PartialEq,
{
	type Output = Self;
	fn sub(self, rhs: Pos<T>) -> Self::Output {
		Pos {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> Mul<T> for Pos<T>
where
	T: Mul<Output = T> + Copy + PartialEq,
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		Pos {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl<T> Mul<Pos<T>> for Pos<T>
where
	T: Mul<Output = T> + Copy + PartialEq,
{
	type Output = Self;
	fn mul(self, rhs: Pos<T>) -> Self::Output {
		Pos {
			x: self.x * rhs.x,
			y: self.y * rhs.y,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_math_operators() {
		assert_eq!(Pos::new(5, 1) + 10, Pos::new(15, 11));
		assert_eq!(Pos::new(5, 1) - 10, Pos::new(-5, -9));
		assert_eq!(Pos::new(5, 3) * 2, Pos::new(10, 6));

		assert_eq!(Pos::new(5, 1) + Pos::new(4, 7), Pos::new(9, 8));
		assert_eq!(Pos::new(5, 1) - Pos::new(4, 7), Pos::new(1, -6));
		assert_eq!(Pos::new(5, 1) * Pos::new(4, 7), Pos::new(20, 7));
	}

	#[test]
	fn test_into() {
		let _: Pos<f32> = Pos::both(7u8).into();
	}

	#[test]
	fn test_map() {
		let r = Pos::new(10, 5).map(|v| {
			if v > 7 {
				(v * 2) as f32
			} else {
				(v + 1) as f32
			}
		});
		assert_eq!(r, Pos::new(20.0, 6.0));
	}
}
