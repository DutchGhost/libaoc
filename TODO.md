-   Add support for the nightly TryFrom trait, this would work really nice with TryConvert.
    This should be done with a #[cfg()], so it only gets implemented if the user uses the nightly TryFrom trait.
    see https://doc.rust-lang.org/std/convert/trait.TryFrom.html .

-   Better documentation and examples on the current lib.

-   Add proper tests and benchmarks.
