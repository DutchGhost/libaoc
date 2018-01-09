use ::std::str::FromStr;
use ::std::fmt;

/// Takes any Iterator, where the items implement AsRef<str>.
/// Returns a Vec<N>, where N implements FromStr.
/// Returns an Error if an error occured.
/// #Examples
/// ```
/// extern crate libaoc;
/// use libaoc::convert::TryConvert;
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
        <N as FromStr>::Err: ::std::fmt::Debug;
    
    /// Returns an iterator over the converted items. Returns an error if an item can not be converted. Continue's after the error.
    fn try_convert_iter(self) -> ::std::iter::Map<I, fn(S) -> Result<N, <N as FromStr>::Err>>;

    /// Returns an iterator over the converted items.
    /// #Panic
    /// Panics when an error occures.
    unsafe fn unsafe_convert_iter(self) -> ::std::iter::Map<I, fn(S) -> N>
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
    fn try_convert_iter(self) -> ::std::iter::Map<I, fn(S) -> Result<N, <N as FromStr>::Err>> {
        self.map(|item| item.as_ref().parse())
    }

    #[inline]
    unsafe fn unsafe_convert_iter(self) -> ::std::iter::Map<I, fn(S) -> N>
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
/// use libaoc::convert::Convert;
/// use libaoc::movement::Position;
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
    fn convert_iter(self) -> ::std::iter::Map<I, fn(T) -> U>;
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
    fn convert_iter(self) -> ::std::iter::Map<I, fn(T) -> U> {
        self.map(|item| U::from(item))
    }
}