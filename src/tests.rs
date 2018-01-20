use convert::Convert;
pub mod test_arraycollect {
    
    #[macro_use]
    use super::*;
    
    #[derive(Debug, PartialEq)]
    struct NonCopy{item: i64}
    impl From<i64> for NonCopy {
        fn from(num: i64) -> NonCopy {
            NonCopy{item: num}
        }
    }

    #[test]
    fn test_array_collect() {
        let mut range = (0..3);
        let result = arraycollect!((&mut range).convert_iter() => [NonCopy; 2]);
        
        assert_eq!(result, Ok([NonCopy{item: 0}, NonCopy{item: 1}]));
        assert_eq!(range.next(), Some(2));

        let s = String::from("Hello world!");
        let result = arraycollect!(s.chars() => [char; 3]);
        assert_eq!(result, Ok(['H', 'e', 'l']));
    }
}