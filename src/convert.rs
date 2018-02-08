use ::std::str::FromStr;

/// This trait allows to convert a stream of `str`'s, into a stream or collection of type U.
/// Return an Error when the conversion fails, but is able to produce the next value that has no Error.
/// # Examples
/// ```
/// extern crate libaoc;
///
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
    /// The Errortype that gets returned on an error.
    type Error;

    /// The Iterator that gets returned from [try_convert_iter()](trait.TryConvert.html#tymethod.try_convert_iter)
    type Iterable: Iterator<Item = Result<U, Self::Error>>;

    /// On succes, returns a vector of all completed conversions. When an error occures, returns an error instead.
    fn try_convert(self) -> Result<Vec<U>, Self::Error>;

    /// Tries to convert a stream of T into a slice of U.
    /// On an error, returns how many items where converted.
    /// # Examples
    ///
    /// ```
    /// extern crate libaoc;
    ///
    /// use libaoc::convert::TryConvert;
    ///
    /// fn main() {
    ///     let s = "1, 2, 3, 4, 5,6";
    ///     let mut buff = [0i64; 6];
    ///
    ///     let succeded = s.split(", ").try_convert_into_slice(&mut buff);
    ///
    ///     assert_eq!([1, 2, 3, 4, 0, 0], buff);
    ///     assert_eq!(Err(4), succeded);
    ///
    ///     let s = "1, 2, 3, 4, 5, 6, 7, 8, 9";
    ///     let written = s.split(", ").try_convert_into_slice(&mut buff);
    ///     assert_eq!(Ok(6), written);
    ///
    ///     let written = s.split(", ").take(2).try_convert_into_slice(&mut buff);
    ///     assert_eq!(Ok(2), written);
    /// }
    /// ```
    fn try_convert_into_slice(self, slice: &mut [U]) -> Result<usize, usize>;

    /// Returns an iterator over the converted items. Returns an error if an item can not be converted. Continue's after the error.
    fn try_convert_iter(self) -> Self::Iterable;
}

impl<U, S, I> TryConvert<U, S, I> for I
where
    U: FromStr,
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    type Error = <U as FromStr>::Err;
    type Iterable = ::std::iter::Map<I, fn(S) -> Result<U, Self::Error>>;

    #[inline]
    fn try_convert(self) -> Result<Vec<U>, Self::Error> {
        self.try_convert_iter().collect()
    }

    #[inline]
    fn try_convert_into_slice(self, slice: &mut [U]) -> Result<usize, usize> {
        let mut number_of_writes = 0;
        for (dst, src) in slice.iter_mut().zip(self.try_convert_iter()) {
            if let Ok(converted) = src {
                *dst = converted;
                number_of_writes += 1;
            }
            else {
                return Err(number_of_writes);
            }
        }
        Ok(number_of_writes)
    }

    #[inline]
    fn try_convert_iter(self) -> Self::Iterable {
        self.map(|item| item.as_ref().parse())
    }
}


/// This trait allows to convert a stream with items of type T into a stream or collection with items of type U.
///
/// # Examples
/// ```
/// extern crate libaoc;
///
/// use libaoc::convert::Convert;
/// use libaoc::movement::Position;
///
/// fn main() {
///     let tuple1 = (0, 0);
///     let tuple2 = (1, 1);
///     let tuple3 = (2, 2);
///
///     let tuples = vec![tuple1, tuple2, tuple3];
///
///     let Positions: Vec<Position<usize>> = tuples.into_iter().convert_into_vec();
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
    /// The Iterator that gets returned from [convert_iter()](trait.Convert.html#tymethod.convert_iter)
    type Iterable: Iterator<Item = U>;

    /// Returns a vector of all completed conversions.
    fn convert_into_vec(self) -> Vec<U>;

    /// Converts the stream, and writes the items into `slice`. Returns how many elements where written to the slice.
    ///
    /// # Examples
    /// ```
    /// extern crate libaoc;
    ///
    /// use libaoc::convert::Convert;
    ///
    /// fn main() {
    ///     let chars = vec![97, 98, 99, 100, 101];
    ///     let mut slice: [char; 5] = ['-'; 5];
    ///
    ///     let written = chars.into_iter().convert_into_slice(&mut slice);
    ///
    ///     assert_eq!(['a', 'b', 'c', 'd', 'e'], slice);
    ///     assert_eq!(5, written);
    /// }
    /// ```
    fn convert_into_slice(self, slice: &mut [U]) -> usize;

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
    fn convert_into_vec(self) -> Vec<U> {
        self.convert_iter().collect()
    }

    #[inline]
    fn convert_into_slice(self, slice: &mut [U]) -> usize {
        slice
            .iter_mut()
            .zip(self.convert_iter())
            .map(|(dst, src)| *dst = src)
            .count()
    }

    #[inline]
    fn convert_iter(self) -> Self::Iterable {
        self.map(move |item| U::from(item))
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FillError {
    FillError,
}

impl ::std::error::Error for FillError {

    #[inline]
    fn description(&self) -> &str {
        match self {
            &FillError::FillError => "The array was partially filled, and therefore dropped."
        }
    }
}

impl ::std::fmt::Display for FillError {

    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// This macro makes it easy to convert an Iterator into an array.
/// The `type` of the array has to be specified when this macro is called.
///
/// The array that get's build uses mem::unitialized to prevent unnecessary allocation,
/// however if the Iterator has less items than the lenght of the array, this means there is still
/// unitialized memory. In this case, the macro will return an error, and drop the array that was build.
/// # Examples
/// ```
/// #[macro_use]
/// extern crate libaoc;
///
/// use libaoc::movement::Position;
/// use libaoc::convert::Convert;
///
/// #[derive(Debug, PartialEq)]
/// struct noncopy{item: i64}
///
/// impl From<i64> for noncopy {
///     fn from(num: i64) -> noncopy {
///         noncopy{item: num}
///     }
/// }
/// fn main() {
///
///     let ss = vec![1, 2, 3];
///     let arr = arraycollect!(ss.into_iter().convert_iter() => [noncopy; 2]);
///     assert_eq!(Ok([noncopy{item: 1}, noncopy{item: 2}]), arr);
/// }
/// ```
#[macro_export]
macro_rules! arraycollect {
    ($iter:expr => [$tgt:ty; $num:tt]) => (
        {
            use ::std::mem;

            struct PartialArray<T> {
                data: mem::ManuallyDrop<[T; $num]>,
                fill: usize,
            }

            impl <T>PartialArray<T> {
                #[inline]
                fn new() -> PartialArray<T> {
                    unsafe {
                        PartialArray {
                            data: mem::ManuallyDrop::new(mem::uninitialized()),
                            fill: 0,
                        }
                    }
                }

                #[inline]
                fn fill_array<I: Iterator<Item = T>>(mut self, iter: I) -> Result<[T; $num], $crate::convert::FillError>
                {
                    for (dst, src) in self.data.iter_mut().zip(iter) {
                        unsafe {
                            ::std::ptr::write(dst, src);
                        }
                        self.fill += 1;
                    }

                    //if the number of items filled is not equal to the number of items that should have been written,
                    //return an error.
                    if self.fill != $num {
                        Err($crate::convert::FillError::FillError)
                    }
                    else {
                        Ok(self.finish())
                    }
                }
                #[inline]
                fn finish(mut self) -> [T; $num] {
                    unsafe {
                        let rd = ::std::ptr::read(&mut self.data);
                        let ret = mem::ManuallyDrop::into_inner(rd);
                        mem::forget(self);
                        ret
                    }
                }
            }

            impl <T>Drop for PartialArray<T> {
                #[inline]
                fn drop(&mut self) {
                    unsafe {
                        ::std::ptr::drop_in_place::<[T]>(&mut self.data[0..self.fill]);
                    }
                }
            }

            //pass in $tgt as generic paremeter, fill_array takes an array with items of $tgt.
            let array: PartialArray<$tgt> = PartialArray::new();
            array.fill_array($iter)
        }
    )
}
