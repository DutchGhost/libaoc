//! This a library containing functions, structs, enums, traits and methods for common little problems while solving the Advent of Code.
//!
//! This library has a trait for converting Iterators,
//! a struct and an enum for keeping track of a Position and a Direction,
//! a trait for calculating the `manhatten-distance`,
//! a trait implemented on all integers, that allow to get the absolute value of that integer,
//! and a trait for quickly sorting a tuple in ascending or descending order.
//!
//! Also supports reading tekst from a file into a String, or Vec<u8>, however this is a feature of this library, and is considered unstable.
#![cfg_attr(feature = "nightly", feature(try_from))]

#[macro_use]
pub mod convert;
pub mod absolute;
pub mod movement;

#[cfg(feature = "readfile")]
pub mod reading;

#[cfg(test)]
mod tests;

/// Forces a move. If the `moved` call would be deleted, rust complains that current.next can't be borrowed more than once at a time.
/// # Examples
/// ```
/// struct List {
///     next: Option<Box<List>>
/// }
/// impl List {
///     fn walk_the_list(&mut self) {
///     let mut current = self;
///     loop {
///         match moved(current).next {
///             None => return,
///             Some(ref mut inner) => current = inner,
///             }
///         }
///     }
/// }
/// extern crate libaoc;
/// use libaoc::moved;
/// fn main() {}
/// ```
#[inline(always)]
pub fn moved<T>(x: T) -> T { x }

/// Copies the passed in reference
/// # Examples
/// ```
/// extern crate libaoc;
/// use libaoc::copy;
/// fn main() {
///     let mut n = 10;
///     let mut copy = {
///         let mut mut_ref = &mut n;
///         let copy = copy(mut_ref);
///         *mut_ref += 1;
///         copy
///     };
///     
///     copy += 1;
/// 
///     assert_eq!(copy, n);
/// }
/// ```
#[inline(always)]
pub fn copy<T: Copy>(x: &T) -> T { *x }

/// Clones the passed in reference
/// # Examples
/// ```
/// extern crate libaoc;
/// use libaoc::clone;
/// fn main() {
///     let mut n = String::from("Hello!");
///     let mut clone = {
///         let mut mut_ref = &mut n;
///         let clone = clone(mut_ref);
///         mut_ref.push('!');
///         clone
///     };
///     
///     clone.push('!');
/// 
///     assert_eq!(clone, n);
/// }
/// ```
#[inline(always)]
pub fn clone<T: Clone> (x: &T) -> T { (*x).clone() }

/// Returns a tuple, sorted by the max value.
/// # Examples
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
/// # Examples
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
/// # Examples
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

/// An easy way to combine the solutions of the problems.
/// When this macro is called, a macro-name, day, year and implementation must be given.
/// This macro then creates a macro with the given name, running the implementation when called.
#[macro_export]
macro_rules! aoc {
    ($day_name:ident, $day:expr, $year:expr, $implementation:block) => (
        macro_rules! $day_name {
            () => (
                println!("Running day {} of year {}:", $day, $year);
                $implementation
                println!();
            );
        }
    )
}

/// Applies any given operator to any given tuple.
/// # Examples
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
/// # Examples
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
/// # Examples
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
/// # Examples
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
/// # Examples
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
/// # Examples
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
/// # Examples
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
