#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LorentzVector {
    pub e: f64,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
}

macro_rules! impl_add_op {
    ($op:ident, $fun:ident) => {
        impl $op<LorentzVector> for LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: LorentzVector) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other.e),
                    px: self.px.$fun(other.px),
                    py: self.py.$fun(other.py),
                    pz: self.pz.$fun(other.pz),
                }
            }
        }
        impl<'a> $op<&'a LorentzVector> for LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: &'a LorentzVector) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other.e),
                    px: self.px.$fun(other.px),
                    py: self.py.$fun(other.py),
                    pz: self.pz.$fun(other.pz),
                }
            }
        }
        impl<'a> $op<LorentzVector> for &'a LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: LorentzVector) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other.e),
                    px: self.px.$fun(other.px),
                    py: self.py.$fun(other.py),
                    pz: self.pz.$fun(other.pz),
                }
            }
        }
        impl<'a, 'b> $op<&'b LorentzVector> for &'a LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: &'b LorentzVector) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other.e),
                    px: self.px.$fun(other.px),
                    py: self.py.$fun(other.py),
                    pz: self.pz.$fun(other.pz),
                }
            }
        }
    }
}

macro_rules! impl_add_op_assign {
    ($op:ident, $fun:ident) => {
        impl $op for LorentzVector {
            fn $fun(&mut self, other: LorentzVector) {
                self.e.$fun(other.e);
                self.px.$fun(other.px);
                self.py.$fun(other.py);
                self.pz.$fun(other.pz);
            }
        }
        impl<'a> $op<&'a LorentzVector> for LorentzVector {
            fn $fun(&mut self, other: &'a LorentzVector) {
                self.e.$fun(other.e);
                self.px.$fun(other.px);
                self.py.$fun(other.py);
                self.pz.$fun(other.pz);
            }
        }
    }
}

impl_add_op!(Add, add);
impl_add_op_assign!(AddAssign, add_assign);
impl_add_op!(Sub, sub);
impl_add_op_assign!(SubAssign, sub_assign);

macro_rules! impl_mul_op {
    ($op:ident, $fun:ident) => {
        impl $op<f64> for LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: f64) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other),
                    px: self.px.$fun(other),
                    py: self.py.$fun(other),
                    pz: self.pz.$fun(other),
                }
            }
        }
        impl<'a> $op<&'a f64> for LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: &'a f64) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other),
                    px: self.px.$fun(other),
                    py: self.py.$fun(other),
                    pz: self.pz.$fun(other),
                }
            }
        }
        impl<'a> $op<f64> for &'a LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: f64) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other),
                    px: self.px.$fun(other),
                    py: self.py.$fun(other),
                    pz: self.pz.$fun(other),
                }
            }
        }
        impl<'a, 'b> $op<&'b f64> for &'a LorentzVector {
            type Output = LorentzVector;
            fn $fun(self, other: &'b f64) -> LorentzVector {
                LorentzVector {
                    e: self.e.$fun(other),
                    px: self.px.$fun(other),
                    py: self.py.$fun(other),
                    pz: self.pz.$fun(other),
                }
            }
        }
    }
}

macro_rules! impl_mul_op_assign {
    ($op:ident, $fun:ident) => {
        impl $op<f64> for LorentzVector {
            fn $fun(&mut self, other: f64) {
                self.e.$fun(other);
                self.px.$fun(other);
                self.py.$fun(other);
                self.pz.$fun(other);
            }
        }
        impl<'a> $op<&'a f64> for LorentzVector {
            fn $fun(&mut self, other: &'a f64) {
                self.e.$fun(other);
                self.px.$fun(other);
                self.py.$fun(other);
                self.pz.$fun(other);
            }
        }
    }
}

impl_mul_op!(Mul, mul);
impl_mul_op_assign!(MulAssign, mul_assign);

impl Mul<LorentzVector> for f64 {
    type Output = LorentzVector;
    fn mul(self, other: LorentzVector) -> LorentzVector {
        LorentzVector {
            e: other.e * self,
            px: other.px * self,
            py: other.py * self,
            pz: other.pz * self,
        }
    }
}

impl<'a> Mul<&'a LorentzVector> for f64 {
    type Output = LorentzVector;
    fn mul(self, other: &'a LorentzVector) -> LorentzVector {
        LorentzVector {
            e: other.e * self,
            px: other.px * self,
            py: other.py * self,
            pz: other.pz * self,
        }
    }
}

impl<'a> Mul<LorentzVector> for &'a f64 {
    type Output = LorentzVector;
    fn mul(self, other: LorentzVector) -> LorentzVector {
        LorentzVector {
            e: other.e * self,
            px: other.px * self,
            py: other.py * self,
            pz: other.pz * self,
        }
    }
}

impl<'a, 'b> Mul<&'b LorentzVector> for &'a f64 {
    type Output = LorentzVector;
    fn mul(self, other: &'b LorentzVector) -> LorentzVector {
        LorentzVector {
            e: other.e * self,
            px: other.px * self,
            py: other.py * self,
            pz: other.pz * self,
        }
    }
}

impl_mul_op!(Div, div);
impl_mul_op_assign!(DivAssign, div_assign);

#[cfg(test)]
mod tests {
    mod arithmetic {
        use LorentzVector;

        const A: LorentzVector = LorentzVector {
            e: 1.,
            px: 2.,
            py: 3.,
            pz: 4.,
        };

        const B: LorentzVector = LorentzVector {
            e: 5.,
            px: 6.,
            py: 7.,
            pz: 8.,
        };

        const C: LorentzVector = LorentzVector {
            e: 6.,
            px: 8.,
            py: 10.,
            pz: 12.,
        };

        const A2: LorentzVector = LorentzVector {
            e: 2.,
            px: 4.,
            py: 6.,
            pz: 8.,
        };

        const B3: LorentzVector = LorentzVector {
            e: -15.,
            px: -18.,
            py: -21.,
            pz: -24.,
        };

        #[test]
        fn add() {
            assert_eq!(A + B, C);
            assert_eq!(&A + B, C);
            assert_eq!(A + &B, C);
            assert_eq!(&A + &B, C);
            assert_eq!(B + A, C);
            assert_eq!(B + &A, C);
            assert_eq!(&B + A, C);
            assert_eq!(&B + &A, C);
        }

        #[test]
        fn add_assign() {
            let mut a = A.clone();
            a += B;
            assert_eq!(a, C);

            let mut a = A.clone();
            a += &B;
            assert_eq!(a, C);

            let mut b = B.clone();
            b += A;
            assert_eq!(b, C);

            let mut b = B.clone();
            b += &A;
            assert_eq!(b, C);
        }

        #[test]
        fn sub() {
            assert_eq!(C - B, A);
            assert_eq!(&C - B, A);
            assert_eq!(C - &B, A);
            assert_eq!(&C - &B, A);
            assert_eq!(C - A, B);
            assert_eq!(&C - A, B);
            assert_eq!(C - &A, B);
            assert_eq!(&C - &A, B);
        }

        #[test]
        fn sub_assign() {
            let mut c = C.clone();
            c -= A;
            assert_eq!(c, B);

            let mut c = C.clone();
            c -= &A;
            assert_eq!(c, B);

            let mut c = C.clone();
            c -= B;
            assert_eq!(c, A);

            let mut c = C.clone();
            c -= &B;
            assert_eq!(c, A);
        }

        #[test]
        fn mul() {
            assert_eq!(A * 2., A2);
            assert_eq!(&A * 2., A2);
            assert_eq!(A * &2., A2);
            assert_eq!(&A * &2., A2);
            assert_eq!(2. * A, A2);
            assert_eq!(&2. * A, A2);
            assert_eq!(2. * &A, A2);
            assert_eq!(&2. * &A, A2);

            assert_eq!(B * -3., B3);
            assert_eq!(&B * -3., B3);
            assert_eq!(B * &-3., B3);
            assert_eq!(&B * &-3., B3);
            assert_eq!(-3. * B, B3);
            assert_eq!(&-3. * B, B3);
            assert_eq!(-3. * &B, B3);
            assert_eq!(&-3. * &B, B3);
        }

        #[test]
        fn mul_assign() {
            let mut a = A.clone();
            a *= 2.;
            assert_eq!(a, A2);

            let mut a = A.clone();
            a *= &2.;
            assert_eq!(a, A2);

            let mut b = B.clone();
            b *= -3.;
            assert_eq!(b, B3);

            let mut b = B.clone();
            b *= &-3.;
            assert_eq!(b, B3);
        }

        #[test]
        fn div() {
            assert_eq!(A2 / 2., A);
            assert_eq!(&A2 / 2., A);
            assert_eq!(A2 / &2., A);
            assert_eq!(&A2 / &2., A);

            assert_eq!(B3 / -3., B);
            assert_eq!(&B3 / -3., B);
            assert_eq!(B3 / &-3., B);
            assert_eq!(&B3 / &-3., B);
        }

        #[test]
        fn div_assign() {
            let mut a = A2.clone();
            a /= 2.;
            assert_eq!(a, A);

            let mut a = A2.clone();
            a /= &2.;
            assert_eq!(a, A);

            let mut b = B3.clone();
            b /= -3.;
            assert_eq!(b, B);

            let mut b = B3.clone();
            b /= &-3.;
            assert_eq!(b, B);
        }
    }
}
