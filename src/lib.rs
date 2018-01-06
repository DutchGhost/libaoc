use std::str::FromStr;
use std::cmp::*;
use std::fmt::{self, Display, Formatter};
use std::ops;

/// A trait to get the absolute value of a number.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::Absolute;
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
where T: Ord
{   
    fn minmax(self) -> Self;
    fn maxmin(self) -> Self;
}

impl <T> MinMax<T> for (T, T)
where
    T: Ord
{
    fn minmax(self) -> Self {
        if self.0 < self.1 { (self.0, self.1) } else { (self.1, self.0) }
    }
    fn maxmin(self) -> Self {
        if self.0 > self.1 { (self.0, self.1) } else { (self.1, self.0) }
    }
}

/// Takes any Iterator, where the items implement AsRef<str>.
/// Returns a Vec<N>, where N implements FromStr.
/// Returns an error if an error occured.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{splitter};
///
/// fn main () {
///     let s = "1, 2, 3, 4, 5";
///     assert_eq!(vec![1, 2, 3, 4, 5], splitter(s.split(", ")).unwrap());
///
///     let s = String::from("1\n2\n3\n4\n5\n6");
///     assert_eq!(vec![1, 2, 3, 4, 5, 6], splitter(s.lines()).unwrap());
/// }
/// ```
#[inline]
pub fn splitter<N, S, I>(iter: I) -> Result<Vec<N>, <N as FromStr>::Err>
where
    N: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    iter.map(|item| item.as_ref().parse()).collect()
}

/// Takes any Iterator, where the items are implement AsRef<str>.
/// Returns a Vec<N>, where N implements FromStr.
/// Returns an Error if an error occured.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{ToNum};
///
/// fn main () {
///     let s = "1, 2, 3, 4, 5";
///     assert_eq!(vec![1, 2, 3, 4, 5], s.split(", ").to_num().unwrap());
///
///     let s = String::from("1\n2\n3\n4\n5\n6");
///     assert_eq!(vec![1, 2, 3, 4, 5, 6], s.lines().to_num().unwrap());
/// }
/// ```
pub trait ToNum<N, S, I>
where
    N: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    fn to_num(self) -> Result<Vec<N>, <N as FromStr>::Err>;
}

impl <N, S, I> ToNum<N, S, I> for I
where
    N: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    fn to_num(self) -> Result<Vec<N>, <N as FromStr>::Err> {
        self.map(|item| item.as_ref().parse()).collect()
    }
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
    #[inline]
    pub fn init_left() -> Direction {
        Direction::Left
    }

    /// Initializes a direction facing to the right.
    #[inline]
    pub fn init_right() -> Direction {
        Direction::Right
    }

    /// Initializes a direction facing up.
    #[inline]
    pub fn init_up() -> Direction {
        Direction::Up
    }

    /// Initializes a direction facing down.
    #[inline]
    pub fn init_down() -> Direction {
        Direction::Down
    }

    /// turns the direction to the right.
    #[inline]
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    /// turns the direction to the left.
    #[inline]
    pub fn turn_left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    /// Reverses the current direction.
    #[inline]
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

/// A position. Great to use in maps or graphs.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{Position, ManhattenDst};
/// 
/// fn main() {
///     let tup = (-10i32, 21i32);
///     let pos = Position::new(-10i32, 21i32);
///     assert_eq!(Position::from(tup), pos);
///     
///     let tuple = (10u16, 1u16);
///     let position = Position::new(10u16, 1u16);
///     
///     assert_eq!(Position::from(tuple), position);
/// }
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Position<N>
where
    N: ops::Add<N> + ops::AddAssign<N> + ops::Sub<N> + ops::SubAssign<N>,
{
    x: N,
    y: N,
}

impl<N> Position<N>
where
    N: ops::Add<N> + ops::AddAssign<N> + ops::Sub<N> + ops::SubAssign<N>
{
    /// Returns a new Position.
    pub fn new(x: N, y: N) -> Position<N> {
        Position { x: x, y: y }
    }

    /// Changes the position with `steps` based on the direction.
    /// If the direction is facing down, y is incremented, if the direction if facing up, y is decremented.
    /// changes the position by `steps`.
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::{Direction, Position};
    /// fn main() {
    ///     let mut pos = Position::new(0, 0);
    ///     let dir = Direction::init_left();
    ///     
    ///     pos.change(&dir, 1);
    /// 
    ///     let otherpos = Position::new(-1, 0);
    ///     assert_eq!(pos, otherpos);
    /// }
    #[inline]
    pub fn change(&mut self, direction: &Direction, steps: N) {
        match direction {
            &Direction::Up => self.y -= steps,
            &Direction::Down => self.y += steps,
            &Direction::Right => self.x += steps,
            &Direction::Left => self.x -= steps,
        }
    }

    /// Adds `steps` to y.
    /// see [decrement_y](struct.Position.html#method.decrement_y), [increment_x](struct.Position.html#method.increment_x), [decrement_x](struct.Position.html#method.decrement_x)
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::{Absolute, Position};
    /// fn main() {
    ///     let mut pos = Position::new(0, 0);
    ///     pos.increment_y(10);
    ///     assert_eq!(Position::new(0, 10), pos);
    /// }
    /// ```
    #[inline]
    pub fn increment_y(&mut self, steps: N) {
        self.y += steps;
    }

    /// Subtracts `steps` from y.
    /// see [increment_y](struct.Position.html#method.increment_y), [increment_x](struct.Position.html#method.increment_x), [decrement_x](struct.Position.html#method.decrement_x)
    #[inline]
    pub fn decrement_y(&mut self, steps: N) {
        self.y -= steps;
    }

    /// Adds `steps` to x.
    /// see [increment_y](struct.Position.html#method.increment_y), [decrement_y](struct.Position.html#method.decrement_y), [decrement_x](struct.Position.html#method.decrement_x)
    #[inline]
    pub fn increment_x(&mut self, steps: N) {
        self.x += steps;
    }

    /// Subtracts `steps` from x.
    /// see [increment_y](struct.Position.html#method.increment_y), [decrement_y](struct.Position.html#method.decrement_y), [increment_x](struct.Position.html#method.increment_x)
    #[inline]
    pub fn decrement_x(&mut self, steps: N) {
        self.x -= steps;
    }
}

impl<N> Display for Position<N>
where
    N: ops::Add<N>
        + ops::AddAssign<N>
        + ops::Sub<N>
        + ops::SubAssign<N>
        + fmt::Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl <N>From<(N, N)> for Position<N>
where
    N: ops::Add<N> + ops::AddAssign<N> + ops::Sub<N> + ops::SubAssign<N>,
{
    #[inline]
    fn from((n1, n2): (N, N)) -> Position<N> {
        Position {x: n1, y: n2}
    }
}

impl<N> Into<(N, N)> for Position<N>
where
    N: ops::Add<N> + ops::AddAssign<N> + ops::Sub<N> + ops::SubAssign<N>,
{
    #[inline]
    fn into(self) -> (N, N) {
        (self.x, self.y)
    }
}

/// Returns the manhatten distance of any Position with type N.
/// A position is either a tuple, or the struct itself see [Position](struct.Position.html).
/// the manhatten distance is the sum of the absolute values of a coordinate.
///
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{ManhattenDst, Position};
/// 
/// fn main() {
///     let pos = Position::new(-1, 11i16);
///     assert_eq!(12, pos.manhattendst());
/// }
/// ```
pub trait ManhattenDst<N>
where
    N: ops::Add<Output = N>,
{
    fn manhattendst(self) -> N;
}

impl <N> ManhattenDst<N> for Position<N>
where
    N: ops::Add<Output = N> + ops::AddAssign<N> + ops::Sub<N> + ops::SubAssign<N> + Absolute
{
    #[inline]
    fn manhattendst(self) -> N {
        self.x.abs() + self.y.abs()
    }
}

impl <N> ManhattenDst<N> for (N, N)
where
    N: ops::Add<Output = N> + Absolute
{   
    #[inline]
    fn manhattendst(self) -> N {
        self.0.abs() + self.1.abs()
    }
}

impl <N> ManhattenDst<N> for (N, N, N)
where
    N: ops::Add<Output = N> + Absolute,
{   
    #[inline]
    fn manhattendst(self) -> N {
        self.0.abs() + self.1.abs() + self.2.abs()
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