mod convert;
mod absolute;
mod movement;

pub mod Convert {
    pub use convert::*;
}

pub mod Absolute {
    pub use absolute::*;
}

pub mod Movement {
    pub use movement::*;
}

pub use Convert::*;
pub use Absolute::*;
pub use Movement::*;

/// Returns a tuple, sorted by the max value.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{sort_biggest};
///
/// #[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
/// struct mytype(i64);
///
/// fn main() {
///
///     let a = &mut mytype(10);
///     let b = &mut mytype(20);
///     assert_eq!((&b, &a), sort_biggest(&a, &b))
/// }
/// ```
#[inline]
pub fn sort_biggest<T: Ord>(a: T, b: T) -> (T, T) {
    if a > b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Returns a tuple, sorted by the min value.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{sort_smallest};
///
/// #[derive(Ord, PartialOrd, PartialEq, Eq, Debug)]
/// struct mytype {
///     item: i64,
///     message: String,
/// }
///
/// fn main() {
///
///     let a = &mytype {item: 10, message: String::from("hello")};
///     let b = &mytype {item: 11, message: String::from("world!")};
///     assert_eq!((&a, &b), sort_smallest(&a, &b))
/// }
/// ```
#[inline]
pub fn sort_smallest<T: Ord>(a: T, b: T) -> (T, T) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

/// 2 functions to 'sort' a tuple.
/// `minmax` returns the tuple in ascending order, `maxmin` in descending order.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::MinMax;
/// fn main() {
///     let mut tup = (20, 10);
///     let mutborrow = &mut tup;
///
///     assert_eq!((10, 20), mutborrow.minmax());
///
///     let mut a = 1;
///     let mut b = 2;
///     let t = (&mut a, &mut b);
///
///     assert_eq!((&mut 2, &mut 1), t.maxmin());
/// }
/// ```
pub trait MinMax<T>
where
    T: Ord,
{
    fn minmax(self) -> Self;
    fn maxmin(self) -> Self;
}

impl<T> MinMax<T> for (T, T)
where
    T: Ord,
{
    #[inline]
    fn minmax(self) -> Self {
        if self.0 < self.1 {
            self
        } else {
            (self.1, self.0)
        }
    }

    #[inline]
    fn maxmin(self) -> Self {
        if self.0 > self.1 {
            self
        } else {
            (self.1, self.0)
        }
    }
}

#[cfg(feature = "readfile")]
pub mod readfile {
    use std::fs::File;
    use std::io::{self, BufReader};
    use std::io::prelude::*;
    use std::path::Path;
    use std::ffi::OsStr;

    fn into_buf_reader<S: AsRef<OsStr>>(s: S) -> Result<BufReader<File>, io::Error> {
        let path: &Path = Path::new(s.as_ref());
        let f = File::open(path)?;
        Ok(BufReader::new(f))
    }
    /// Opens a file, an reads it to whatever type it was called on.
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::readfile::ReadFile;
    /// fn main() {
    ///     let puzzle = match Vec::<u8>::read_file(r"test.txt") {
    ///         Ok(content) => content,
    ///         Err(_) => Vec::new(),
    ///     };
    ///     assert_eq!(b"hello! this is a test!"[..], puzzle[..]);
    /// }
    /// ```
    pub trait ReadFile {
        type Content;

        fn read_file<S: AsRef<OsStr>>(s: S) -> Result<Self::Content, io::Error>;
    }

    impl ReadFile for String {
        type Content = String;
        fn read_file<S: AsRef<OsStr>>(path: S) -> Result<Self::Content, io::Error> {
            let mut s = String::new();
            let mut bufreader = into_buf_reader(path)?;
            bufreader.read_to_string(&mut s)?;
            Ok(s)
        }
    }

    impl<T> ReadFile for Vec<T> {
        type Content = Vec<u8>;
        fn read_file<S: AsRef<OsStr>>(path: S) -> Result<Self::Content, io::Error> {
            let mut v: Vec<u8> = Vec::new();
            let mut bufreader = into_buf_reader(path)?;
            bufreader.read_to_end(&mut v)?;
            Ok(v)
        }
    }
}

/// Applies any given operator to any given tuple.
/// #Examples
/// ```
/// #[macro_use(apply)]
/// extern crate libaoc;
///
/// fn main() {
///     assert_eq!(10, apply!(+, (5, 4, 1)));
///     assert_eq!(0, apply!(-, (5, 4, 1)));
/// }
#[macro_export]
macro_rules! apply {
    ($oper:tt, ( $first:expr $(, $rest:expr)* )) => { apply!( @inner ($oper) ($first) ($($rest,)*) ) };
    (@inner ($oper:tt) ($prev:expr) ($curr:expr, $($rest:expr,)*) ) => { apply!( @inner ($oper) ($prev $oper $curr) ($($rest,)*) ) };
    (@inner ($oper:tt)($final:expr) ()) => { $final };
}
/// Subtracts all items in a tuple.
/// #Examples
/// ```
/// #[macro_use(sub, apply)]
/// extern crate libaoc;
///
/// fn main() {
///     assert_eq!(0, sub!((5, 4, 1)));
/// }
/// ```
#[macro_export]
macro_rules! sub {
    ($tup:tt) => (apply!(-, $tup))
}
/// Adds all items in a tuple.
/// #Examples
/// ```
/// #[macro_use(add, apply)]
/// extern crate libaoc;
///
/// fn main() {
///     assert_eq!(10, add!((5, 4, 1)));
/// }
/// ```
#[macro_export]
macro_rules! add {
    ($tup:tt) => (apply!(+, $tup))
}
/// Divides all items in a tuple. Panics if divided by 0.
/// #Examples
/// ```
/// #[macro_use(div, apply)]
/// extern crate libaoc;
///
/// fn main() {
///     assert_eq!(2, div!((8, 4, 1)));
/// }
/// ```
#[macro_export]
macro_rules! div {
    ($tup:tt) => (apply!(/, $tup))
}
/// Multiplies all items in a tuple.
/// #Examples
/// ```
/// #[macro_use(mul, apply)]
/// extern crate libaoc;
///
/// fn main() {
///     assert_eq!(80, mul!((8, 5, 2)));
/// }
/// ```
#[macro_export]
macro_rules! mul {
    ($tup:tt) => (apply!(*, $tup))
}
/// 'Remainders' all items in a tuple.
/// #Examples
/// ```
/// #[macro_use(rem, apply)]
/// extern crate libaoc;
///
/// fn main() {
///     assert_eq!(2, rem!((5, 10, 3)));
/// }
/// ```
#[macro_export]
macro_rules! rem {
    ($tup:tt) => (apply!(%, $tup))
}

/// This macro is used to generate `noop` functions.
/// Works only on tuples, but is great to use in functional programming when you just need to pass a `noop` function into another function.
/// #Examples
/// ```
/// #[macro_use]
/// extern crate libaoc;
/// fn main() {
///     let f = noop!(&mut (i64, i64));
///     assert_eq!((10, 10), take_func(f, (10, 10)));
///
///     let real_func = |n: &mut (i64, i64)| {n.0 += 1; n.1 += 2;};
///     assert_eq!((11, 12), take_func(real_func, (10, 10)));
/// }
/// fn take_func<F>(f: F, mut n: (i64, i64)) -> (i64, i64)
/// where
///     F: Fn(&mut (i64, i64))
/// {
///     f(&mut n);
///     n
/// }
/// ```
#[macro_export]
macro_rules! noop {
    ($type:ty) => {
        |_: $type| {}
    };
}
