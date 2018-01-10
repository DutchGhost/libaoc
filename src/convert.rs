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
pub trait TryConvert<U, S, I>
where
    U: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    type Error;
    type Iterable : Iterator<Item = Result<U, Self::Error>>;
    type UnsafeIterable : Iterator <Item = U>;

    /// On succes, returns a vector of all completed conversions. When an error occures, returns an error instead.
    fn try_convert(self) -> Result<Vec<U>, Self::Error>;

    /// Tries to convert a stream of T into a slice of U.
    /// On an error, returns how many items where converted.
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::convert::TryConvert;
    /// fn main() {
    ///     let s = "1, 2, 3, 4, 5,6";
    ///     let mut buff = [0i64; 6];
    ///     
    ///     let succeded = s.split(", ").try_convert_into_slice(&mut buff);
    /// 
    ///     assert_eq!([1, 2, 3, 4, 0, 0], buff);
    ///     assert_eq!(Err(4), succeded);
    /// }
    /// ```
    fn try_convert_into_slice(self, slice: &mut [U]) -> Result<usize, usize>;

    /// On succes, returns a vector of all completed conversions.
    /// #Panic
    /// Panics when an error occures.
    unsafe fn unsafe_convert(self) -> Vec<U>
    where
        Self::Error: ::std::fmt::Debug;
    
    /// Returns an iterator over the converted items. Returns an error if an item can not be converted. Continue's after the error.
    fn try_convert_iter(self) -> Self::Iterable;

    /// Returns an iterator over the converted items.
    /// #Panic
    /// Panics when an error occures.
    unsafe fn unsafe_convert_iter(self) -> Self::UnsafeIterable
    where
        Self::Error: fmt::Debug;
}

impl<U, S, I> TryConvert<U, S, I> for I
where
    U: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{   
    type Error = <U as FromStr>::Err;
    type Iterable = ::std::iter::Map<I, fn(S) -> Result<U, Self::Error>>;
    type UnsafeIterable = ::std::iter::Map<I, fn(S) -> U>;

    #[inline]
    fn try_convert(self) -> Result<Vec<U>, Self::Error> {
        self.try_convert_iter().collect()
    }

    #[inline]
    fn try_convert_into_slice(self, slice: &mut [U]) -> Result<usize, usize> {
        for ((idx, dst), src) in slice.iter_mut().enumerate().zip(self.try_convert_iter()) {
            if let Ok(converted) = src {
                *dst = converted;
            }
            else {
                return Err(idx);
            }
        }
        Ok(slice.len())
    }

    #[inline]
    unsafe fn unsafe_convert(self) -> Vec<U>
    where
        Self::Error: fmt::Debug,
    {
        self.unsafe_convert_iter().collect()
    }

    #[inline]
    fn try_convert_iter(self) -> Self::Iterable {
        self.map(|item| item.as_ref().parse())
    }

    #[inline]
    unsafe fn unsafe_convert_iter(self) -> Self::UnsafeIterable
    where
        Self::Error: fmt::Debug
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
pub trait Convert<T, U, I,>
where
    U: From<T>,
    I: Iterator<Item = T>,
{
    type Iterable : Iterator<Item = U>;
    
    /// Returns a vector of all completed conversions.
    fn convert(self) -> Vec<U>;

    /// Converts the stream, and put's the items into `slice`.
    /// #Examples
    /// ```
    /// extern crate libaoc;
    /// use libaoc::convert::Convert;
    /// fn main() {
    ///     let chars = vec![97, 98, 99, 100, 101];
    ///     let mut slice: [char; 5] = ['-'; 5];
    /// 
    ///     chars.into_iter().convert_into_slice(&mut slice);
    ///     
    ///     assert_eq!(['a', 'b', 'c', 'd', 'e'], slice);
    /// }
    /// ```
    fn convert_into_slice(self, slice: &mut [U]);
    
    /// Returns an iterator that performs the conversions.
    fn convert_iter(self) -> Self::Iterable;
}

impl<T, U, I> Convert<T, U, I> for I
where
    U: From<T>,
    I: Iterator<Item = T>,
{
    type Iterable = ::std::iter::Map<I, fn(T) -> U>;

    #[inline]
    fn convert(self) -> Vec<U> {
        self.convert_iter().collect()
    }
    
    #[inline]
    fn convert_into_slice(self, slice: &mut [U]) {
        slice.iter_mut().zip(self.convert_iter()).for_each(|(dst, src)| *dst = src)
    }

    #[inline]
    fn convert_iter(self) -> Self::Iterable {
        self.map(move |item| U::from(item))
    }
}