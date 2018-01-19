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
    /// The Iterator that gets returned from [convert_iter()](trait.Convert.html#tymethod.convert_iter)
    type Iterable: Iterator<Item = U>;
    
    /// Returns a vector of all completed conversions.
    fn convert(self) -> Vec<U>;

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
    fn convert(self) -> Vec<U> {
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

/// This macro allows to create a function that takes any Iterator<Item = T>, and converts it into
/// an array with elements of type U. See the examples.
/// 
/// # Examples
/// ```
/// #[macro_use]
/// extern crate libaoc;
/// 
/// use libaoc::movement::Position;
///
/// fn main() {
///     let tup1 = (0, 0);
///     let tup2 = (1, 1);
///     let tup3 = (2, 2);
/// 
///     let tuples = vec![tup1, tup2, tup3];
/// 
///     let arr = convert!(tuples.into_iter() => [Position<i64>; 2]);
///     assert_eq!(Ok([Position::new(0, 0), Position::new(1, 1)]), arr);
/// }
/// ```
#[macro_export]
macro_rules! convert {
    ($iter:expr => [$tgt:ty; $num:tt]) => (
        {
            unsafe {
                let mut arr: [ $tgt ; $num ] = std::mem::uninitialized();
                let mut filled = 0;
                
                for (dst, src) in arr.iter_mut().zip($iter) {
                    ::std::ptr::write(dst, src.into());
                    filled += 1;
                }
                
                if filled != $num {

                    // Not all positions were filled. Drop the filled positions
                    // manually and forget the array to prevent the
                    // uninitailized memory from being dropped.
                    for i in 0..filled {
                        ::std::ptr::drop_in_place(&mut arr[i]);
                    }
                    ::std::mem::forget(arr);
                    
                    Err("Too few elements")
                } else {
                    Ok(arr)
                }
            }
        }
    )
}
/// This macro makes it easy to convert an Iterator into an array.
/// The `type` of the array has to be specified when this macro is called.
/// 
/// The function that get's build uses mem::unitialized to prevent unnecessary allocation,
/// however if the Iterator has less items than the lenght of the array, this means there is still
/// unitialized memory. In this case, the function will return an error, and drop the array that was build.
/// 
/// # Examples
/// ```
/// #[macro_use(convert_func)]
/// extern crate libaoc;
/// use libaoc::movement::Position;
/// use libaoc::convert::Convert;
/// fn main() {
///     convert_func!(ArrayConvert, into_array -> [Position<usize>; 3]);
///     convert_func!(extend ArrayConvert with _ArrayConvert, into_array4 -> [Position<usize>; 4]);
///     let tuples = vec![(1, 2), (3, 4), (5, 6)];
///     
///     let positions: [Position<usize>; 3] = [Position::new(1, 2), Position::new(3, 4), Position::new(5, 6)];
///     assert_eq!(positions, tuples.into_iter().into_array().unwrap());
/// 
///     let positionss: [Position<usize>; 4] = [Position::new(0usize, 0), Position::new(1, 1), Position::new(2, 2), Position::new(3, 3)];
///     let tups: Vec<(usize, usize)> = vec![(0, 0), (1, 1), (2, 2), (3, 3)];
///     assert_eq!(positionss, tups.into_iter().into_array4().unwrap());
/// }
/// ```

//@TODO:
//  Recursive definition,
//  Pherhaps return an error if the Iterator has more items than the array,
//  Or return the items left from the Iterator as an Iterator.
#[macro_export]
macro_rules! convert_func {
    ($traitname:ident, $funcname:ident -> [$tgt:ty; $num:tt]) => {
        pub trait $traitname
        {
            fn $funcname(self) -> Result<[$tgt; $num], &'static str>;
        }

        impl <T, I>$traitname for I
        where
            I: Convert<T, $tgt, I> + Iterator<Item = T>,
            $tgt: From<T>,
        {
            fn $funcname(self) -> Result<[$tgt; $num], &'static str> {
                unsafe {
                    let mut arr: [ $tgt ; $num ] = ::std::mem::uninitialized();
                    let mut filled = 0;

                    //fill the array with items
                    for (dst, src) in arr.iter_mut().zip(self.convert_iter()) {
                        ::std::ptr::write(dst, src);
                        filled += 1;
                    }
                    
                    //if something went wrong, clean up!
                    if filled != $num {

                        //drop the items
                        for i in 0..filled {
                            ::std::ptr::drop_in_place(&mut arr[i]);
                        }

                        //forget the array
                        ::std::mem::forget(arr);
                        Err("there was a problem converting.")
                    }
                    else {
                        Ok(arr)
                    }
                }
            }
        } 
    };
    (extend $traitname:ident with $newname:ident, $funcname:ident -> [$tgt:ty; $num:tt]) => {
        pub trait $newname<T, I>: $traitname + Convert<T, $tgt, I> + Sized
        where
            $tgt: From<T>,
            I: Iterator<Item = T>
        {
            fn $funcname(self) -> Result<[$tgt; $num], &'static str> {
                unsafe {
                    let mut arr: [ $tgt ; $num ] = ::std::mem::uninitialized();
                    let mut filled = 0;
                
                    for (dst, src) in arr.iter_mut().zip(self.convert_iter()) {
                        ::std::ptr::write(dst, src);
                        filled += 1;
                    }
                    
                    //if something went wrong, clean up!
                    if filled != $num {

                        //drop the items
                        for i in 0..filled {
                            ::std::ptr::drop_in_place(&mut arr[i]);
                        }

                        //forget the array
                        ::std::mem::forget(arr);
                        Err("there was a problem converting.")
                    }
                    else {
                        Ok(arr)
                    }
                }
            }
        }
        impl <T, I>$newname<T, I> for I
        where
            I: $traitname + Iterator<Item = T>,
            $tgt: From<T>
        {}
    }
}