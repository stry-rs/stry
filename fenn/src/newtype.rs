/// A macro for easy creation of [`newtype`]s.
///
/// # Note
///
/// Newtypes created by this macro are `sealed`, meaning they **do not**
/// implement [`AsMut`], [`AsRef`], [`Deref`], or [`DerefMut`].
///
/// Doing so would invalidate any use of a `newtype`.
///
/// [`newtype`]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
/// [`AsMut`]: https://doc.rust-lang.org/stable/std/convert/trait.AsMut.html
/// [`AsRef`]: https://doc.rust-lang.org/stable/std/convert/trait.AsRef.html
/// [`Deref`]: https://doc.rust-lang.org/stable/std/ops/trait.Deref.html
/// [`DerefMut`]: https://doc.rust-lang.org/stable/std/ops/trait.DerefMut.html
#[macro_export]
macro_rules! newtype {
    ($( #[$attrs:meta] )* $name:ident $( : $default:ty )?) => {
        $( #[$attrs] )*
        #[derive(Debug)]
        pub struct $name<T $( = $default )?>(pub T);

        impl<T> ::core::default::Default for $name<T>
        where
            T: ::core::default::Default,
        {
            fn default() -> Self {
                Self(T::default())
            }
        }

        // impl<T> ::core::convert::From<T> for $name<T> {
        //     fn from(other: T) -> Self {
        //         Self(other)
        //     }
        // }

        //#region[rgba(186,255,201,0.05)] copy/clone
        impl<T> ::core::clone::Clone for $name<T>
        where
            T: ::core::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl<T> ::core::marker::Copy for $name<T>
        where
            T: ::core::marker::Copy,
        {}
        //#endregion

        //#region [rgba(186,225,255,0.05)] eq/ord
        impl<T> ::core::cmp::PartialEq for $name<T>
        where
            T: ::core::cmp::PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<T> ::core::cmp::Eq for $name<T>
        where
            T: ::core::cmp::Eq,
        {}

        impl<T> ::core::cmp::PartialOrd for $name<T>
        where
            T: ::core::cmp::PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        impl<T> ::core::cmp::Ord for $name<T>
        where
            T: ::core::cmp::Ord,
        {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
        //#endregion

        impl<T> ::core::ops::Not for $name<T>
        where
            T: ::core::ops::Not<Output = T>,
        {
            type Output = Self;

            fn not(self) -> Self {
                Self(!self.0)
            }
        }

        impl<T> ::core::ops::Neg for $name<T>
        where
            T: ::core::ops::Neg<Output = T>,
        {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }

        impl<T> ::core::hash::Hash for $name<T>
        where
            T: ::core::hash::Hash,
        {
            fn hash<H:>(&self, state: &mut H)
            where
                H: ::core::hash::Hasher,
            {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }

        //#region [rgba(255,179,186,0.05)] operations
        impl<T> ::core::ops::Add for $name<T>
        where
            T: ::core::ops::Add<Output = T>,
        {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl<T> ::core::ops::AddAssign for $name<T>
        where
            T: ::core::ops::AddAssign,
        {
            fn add_assign(&mut self, other: Self) {
                self.0 += other.0;
            }
        }

        impl<T> ::core::ops::BitAnd for $name<T>
        where
            T: ::core::ops::BitAnd<Output = T>,
        {
            type Output = Self;

            fn bitand(self, other: Self) -> Self::Output {
                Self(self.0 & other.0)
            }
        }

        impl<T> ::core::ops::BitAndAssign  for $name<T>
        where
            T: ::core::ops::BitAndAssign + ::core::ops::BitAnd<Output = T>,
        {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0
            }
        }

        impl<T> ::core::ops::BitOr for $name<T>
        where
            T: ::core::ops::BitOr<Output = T>,
        {
            type Output = Self;

            fn bitor(self, other: Self) -> Self::Output {
                Self(self.0 | other.0)
            }
        }

        impl<T> ::core::ops::BitOrAssign  for $name<T>
        where
            T: ::core::ops::BitOrAssign + ::core::ops::BitOr<Output = T>,
        {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0
            }
        }

        impl<T> ::core::ops::BitXor for $name<T>
        where
            T: ::core::ops::BitXor<Output = T>,
        {
            type Output = Self;

            fn bitxor(self, other: Self) -> Self::Output {
                Self(self.0 ^ other.0)
            }
        }

        impl<T> ::core::ops::BitXorAssign  for $name<T>
        where
            T: ::core::ops::BitXorAssign + ::core::ops::BitXor<Output = T>,
        {
            fn bitxor_assign(&mut self, other: Self) {
                self.0 ^= other.0
            }
        }

        impl<T> ::core::ops::Div for $name<T>
        where
            T: ::core::ops::Div<Output = T>,
        {
            type Output = Self;

            fn div(self, other: Self) -> Self::Output {
                Self(self.0 / other.0)
            }
        }

        impl<T> ::core::ops::DivAssign for $name<T>
        where
            T: ::core::ops::DivAssign,
        {
            fn div_assign(&mut self, other: Self) {
                self.0 /= other.0
            }
        }

        impl<T> ::core::ops::Mul for $name<T>
        where
            T: ::core::ops::Mul<Output = T>,
        {
            type Output = Self;

            fn mul(self, other: Self) -> Self::Output {
                Self(self.0 * other.0)
            }
        }

        impl<T> ::core::ops::MulAssign for $name<T>
        where
            T: ::core::ops::MulAssign,
        {
            fn mul_assign(&mut self, other: Self) {
                self.0 *= other.0
            }
        }

        impl<T> ::core::ops::Rem for $name<T>
        where
            T: ::core::ops::Rem<Output = T>,
        {
            type Output = Self;

            fn rem(self, other: Self) -> Self::Output {
                Self(self.0 % other.0)
            }
        }

        impl<T> ::core::ops::RemAssign for $name<T>
        where
            T: ::core::ops::RemAssign,
        {
            fn rem_assign(&mut self, other: Self) {
                self.0 %= other.0
            }
        }

        impl<T> ::core::ops::Sub for $name<T>
        where
            T: ::core::ops::Sub<Output = T>,
        {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self(self.0 - other.0)
            }
        }

        impl<T> ::core::ops::SubAssign for $name<T>
        where
            T: ::core::ops::SubAssign,
        {
            fn sub_assign(&mut self, other: Self) {
                self.0 -= other.0
            }
        }

        impl<T> ::core::ops::Shl for $name<T>
        where
            T: ::core::ops::Shl<Output = T>,
        {
            type Output = Self;

            fn shl(self, other: Self) -> Self::Output {
                Self(self.0 << other.0)
            }
        }

        impl<T> ::core::ops::ShlAssign for $name<T>
        where
            T: ::core::ops::ShlAssign,
        {
            fn shl_assign(&mut self, other: Self) {
                self.0 <<= other.0
            }
        }

        impl<T> ::core::ops::Shr for $name<T>
        where
            T: ::core::ops::Shr<Output = T>,
        {
            type Output = Self;

            fn shr(self, other: Self) -> Self::Output {
                Self(self.0 >> other.0)
            }
        }

        impl<T> ::core::ops::ShrAssign for $name<T>
        where
            T: ::core::ops::ShrAssign,
        {
            fn shr_assign(&mut self, other: Self) {
                self.0 >>= other.0
            }
        }
        //#endregion
    };
}
