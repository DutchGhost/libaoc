pub mod test_arraycollect {
    use convert::Convert;
    #[derive(Debug, PartialEq)]
    struct NonCopy {
        item: i64,
    }
    impl From<i64> for NonCopy {
        fn from(num: i64) -> NonCopy {
            NonCopy { item: num }
        }
    }

    impl NonCopy {
        fn new(num: i64) -> NonCopy {
            NonCopy { item: num }
        }
    }

    #[test]
    fn test_array_collect() {
        let mut range = (0..3);
        let result = arraycollect!((&mut range).convert_iter() => [NonCopy; 2]);

        let cmp = [NonCopy::new(0), NonCopy::new(1)];

        assert_eq!(result, Ok(cmp));
        assert_eq!(range.next(), Some(2));

        let s = String::from("Hello world!");
        let result = arraycollect!(s.chars() => [char; 3]);
        assert_eq!(result, Ok(['H', 'e', 'l']));
    }

    #[test]
    fn test_mut_array_collect() {
        let mut v = vec![NonCopy::new(1), NonCopy::new(2), NonCopy::new(4)];

        {
            let mut arr = arraycollect!(v.iter_mut() => [&mut NonCopy; 3]).unwrap();

            //if we iterate over 'arr', we get a mutable reference, to a mutable reference, to a NonCopy. Double deref.
            for item in arr.iter_mut() {
                **item = NonCopy::new(0);
            }
            assert_eq!(
                [
                    &mut NonCopy::new(0),
                    &mut NonCopy::new(0),
                    &mut NonCopy::new(0)
                ],
                arr
            );
        }
        assert_eq!(vec![NonCopy::new(0), NonCopy::new(0), NonCopy::new(0)], v);
    }
}
