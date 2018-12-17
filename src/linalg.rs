#![allow(clippy::many_single_char_names)]

#[cfg(feature = "approx")]
use approx;
use num_traits;
use std::fmt;
use std::mem;
use std::ops;

/// A 3x3 matrix used for linear color transformations
#[derive(Copy, Debug, PartialEq)]
pub struct Matrix3<T> {
    /// An array containing the cell values
    pub m: [T; 9],
}

impl<T> Clone for Matrix3<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        unsafe {
            let mut new_arr: [T; 9] = mem::uninitialized();
            new_arr.clone_from_slice(&self.m);

            Matrix3 { m: new_arr }
        }
    }
}

impl<T> Matrix3<T>
where
    T: num_traits::Num + Copy + num_traits::Zero + num_traits::NumCast,
{
    /// Construct a new `Matrix3` from a list of values (row major)
    #[inline]
    pub fn new(values: [T; 9]) -> Self {
        Matrix3 { m: values }
    }

    /// Construct a new `Matrix3` with all zero entries
    #[inline]
    pub fn zero() -> Self {
        Matrix3 { m: [T::zero(); 9] }
    }

    /// Construct an identity matrix
    #[inline]
    pub fn identity() -> Self {
        let one = num_traits::cast(1.0).unwrap();
        let zero = num_traits::cast(0.0).unwrap();
        Matrix3::new([one, zero, zero, zero, one, zero, zero, zero, one])
    }

    /// Construct a new `Matrix3` with all values set to `val`
    #[inline]
    pub fn broadcast(val: T) -> Self {
        Matrix3 { m: [val; 9] }
    }

    /// Return a slice to the elements in the matrix
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.m
    }
    #[inline]
    /// Return a mutable slice to the elements in the matrix
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.m
    }
    #[inline]
    /// Return a nine element tuple containing the elements
    pub fn to_tuple(self) -> (T, T, T, T, T, T, T, T, T) {
        unsafe {
            (
                *self.m.get_unchecked(0),
                *self.m.get_unchecked(1),
                *self.m.get_unchecked(2),
                *self.m.get_unchecked(3),
                *self.m.get_unchecked(4),
                *self.m.get_unchecked(5),
                *self.m.get_unchecked(6),
                *self.m.get_unchecked(7),
                *self.m.get_unchecked(8),
            )
        }
    }

    /// Compute the determinant of the matrix
    #[inline]
    pub fn determinant(&self) -> T {
        let (a, b, c, d, e, f, g, h, i) = self.clone().to_tuple();

        a * e * i + b * f * g + c * d * h - c * e * g - b * d * i - a * f * h
    }

    /// Transpose the matrix
    #[inline]
    pub fn transpose(self) -> Self {
        let (a, b, c, d, e, f, g, h, i) = self.to_tuple();

        Matrix3 {
            m: [a, d, g, b, e, h, c, f, i],
        }
    }

    /// Compute the inverse of the matrix
    ///
    /// None is returned if the matrix is singular (determinant = 0)
    #[inline]
    pub fn inverse(self) -> Option<Self> {
        let det = self.determinant();
        let (a, b, c, d, e, f, g, h, i) = self.to_tuple();

        let ca = e * i - f * h;
        let cb = f * g - d * i;
        let cc = d * h - e * g;
        let cd = c * h - b * i;
        let ce = a * i - c * g;
        let cf = b * g - a * h;
        let cg = b * f - c * e;
        let ch = c * d - a * f;
        let ci = a * e - b * d;

        let cofactor_transpose = Matrix3::new([ca, cd, cg, cb, ce, ch, cc, cf, ci]);

        if det != num_traits::cast(0).unwrap() {
            Some(cofactor_transpose * (num_traits::cast::<_, T>(1).unwrap() / det))
        } else {
            None
        }
    }

    /// Transform a vector using `self`
    #[inline]
    pub fn transform_vector<U>(&self, vec: (U, U, U)) -> (U, U, U)
    where
        U: num_traits::NumCast,
    {
        let (v1, v2, v3) = vec;
        let fv1: T = num_traits::cast(v1).unwrap();
        let fv2: T = num_traits::cast(v2).unwrap();
        let fv3: T = num_traits::cast(v3).unwrap();

        let (m1, m2, m3, m4, m5, m6, m7, m8, m9) = self.clone().to_tuple();

        let fo1 = fv1 * m1 + fv2 * m2 + fv3 * m3;
        let fo2 = fv1 * m4 + fv2 * m5 + fv3 * m6;
        let fo3 = fv1 * m7 + fv2 * m8 + fv3 * m9;

        let o1: U = num_traits::cast(fo1).unwrap();
        let o2: U = num_traits::cast(fo2).unwrap();
        let o3: U = num_traits::cast(fo3).unwrap();

        (o1, o2, o3)
    }
}

impl<T> Default for Matrix3<T>
where
    T: num_traits::Num + Copy + Default,
{
    fn default() -> Self {
        Matrix3 {
            m: [T::default(); 9],
        }
    }
}

#[cfg(feature = "approx")]
impl<T> approx::AbsDiffEq for Matrix3<T>
where
    T: num_traits::Num + Copy + approx::AbsDiffEq,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.m
            .iter()
            .zip(other.m.iter())
            .fold(true, move |st, (lhs, rhs)| {
                st && lhs.abs_diff_eq(&rhs, epsilon.clone())
            })
    }
}
#[cfg(feature = "approx")]
impl<T> approx::RelativeEq for Matrix3<T>
where
    T: num_traits::Num + Copy + approx::RelativeEq,
    T::Epsilon: Clone,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.m
            .iter()
            .zip(other.m.iter())
            .fold(true, move |st, (lhs, rhs)| {
                st && lhs.relative_eq(&rhs, epsilon.clone(), max_relative.clone())
            })
    }
}
#[cfg(feature = "approx")]
impl<T> approx::UlpsEq for Matrix3<T>
where
    T: num_traits::Num + Copy + approx::UlpsEq,
    T::Epsilon: Clone,
{
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.m
            .iter()
            .zip(other.m.iter())
            .fold(true, move |st, (lhs, rhs)| {
                st && lhs.ulps_eq(&rhs, epsilon.clone(), max_ulps)
            })
    }
}
impl<T> ops::Div<T> for Matrix3<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;
    #[inline]
    fn div(self, rhs: T) -> Self {
        let mut output = self;
        output /= rhs;
        output
    }
}

impl<T> ops::DivAssign<T> for Matrix3<T>
where
    T: num_traits::Num + Copy,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        unsafe {
            *self.m.get_unchecked_mut(0) = *self.m.get_unchecked(0) / rhs;
            *self.m.get_unchecked_mut(1) = *self.m.get_unchecked(1) / rhs;
            *self.m.get_unchecked_mut(2) = *self.m.get_unchecked(2) / rhs;
            *self.m.get_unchecked_mut(3) = *self.m.get_unchecked(3) / rhs;
            *self.m.get_unchecked_mut(4) = *self.m.get_unchecked(4) / rhs;
            *self.m.get_unchecked_mut(5) = *self.m.get_unchecked(5) / rhs;
            *self.m.get_unchecked_mut(6) = *self.m.get_unchecked(6) / rhs;
            *self.m.get_unchecked_mut(7) = *self.m.get_unchecked(7) / rhs;
            *self.m.get_unchecked_mut(8) = *self.m.get_unchecked(8) / rhs;
        }
    }
}

impl<T> ops::Mul for Matrix3<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Matrix3<T>) -> Self {
        let mut output = self;
        output *= rhs;
        output
    }
}
impl<T> ops::Mul<T> for Matrix3<T>
where
    T: num_traits::Num + Copy,
{
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self {
        let mut output = self;
        output *= rhs;
        output
    }
}

impl<T> ops::MulAssign<T> for Matrix3<T>
where
    T: num_traits::Num + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        unsafe {
            *self.m.get_unchecked_mut(0) = *self.m.get_unchecked(0) * rhs;
            *self.m.get_unchecked_mut(1) = *self.m.get_unchecked(1) * rhs;
            *self.m.get_unchecked_mut(2) = *self.m.get_unchecked(2) * rhs;
            *self.m.get_unchecked_mut(3) = *self.m.get_unchecked(3) * rhs;
            *self.m.get_unchecked_mut(4) = *self.m.get_unchecked(4) * rhs;
            *self.m.get_unchecked_mut(5) = *self.m.get_unchecked(5) * rhs;
            *self.m.get_unchecked_mut(6) = *self.m.get_unchecked(6) * rhs;
            *self.m.get_unchecked_mut(7) = *self.m.get_unchecked(7) * rhs;
            *self.m.get_unchecked_mut(8) = *self.m.get_unchecked(8) * rhs;
        }
    }
}

impl<T> ops::MulAssign for Matrix3<T>
where
    T: num_traits::Num + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Matrix3<T>) {
        // We know the exact size of the array for all matrices
        // and need to index it many times. Thus, we use
        // unchecked indexing to help performance.
        unsafe {
            let (l1, l2, l3, l4, l5, l6, l7, l8, l9) = (
                *self.m.get_unchecked(0),
                *self.m.get_unchecked(1),
                *self.m.get_unchecked(2),
                *self.m.get_unchecked(3),
                *self.m.get_unchecked(4),
                *self.m.get_unchecked(5),
                *self.m.get_unchecked(6),
                *self.m.get_unchecked(7),
                *self.m.get_unchecked(8),
            );
            let (r1, r2, r3, r4, r5, r6, r7, r8, r9) = (
                *rhs.m.get_unchecked(0),
                *rhs.m.get_unchecked(1),
                *rhs.m.get_unchecked(2),
                *rhs.m.get_unchecked(3),
                *rhs.m.get_unchecked(4),
                *rhs.m.get_unchecked(5),
                *rhs.m.get_unchecked(6),
                *rhs.m.get_unchecked(7),
                *rhs.m.get_unchecked(8),
            );

            *self.m.get_unchecked_mut(0) = l1 * r1 + l2 * r4 + l3 * r7;
            *self.m.get_unchecked_mut(1) = l1 * r2 + l2 * r5 + l3 * r8;
            *self.m.get_unchecked_mut(2) = l1 * r3 + l2 * r6 + l3 * r9;

            *self.m.get_unchecked_mut(3) = l4 * r1 + l5 * r4 + l6 * r7;
            *self.m.get_unchecked_mut(4) = l4 * r2 + l5 * r5 + l6 * r8;
            *self.m.get_unchecked_mut(5) = l4 * r3 + l5 * r6 + l6 * r9;

            *self.m.get_unchecked_mut(6) = l7 * r1 + l8 * r4 + l9 * r7;
            *self.m.get_unchecked_mut(7) = l7 * r2 + l8 * r5 + l9 * r8;
            *self.m.get_unchecked_mut(8) = l7 * r3 + l8 * r6 + l9 * r9;
        }
    }
}

impl<T> ops::Add for Matrix3<T>
where
    T: num_traits::Num + Copy + ops::AddAssign<T>,
{
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        let mut output = self;
        output += rhs;
        output
    }
}

impl<T> ops::AddAssign for Matrix3<T>
where
    T: num_traits::Num + Copy + ops::AddAssign<T>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        unsafe {
            *self.m.get_unchecked_mut(0) += *rhs.m.get_unchecked(0);
            *self.m.get_unchecked_mut(1) += *rhs.m.get_unchecked(1);
            *self.m.get_unchecked_mut(2) += *rhs.m.get_unchecked(2);
            *self.m.get_unchecked_mut(3) += *rhs.m.get_unchecked(3);
            *self.m.get_unchecked_mut(4) += *rhs.m.get_unchecked(4);
            *self.m.get_unchecked_mut(5) += *rhs.m.get_unchecked(5);
            *self.m.get_unchecked_mut(6) += *rhs.m.get_unchecked(6);
            *self.m.get_unchecked_mut(7) += *rhs.m.get_unchecked(7);
            *self.m.get_unchecked_mut(8) += *rhs.m.get_unchecked(8);
        }
    }
}

impl<T> ops::Sub for Matrix3<T>
where
    T: num_traits::Num + Copy + ops::SubAssign<T>,
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        let mut output = self;
        output -= rhs;
        output
    }
}

impl<T> ops::SubAssign for Matrix3<T>
where
    T: num_traits::Num + Copy + ops::SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        unsafe {
            *self.m.get_unchecked_mut(0) -= *rhs.m.get_unchecked(0);
            *self.m.get_unchecked_mut(1) -= *rhs.m.get_unchecked(1);
            *self.m.get_unchecked_mut(2) -= *rhs.m.get_unchecked(2);
            *self.m.get_unchecked_mut(3) -= *rhs.m.get_unchecked(3);
            *self.m.get_unchecked_mut(4) -= *rhs.m.get_unchecked(4);
            *self.m.get_unchecked_mut(5) -= *rhs.m.get_unchecked(5);
            *self.m.get_unchecked_mut(6) -= *rhs.m.get_unchecked(6);
            *self.m.get_unchecked_mut(7) -= *rhs.m.get_unchecked(7);
            *self.m.get_unchecked_mut(8) -= *rhs.m.get_unchecked(8);
        }
    }
}

impl<T> fmt::Display for Matrix3<T>
where
    T: num_traits::Num + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|{} {} {}|\n|{} {} {}|\n|{} {} {}|",
            self.m[0],
            self.m[1],
            self.m[2],
            self.m[3],
            self.m[4],
            self.m[5],
            self.m[6],
            self.m[7],
            self.m[8]
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mul() {
        let m1 = Matrix3::<f32>::zero();
        let m2 = Matrix3::zero();
        assert_eq!(m1 * m2, Matrix3::zero());
        let m3 = Matrix3::<f32>::identity();
        let m4 = Matrix3::identity();
        assert_eq!(m3 * m4, Matrix3::identity());
        assert_eq!(m3 * m1, Matrix3::zero());
        let m5 = Matrix3::broadcast(1.0f32);
        assert_eq!(m5 * 2.0, Matrix3::broadcast(2.0));
        assert_eq!(m5 * m5, Matrix3::broadcast(3.0f32));
        assert_eq!(m5 * m3, m5);
        let m6 = Matrix3::new([1, 0, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(m6 * m6, Matrix3::new([2, 0, 2, 0, 1, 0, 2, 0, 2]));
        assert_eq!(m6 * Matrix3::identity(), m6);
    }

    #[test]
    fn test_add() {
        let m1 = Matrix3::broadcast(1.0) + Matrix3::broadcast(2.0);
        assert_eq!(m1, Matrix3::broadcast(3.0));
        let m2: Matrix3<f32> = Matrix3::identity() - Matrix3::identity();
        assert_eq!(m2, Matrix3::zero());
    }

    #[test]
    fn test_invert() {
        let m1 = Matrix3::<f32>::identity();
        assert_eq!(m1, m1.inverse().unwrap());

        let m2 = Matrix3::broadcast(1.0f32);
        assert_eq!(m2.inverse(), None);
        assert_eq!(m2.determinant(), 0.0);

        let m3 = Matrix3::new([1.0, 2.0, 1.0, 4.0, 2.0, 3.0, 1.0, 3.0, 1.0]);
        let m4 = Matrix3::new([-7.0, 1.0, 4.0, -1.0, 0.0, 1.0, 10.0, -1.0, -6.0]);
        assert_eq!(m3.inverse().unwrap(), m4);
        assert_eq!(m3.inverse().unwrap() * m3, Matrix3::identity());
    }
}
