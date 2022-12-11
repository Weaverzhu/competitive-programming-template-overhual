use std::ops::{Add, Mul};
pub trait SemiRing: Add<Output = Self> + Mul<Output = Self> + Copy {
    fn zero() -> Self;
    fn one() -> Self;
}
pub const SIZE: usize = 2;
#[derive(Clone)]
pub struct SquareMatrix<T> {
    buf: [[T; SIZE]; SIZE],
}
impl<T: SemiRing> SquareMatrix<T> {
    pub fn zero() -> Self {
        let z = T::zero();
        SquareMatrix {
            buf: [[z; SIZE]; SIZE],
        }
    }
    pub fn identity() -> Self {
        let mut m = Self::zero();
        for i in 0..SIZE {
            m.buf[i][i] = T::one();
        }
        m
    }
    pub fn matmul(&self, rhs: &Self) -> Self {
        let mut res = Self::zero();
        for (x, a) in res.buf.iter_mut().zip(self.buf.iter()) {
            for (a, b) in a.iter().zip(rhs.buf.iter()) {
                for (x, b) in x.iter_mut().zip(b.iter()) {
                    *x = *x + *a * *b;
                }
            }
        }
        res
    }
    pub fn matadd(&self, rhs: &Self) -> Self {
        let mut c = Self::zero();
        for (c, (a, b)) in c.buf.iter_mut().zip(self.buf.iter().zip(rhs.buf.iter())) {
            for (c, (a, b)) in c.iter_mut().zip(a.iter().zip(b.iter())) {
                *c = *a + *b;
            }
        }
        c
    }
    pub fn matpow(&self, mut n: usize) -> Self {
        let mut t = Self::identity();
        let mut s = self.clone();
        while n > 0 {
            if n & 1 == 1 {
                t = t.matmul(&s);
            }
            s = s.matmul(&s);
            n >>= 1;
        }
        t
    }
}
impl<T> std::ops::Index<usize> for SquareMatrix<T> {
    type Output = [T; SIZE];
    fn index(&self, x: usize) -> &Self::Output {
        &self.buf[x]
    }
}
impl<T> std::ops::IndexMut<usize> for SquareMatrix<T> {
    fn index_mut(&mut self, x: usize) -> &mut Self::Output {
        &mut self.buf[x]
    }
}
