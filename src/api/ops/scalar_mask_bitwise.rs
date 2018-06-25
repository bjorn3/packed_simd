//! Vertical (lane-wise) vector-vector bitwise operations.

macro_rules! impl_ops_scalar_mask_bitwise {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident |
        ($true:expr, $false:expr)
    ) => {
        impl ::ops::BitXor<bool> for $id {
            type Output = Self;
            #[inline]
            fn bitxor(self, other: bool) -> Self {
                self ^ $id::splat(other)
            }
        }
        impl ::ops::BitXor<$id> for bool {
            type Output = $id;
            #[inline]
            fn bitxor(self, other: $id) -> $id {
                $id::splat(self) ^ other
            }
        }

        impl ::ops::BitAnd<bool> for $id {
            type Output = Self;
            #[inline]
            fn bitand(self, other: bool) -> Self {
                self & $id::splat(other)
            }
        }
        impl ::ops::BitAnd<$id> for bool {
            type Output = $id;
            #[inline]
            fn bitand(self, other: $id) -> $id {
                $id::splat(self) & other
            }
        }

        impl ::ops::BitOr<bool> for $id {
            type Output = Self;
            #[inline]
            fn bitor(self, other: bool) -> Self {
                self | $id::splat(other)
            }
        }
        impl ::ops::BitOr<$id> for bool {
            type Output = $id;
            #[inline]
            fn bitor(self, other: $id) -> $id {
                $id::splat(self) | other
            }
        }

        impl ::ops::BitAndAssign<bool> for $id {
            #[inline]
            fn bitand_assign(&mut self, other: bool) {
                *self = *self & other;
            }
        }
        impl ::ops::BitOrAssign<bool> for $id {
            #[inline]
            fn bitor_assign(&mut self, other: bool) {
                *self = *self | other;
            }
        }
        impl ::ops::BitXorAssign<bool> for $id {
            #[inline]
            fn bitxor_assign(&mut self, other: bool) {
                *self = *self ^ other;
            }
        }

        #[cfg(test)]
        interpolate_idents! {
            mod [test_ops_scalar_mask_bitwise_ $id] {
                use super::*;
                #[test]
                fn ops_scalar_mask_bitwise() {
                    let ti = true;
                    let fi = false;
                    let t = $id::splat(ti);
                    let f = $id::splat(fi);
                    assert!(t != f);
                    assert!(!(t == f));

                    // BitAnd:
                    assert_eq!(ti & f, f);
                    assert_eq!(t & fi, f);
                    assert_eq!(fi & t, f);
                    assert_eq!(f & ti, f);
                    assert_eq!(ti & t, t);
                    assert_eq!(t & ti, t);
                    assert_eq!(fi & f, f);
                    assert_eq!(f & fi, f);

                    // BitOr:
                    assert_eq!(ti | f, t);
                    assert_eq!(t | fi, t);
                    assert_eq!(fi | t, t);
                    assert_eq!(f | ti, t);
                    assert_eq!(ti | t, t);
                    assert_eq!(t | ti, t);
                    assert_eq!(fi | f, f);
                    assert_eq!(f | fi, f);

                    // BitXOR:
                    assert_eq!(ti ^ f, t);
                    assert_eq!(t ^ fi, t);
                    assert_eq!(fi ^ t, t);
                    assert_eq!(f ^ ti, t);
                    assert_eq!(ti ^ t, f);
                    assert_eq!(t ^ ti, f);
                    assert_eq!(fi ^ f, f);
                    assert_eq!(f ^ fi, f);

                    {
                        // AndAssign:
                        let mut v = f;
                        v &= ti;
                        assert_eq!(v, f);
                    }
                    {
                        // OrAssign:
                        let mut v = f;
                        v |= ti;
                        assert_eq!(v, t);
                    }
                    {
                        // XORAssign:
                        let mut v = f;
                        v ^= ti;
                        assert_eq!(v, t);
                    }
                }
            }
        }
    };
}