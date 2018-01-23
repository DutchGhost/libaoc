pub mod test_arraycollect {
    use convert::Convert;

    //This is a noncopy wrapper around an i64.
    //it is needed to show that arraycollect!() also works on noncopy types!
    #[derive(Debug, PartialEq)]
    struct NonCopy{ item: i64 }

    impl From<i64> for NonCopy {
        fn from(num: i64) -> NonCopy {
            NonCopy{item: num}
        }
    }

    impl NonCopy {
        fn new(n: i64) -> NonCopy {
            NonCopy {item: n}
        }
    }
    
    #[test]
    fn test_convert_and_array_collect() {
        let mut range = 0..3;
        let result = arraycollect!((&mut range).convert_iter() => [NonCopy; 2]);

        let cmp = [NonCopy::new(0), NonCopy::new(1)];
        assert_eq!(result, Ok(cmp));
        assert_eq!(range.next(), Some(2));

        let s = String::from("Hello world!");
        let result = arraycollect!(s.chars() => [char; 3]);
        assert_eq!(result, Ok(['H', 'e', 'l']));
    }
}
