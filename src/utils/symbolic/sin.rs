use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use utils;
use utils::symbolic::{UnivariateFn, SymAdd, SymMult, SymSub, SymNeg, SymComp};
use utils::symbolic;
use math::Scalar;

/// The sinus function.
#[derive(Clone, Copy)]
pub struct Sin;

/// The sinus function.
#[inline]
pub fn sin<A>(a: A) -> SymComp<Sin, A> {
    symbolic::comp(Sin, a)
}

impl<N: Scalar> UnivariateFn<N, N> for Sin {
    #[inline]
    fn d0(&self, t: N) -> N {
        t.sin()
    }

    #[inline]
    fn d1(&self, t: N) -> N {
        t.cos()
    }

    #[inline]
    fn d2(&self, t: N) -> N {
        -t.sin()
    }

    #[inline]
    fn dn(&self, t: N, n: uint) -> N {
        utils::dsin(n, t)
    }
}

impl_ops_noparam!(Sin);
