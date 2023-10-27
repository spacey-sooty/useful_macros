#[macro_export]
macro_rules! map {
    ( $( $x:expr, $y:expr ),* ) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                let mut count = 0;
                for i in $x {
                    temp_map.insert(i, $y[count]);
                    count += 1;
                }
            )*
            temp_map
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::map;
    use std::collections::HashMap;
    #[test]
    fn test_return_type() {
        let _: HashMap<u8, &str> = map!([1, 2], ["Hello", "World"]);
    }

    #[test]
    fn test_values() {
        let map = map!([1, 2], ["Hello", "World"]);
        assert_eq!(map.get(&1).unwrap(), &"Hello");
        assert_eq!(map.get(&2).unwrap(), &"World");
    }

    #[test]
    fn test_mutation() {
        let mut map = map!([1, 2], ["Hello", "World"]);
        assert_eq!(map.get(&1).unwrap(), &"Hello");
        assert_eq!(map.get(&2).unwrap(), &"World");

        map.insert(3, "Foo");
        assert_eq!(map.get(&3).unwrap(), &"Foo");

        map.insert(2, "Bar");
        assert_eq!(map.get(&2).unwrap(), &"Bar");

        map.clear();
        assert_eq!(map.is_empty(), true);
    }
}
