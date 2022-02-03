use crate::direction::{Angle, Direction};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
    pub const fn new(cs: [T; N]) -> Self {
        Self(cs)
    }
}

// floating point 2-vectors
impl Vector<f32, 2> {
    pub const fn unit(angle: Angle) -> Self {
        Self::new([angle.cos(), angle.sin()])
    }

    pub const fn in_direction(direction: Direction, length: f32) -> Self {
        let unit = Self::unit(direction.angle_to(Direction::East));
        Self::new([unit.0[0] * length, unit.0[1] * length])
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

macro_rules! impl_vector_op {
    (unary $op:ident :: $method:ident) => {
        impl<T, O, const N: usize> std::ops::$op for Vector<T, N>
        where
            T: std::ops::$op<Output = O>,
        {
            type Output = Vector<O, N>;
            fn $method(self) -> Self::Output {
                Vector(self.0.map(|c| std::ops::$op::$method(c)))
            }
        }
    };
    // unfortunately, I don't believe it's possible to implement both $op<U> and $op<Vector<U>>
    // whenever T: $op<U> because the two impls overlap causing recursion even with feature(specialization)
    (binary $op:ident :: $method:ident) => {
        impl<T, U, O, const N: usize> std::ops::$op<U> for Vector<T, N>
        where
            U: Copy,
            T: std::ops::$op<U, Output = O>,
        {
            type Output = Vector<O, N>;
            fn $method(self, other: U) -> Self::Output {
                Vector(self.0.map(|c| std::ops::$op::$method(c, other)))
            }
        }

        impl_vector_op!(binary_componentwise $op::$method for i8, i16, i32, i64, i128, isize);
        impl_vector_op!(binary_componentwise $op::$method for u8, u16, u32, u64, u128, usize);
        impl_vector_op!(binary_componentwise $op::$method for f32, f64);
    };
    (binary_componentwise $op:ident :: $method:ident for $( $ty:ty ),*) => {
        $(
            impl<const N: usize> std::ops::$op for Vector<$ty, N> {
                type Output = Vector<$ty, N>;
                fn $method(self, other: Self) -> Self::Output {
                    Vector(
                        self.0
                            .zip(other.0)
                            .map(|(a, b)| std::ops::$op::$method(a, b)),
                    )
                }
            }
        )*
    };
}

impl_vector_op!(unary Not::not);
impl_vector_op!(unary Neg::neg);
impl_vector_op!(binary Add::add);
impl_vector_op!(binary Sub::sub);
impl_vector_op!(binary Mul::mul);
impl_vector_op!(binary Div::div);
impl_vector_op!(binary Rem::rem);
