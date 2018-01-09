/// A trait to get the absolute value of a number.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::absolute::Absolute;
/// fn main() {
///     assert_eq!(10, (-10i32).abs());
///     assert_eq!(20, 20u32.abs());
/// }
/// ```
pub trait Absolute {
    fn abs(self) -> Self;
}

/// Used to implement `Absolute` for any i.. integer type.
/// Any i.. integer type can be negative, so .abs() is needed in order to return the absolute value.
macro_rules! i_absolute {
    ($type:ty) => (
        impl Absolute for $type {

            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
        }
    )
}

/// Used to implement `Absolute` for any u.. integer type.
/// Any u.. integer type can not be negative, so `self` is already the absolute value.
macro_rules! u_absolute {
    ($type:ty) => (
        impl Absolute for $type {

            #[inline]
            fn abs(self) -> Self {
                self
            }
        }
    )
}

i_absolute!(i64);
i_absolute!(i32);
i_absolute!(i16);
i_absolute!(i8);
i_absolute!(isize);

u_absolute!(u64);
u_absolute!(u32);
u_absolute!(u16);
u_absolute!(u8);
u_absolute!(usize);

impl<N: Absolute> Absolute for (N, N) {
    #[inline]
    fn abs(self) -> Self {
        (self.0.abs(), self.1.abs())
    }
}
