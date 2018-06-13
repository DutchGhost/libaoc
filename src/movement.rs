use absolute::Absolute;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// An enum to represent a direction.
/// Is great to use in maps, or when 'following' some kind of line.
/// Use an Option<Directon> if there might be a lack of a Directon!
/// [`turn_right`]: enum.Direction.html#method.turn_right
/// [`turn_left`]: enum.Direction.html#method.turn_left
/// [`init`]: enum.Direction.html#variant.Init
/// [`right`]: enum.Direction.html#variant.Right
/// [`left`]: enum.Direction.html#variant.Left
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
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
            Direction::Right => Direction::Left,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

/// A position. Great to use in maps or graphs.
/// # Examples
/// ```
/// extern crate libaoc;
/// use libaoc::movement::{Position, ManhattenDst};
/// use libaoc::absolute::Absolute;
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
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
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
        Position { x, y }
    }

    /// Changes the position with `steps` based on the direction.
    /// If the direction is facing down, `y` is incremented, if the direction if facing up, `y` is decremented.
    /// # Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::movement::{Position, Direction};
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
    #[inline]
    pub fn change(&mut self, direction: &Direction, steps: N) {
        match *direction {
            Direction::Up => self.y -= steps,
            Direction::Down => self.y += steps,
            Direction::Right => self.x += steps,
            Direction::Left => self.x -= steps,
        }
    }

    /// Same as [`change`], but now increments `y` when facing upwards, and decrements `y` when facing downwards.
    /// # Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::movement::{Direction, Position};
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
        match *direction {
            Direction::Up => self.y += steps,
            Direction::Down => self.y -= steps,
            Direction::Right => self.x += steps,
            Direction::Left => self.x -= steps,
        }
    }

    /// Check whether self and `other` are adjecent. That is, if the absolute x value and the absolute y value after subtracting `self` from `other`
    /// is either (1, 0), (0, 1) or (1, 1).
    /// # Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::movement::Position;
    /// use libaoc::absolute::Absolute;
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
    pub fn is_adjecent(&self, other: &Position<N>) -> bool
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
    /// # Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::movement::Position;
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

    /// Copies x and y into a tuple
    pub fn cpy_into_tuple(&self) -> (N, N)
    where
        N: Copy,
    {
        (*self).into()
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

/// Returns the manhatten distance of any Position with type N.
/// A position is either a tuple, or the struct [Position](struct.Position.html).
/// the manhatten distance is the sum of the absolute values of a coordinate.
///
/// # Examples
/// ```
/// extern crate libaoc;
/// use libaoc::movement::{ManhattenDst, Position};
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
    /// Returns the `manhattendistance` of self.
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
