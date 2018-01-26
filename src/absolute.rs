/// A trait to get the absolute value of a number.
/// # Examples
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

macro_rules! rec_i_absolute {
    ($head:ty) => {
        impl Absolute for $head {
            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }
        }
    };
    ($head:ty, $($tail:ty),*) => {
        rec_i_absolute!($head);
        rec_i_absolute!($($tail),*);
    };
}

macro_rules! rec_u_absolute {
    ($head:ty) => {
        impl Absolute for $head {
            #[inline(always)]
            fn abs(self) -> Self {
                self
            }
        }
    };
    ($head:ty, $($tail:ty),*) => {
        rec_u_absolute!($head);
        rec_u_absolute!($($tail),*);
    };
}

//@FIXME!
// macro_rules! rec_u_absolute {
//     ($($tail:ty,)+) => {
//         impl Absolute for $tail {
//             #[inline(always)]
//             fn abs(self) -> Self {
//                 self
//             }
//         }
//         rec_u_absolute!($($tail,)*);
//     };
// }


rec_i_absolute!(i64, i32, i16, i8, isize);
rec_u_absolute!(u64, u32, u16, u8, usize);

impl<N: Absolute> Absolute for (N, N) {
    #[inline]
    fn abs(self) -> Self {
        (self.0.abs(), self.1.abs())
    }
}
