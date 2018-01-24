-   Add support for the nightly TryFrom trait, this would work really nice with TryConvert.
    This should be done with a #[cfg()], so it only gets implemented if the user uses the nightly TryFrom trait.
    see https://doc.rust-lang.org/std/convert/trait.TryFrom.html .

-   Better documentation and examples on the current lib.

-   Add proper tests and benchmarks.

-   Think about adding an .enumerate() call on the convert_func!() macro,
    so we know whether the entire Iterator is consumed.

    Now we only return an error if the Iterator has less items than the array,
    pherhaps an error if the Iterator still has items left after the array is build is also nice?.

    Or return the leftover as an Iterator?
