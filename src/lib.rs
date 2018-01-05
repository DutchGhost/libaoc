use std::str::FromStr;
use std::cmp::*;
use std::fmt::{self, Display, Formatter};
use std::ops;

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
    if a > b { (a, b) } else { (b, a) }
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
    if a > b { (b, a) } else { (a, b) }
}

/// Takes any Iterator, where the items are an &'a str.
/// Returns a Vec<N>, where N implements FromStr.
/// Panics if an error occures.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{splitter};
///
/// fn main () {
///     let s = "1, 2, 3, 4, 5";
///     assert_eq!(vec![1, 2, 3, 4, 5], splitter(s.split(", ")).unwrap());
///
///     let s = "1\n2\n3\n4\n5\n6";
///     assert_eq!(vec![1, 2, 3, 4, 5, 6], splitter(s.lines()).unwrap());
/// }
/// ```
#[inline]
pub fn splitter<'a, S: AsRef<str>, I: Iterator<Item = S>, N: FromStr>(iter: I) -> Result<Vec<N>, <N as FromStr>::Err>
{
    iter.map(|item| item.as_ref().parse()).collect()
}

/// An enum to reprisent a direction.
/// Is great to use in maps, or when 'following' some kind of line.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    /// Initializes a direction facing to the left.
    pub fn init_left() -> Direction {
        Direction::Left
    }

    /// Initializes a direction facing to the right.
    pub fn init_right() -> Direction {
        Direction::Right
    }

    /// Initializes a direction facing up.
    pub fn init_up() -> Direction {
        Direction::Up
    }

    /// Initializes a direction facing down.
    pub fn init_down() -> Direction {
        Direction::Down
    }

    /// turns the direction to the right.
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    /// turns the direction to the left.
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    /// Reverses the current direction.
    pub fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Right,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Position<N>
where
    N: ops::Add<N> + ops::AddAssign<N> + ops::Sub<N> + ops::SubAssign<N> + From<u8>,
{
    x: N,
    y: N,
}

impl<N> Position<N>
where
    N: ops::Add<N> + ops::AddAssign<N> + ops::Sub<N> + ops::SubAssign<N> + From<u8>,
{
    /// Returns a new Position.
    pub fn new(x: N, y: N) -> Position<N> {
        Position { x: x, y: y }
    }

    /// Changes the position based on the direction.
    /// If the direction is facing down, y is incremented, if the direction if facing up, y is decremented.
    /// changes the position by `steps`.
    pub fn change(&mut self, direction: &Direction, steps: N) {
        match direction {
            &Direction::Up => self.y -= steps,
            &Direction::Down => self.y += steps,
            &Direction::Right => self.x += steps,
            &Direction::Left => self.x -= steps,
        }
    }
}

impl<N> Display for Position<N>
where
    N: ops::Add<N>
        + ops::AddAssign<N>
        + ops::Sub<N>
        + ops::SubAssign<N>
        + From<u8>
        + fmt::Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Applies any given operator to any given tuple.
/// #Examples
/// ```
/// #[macro_use]
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
/// #[macro_use]
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
/// #[macro_use]
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
/// #[macro_use]
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
/// #[macro_use]
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
/// #[macro_use]
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