use std::str::FromStr;
use std::cmp::*;
use std::fmt::{self, Display, Formatter};
use std::ops::*;

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

/// Takes any Iterator, where the items implement AsRef<str>.
/// Returns a Vec<N>, where N implements FromStr.
/// Returns an Error if an error occured.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{TryConvert};
///
/// fn main () {
///     let s = "1, 2, 3, 4, 5";
///     assert_eq!(vec![1, 2, 3, 4, 5], s.split(", ").try_convert().unwrap());
///
///     let s = String::from("1\n2\n3\n4\n5\n6");
///     assert_eq!(vec![1, 2, 3, 4, 5, 6], s.lines().try_convert().unwrap());
///
///     let my_str = "1, 2, 3,4, 5";
///     let mut iter = my_str.split(", ").try_convert_iter();
///
///     assert_eq!(Some(Ok(1)), iter.next());
///     assert_eq!(Some(Ok(2)), iter.next());
/// 
///     //the next would be 3, but that gives an error. assert_NE.
///     assert_ne!(Some(Ok(3)), iter.next());
/// 
///     //due to the split call, 4 isn't part of the stream, so next one is 5.
///     assert_eq!(Some(Ok(5)), iter.next());
/// }
/// ```
pub trait TryConvert<N, S, I>
where
    N: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    /// On succes, returns a vector of all completed conversions. When an error occures, returns an error instead.
    fn try_convert(self) -> Result<Vec<N>, <N as FromStr>::Err>;

    /// On succes, returns a vector of all completed conversions.
    /// #Panic
    /// Panics when an error occures.
    unsafe fn unsafe_convert(self) -> Vec<N>
    where
        <N as FromStr>::Err: fmt::Debug;
    
    /// Returns an iterator over the converted items. Returns an error if an item can not be converted. Continue's after the error.
    fn try_convert_iter(self) -> std::iter::Map<I, fn(S) -> Result<N, <N as FromStr>::Err>>;

    /// Returns an iterator over the converted items.
    /// #Panic
    /// Panics when an error occures.
    unsafe fn unsafe_convert_iter(self) -> std::iter::Map<I, fn(S) -> N>
    where
        <N as FromStr>::Err: fmt::Debug;
}

impl<N, S, I> TryConvert<N, S, I> for I
where
    N: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{   
    #[inline]
    fn try_convert(self) -> Result<Vec<N>, <N as FromStr>::Err> {
        self.try_convert_iter().collect()
    }

    #[inline]
    unsafe fn unsafe_convert(self) -> Vec<N>
    where
        <N as FromStr>::Err: fmt::Debug,
    {
        self.unsafe_convert_iter().collect()
    }

    #[inline]
    fn try_convert_iter(self) -> std::iter::Map<I, fn(S) -> Result<N, <N as FromStr>::Err>> {
        self.map(|item| item.as_ref().parse())
    }

    #[inline]
    unsafe fn unsafe_convert_iter(self) -> std::iter::Map<I, fn(S) -> N>
    where
        <N as FromStr>::Err: fmt::Debug
    {
        self.map(|item| item.as_ref().parse().unwrap())
    }
}

/// Used to convert a stream of T into a Vec of U.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::{Convert, Position};
/// fn main() {
///     let tuple1 = (0, 0);
///     let tuple2 = (1, 1);
///     let tuple3 = (2, 2);
///
///     let tuples = vec![tuple1, tuple2, tuple3];
///
///     let Positions: Vec<Position<usize>> = tuples.into_iter().convert();
///
///     assert_eq!(vec![Position::new(0, 0), Position::new(1, 1), Position::new(2, 2)], Positions);
/// 
///     let tups = vec![(4, 4), (5, 5), (3, 4)];
/// 
///     let mut convert_iter = tups.into_iter().rev().convert_iter();
/// 
///     assert_eq!(Some(Position::new(3, 4)), convert_iter.next());
/// }
/// ```
pub trait Convert<T, U, I>
where
    U: From<T>,
    I: Iterator<Item = T>,
{
    /// Returns a vector of all completed conversions.
    fn convert(self) -> Vec<U>;
    
    /// Returns an iterator that performs the conversions.
    fn convert_iter(self) -> std::iter::Map<I, fn(T) -> U>;
}

impl<T, U, I> Convert<T, U, I> for I
where
    U: From<T>,
    I: Iterator<Item = T>,
{
    #[inline]
    fn convert(self) -> Vec<U> {
        self.convert_iter().collect()
    }

    #[inline]
    fn convert_iter(self) -> std::iter::Map<I, fn(T) -> U> {
        self.map(|item| U::from(item))
    }
}

/// An enum to represent a direction.
/// Is great to use in maps, or when 'following' some kind of line.
/// Also supports an init variant.
/// When the current variant is [`init`], using [`turn_right`] returns the variant [`right`].
/// When the current variant is [`init`], using [`turn_left`] returns the variant [`left`].
/// [`turn_right`]: enum.Direction.html#method.turn_right
/// [`turn_left`]: enum.Direction.html#method.turn_left
/// [`init`]: enum.Direction.html#variant.Init
/// [`right`]: enum.Direction.html#variant.Right
/// [`left`]: enum.Direction.html#variant.Left
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    Init,
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

    /// Initializes a direction that has no facing yet.
    #[inline]
    pub fn init_init() -> Direction {
        Direction::Init
    }

    /// turns the direction to the right.
    #[inline]
    pub fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Init => Direction::Right,
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
            Direction::Init => Direction::Left,
        }
    }

    /// Reverses the current direction.
    /// #Panic
    /// Panics whenever the current direction is [`Init`](enum.Direction.html#variant.Init).
    #[inline]
    pub fn reverse(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Init => panic!("Reversing a Direction::Init is not possible"),
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
/// use libaoc::{Position, ManhattenDst, Absolute};
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
///
///     let p1 = Position::new(10, 20);
///     let p2 = Position::new(20, 30);
///
///     assert_eq!(Position::new(10, 10), p2 - p1);
///
///     let rp1 = &Position::new(10, 20);
///     let otherp2 = Position::new(-20, 30);
///
///     let p3 = otherp2 - rp1;
///     assert_eq!(Position::new(-30, 10), p3);
///
///     assert_eq!(Position::new(30, 10), p3.abs());
///
///     assert_eq!(Position::new(-10, 40), p3 + p2);
/// }
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Position<N> {
    x: N,
    y: N,
}
macro_rules! binops {
    (impl $imp:ident, $method:ident for $pos:ident, $oper:tt) => {

        // impl Imp<pos<N>> for pos<N>. Does Not require Clone, because the value is owned.
        impl<N> $imp<$pos<N>> for $pos<N>
        where
            N: $imp<Output = N>
        {
            type Output = $pos<N>;

            #[inline]
            fn $method(self, other: $pos<N>) -> Self::Output {
                $pos { x: self.x $oper other.x, y: self.y $oper other.y}
            }
        }

        // impl <'a> Imp<pos<N>> for 'a pos<N>
        impl<'a, N> $imp<$pos<N>> for &'a $pos<N>
        where
            N: $imp<Output = N> + Clone
        {
            type Output = $pos<N>;

            #[inline]
            fn $method(self, other: $pos<N>) -> Self::Output {
                $pos { x: self.x.clone() $oper other.x, y: self.y.clone() $oper other.y}
            }
        }

        // impl <'b> Imp<&'b pos<N>> for pos<N>
        impl<'b, N> $imp<&'b $pos<N>> for $pos<N>
        where
            N: $imp<Output = N> + Clone
        {
            type Output = $pos<N>;

            #[inline]
            fn $method(self, other: &'b $pos<N>) -> Self::Output {
                $pos { x: self.x $oper other.x.clone(), y: self.y $oper other.y.clone()}
            }
        }

        // impl <'a, 'b> Imp<'b pos<N>> for 'a pos<N>
        impl<'a, 'b, N> $imp<&'b $pos<N>> for &'a $pos<N>
        where
            N: $imp<Output = N> + Clone
        {
            type Output = $pos<N>;

            #[inline]
            fn $method(self, other: &'b $pos<N>) -> Self::Output {
                $pos { x: self.x.clone() $oper other.x.clone(), y: self.y.clone() $oper other.y.clone()}
            }
        }
    }
}

binops!(impl Add, add for Position, +);
binops!(impl Sub, sub for Position, -);

impl<N> Position<N>
where
    N: AddAssign<N> + SubAssign<N>,
{
    /// Returns a new Position.
    #[inline]
    pub fn new(x: N, y: N) -> Position<N> {
        Position { x: x, y: y }
    }

    /// Changes the position with `steps` based on the direction.
    /// If the direction is facing down, `y` is incremented, if the direction if facing up, `y` is decremented.
    /// If the direction is [`Direction::Init`], no update is made.
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::{Direction, Position};
    /// fn main() {
    ///     let mut pos = Position::new(0, 0);
    ///     let dir = Direction::init_up();
    ///
    ///     pos.change(&dir, 1);
    ///
    ///     let otherpos = Position::new(0, -1);
    ///     assert_eq!(pos, otherpos);
    /// }
    /// ```
    ///
    /// [`Direction::Init`]: enum.Direction.html#variant.Init
    #[inline]
    pub fn change(&mut self, direction: &Direction, steps: N) {
        match direction {
            &Direction::Up => self.y -= steps,
            &Direction::Down => self.y += steps,
            &Direction::Right => self.x += steps,
            &Direction::Left => self.x -= steps,
            &Direction::Init => return,
        }
    }

    /// Same as [`change`], but now increments `y` when facing upwards, and decrements `y` when facing downwards.
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::{Direction, Position};
    /// fn main() {
    ///     let mut pos = Position::new(0, 0);
    ///     let dir = Direction::init_up();
    ///
    ///     pos.rev_change(&dir, 1);
    ///
    ///     let otherpos = Position::new(0, 1);
    ///     assert_eq!(pos, otherpos);
    /// }
    /// ```
    ///
    /// [`change`]: #method.change
    #[inline]
    pub fn rev_change(&mut self, direction: &Direction, steps: N) {
        match direction {
            &Direction::Up => self.y += steps,
            &Direction::Down => self.y -= steps,
            &Direction::Right => self.x += steps,
            &Direction::Left => self.x -= steps,
            &Direction::Init => return,
        }
    }

    /// Check whether self and `other` are adjecent. That is, if the absolute x value and the absolute y value after subtracting `self` from `other`
    /// is either (1, 0), (0, 1) or (1, 1).
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::Position;
    /// fn main() {
    ///     let pos1 = Position::new(5, 6);
    ///     let pos2 = Position::new(6, 7);
    ///
    ///
    ///     assert_eq!(true, pos1.is_adjecent(&pos2));
    ///     assert_eq!(true, pos2.is_adjecent(&pos1));
    ///
    ///     let pos3 = Position::new(-1, 0);
    ///     let pos4 = Position::new(0, 0);
    ///
    ///     assert_eq!(true, pos3.is_adjecent(&pos4));
    ///     assert_eq!(true, pos4.is_adjecent(&pos3));
    ///
    ///     assert_eq!(false, pos4.is_adjecent(&pos4));
    ///     assert_eq!(false, pos3.is_adjecent(&pos1));
    /// }
    /// ```
    #[inline]
    pub fn is_adjecent<'a, 'b>(&'a self, other: &'b Position<N>) -> bool
    where
        N: Sub<Output = N> + Clone + From<i8> + PartialEq + Absolute,
    {
        match (self - other).abs() {
            Position { ref x, ref y } if x == &N::from(0) && y == &N::from(1) => true,
            Position { ref x, ref y } if x == &N::from(1) && y == &N::from(0) => true,
            Position { ref x, ref y } if x == &N::from(1) && y == &N::from(1) => true,
            _ => false,
        }
    }

    /// Adds `steps` to y.
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
    #[inline]
    pub fn decrement_y(&mut self, steps: N) {
        self.y -= steps;
    }

    /// Adds `steps` to x.
    #[inline]
    pub fn increment_x(&mut self, steps: N) {
        self.x += steps;
    }

    /// Subtracts `steps` from x.
    #[inline]
    pub fn decrement_x(&mut self, steps: N) {
        self.x -= steps;
    }

    /// Returns a reference to the current x value.
    #[inline]
    pub fn get_ref_x(&self) -> &N {
        &self.x
    }

    /// Returns a reference to the current y value.
    #[inline]
    pub fn get_ref_y(&self) -> &N {
        &self.y
    }

    /// Returns a tuple of &x, &y.
    #[inline]
    pub fn get_ref(&self) -> (&N, &N) {
        (&self.x, &self.y)
    }

    /// Clones x and y into a tuple.
    #[inline]
    pub fn to_tuple(&self) -> (N, N)
    where
        N: Clone,
    {
        self.clone().into()
    }
}

impl<N: Absolute> Absolute for Position<N> {
    #[inline]
    fn abs(self) -> Self {
        Position {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<N: fmt::Display> Display for Position<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<N> From<(N, N)> for Position<N> {
    #[inline]
    fn from((n1, n2): (N, N)) -> Position<N> {
        Position { x: n1, y: n2 }
    }
}

impl<N> Into<(N, N)> for Position<N> {
    #[inline]
    fn into(self) -> (N, N) {
        (self.x, self.y)
    }
}

impl<N: Absolute> Absolute for (N, N) {
    #[inline]
    fn abs(self) -> Self {
        (self.0.abs(), self.1.abs())
    }
}

/// Returns the manhatten distance of any Position with type N.
/// A position is either a tuple, or the struct [Position](struct.Position.html).
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
    N: Add<Output = N>,
{
    fn manhattendst(self) -> N;
}

impl<N> ManhattenDst<N> for Position<N>
where
    N: Add<Output = N> + Absolute,
{
    #[inline]
    fn manhattendst(self) -> N {
        self.x.abs() + self.y.abs()
    }
}

impl<N> ManhattenDst<N> for (N, N)
where
    N: Add<Output = N> + Absolute,
{
    #[inline]
    fn manhattendst(self) -> N {
        self.0.abs() + self.1.abs()
    }
}

impl<N> ManhattenDst<N> for (N, N, N)
where
    N: Add<Output = N> + Absolute,
{
    #[inline]
    fn manhattendst(self) -> N {
        self.0.abs() + self.1.abs() + self.2.abs()
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
