#[macro_export]
macro_rules! hash_map_of {
    () => {
        std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {
        std::collections::HashMap::from([
            $(
                ($key, $value),
            )*
        ])
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn hmap_test() {
        #[derive(Debug, PartialEq, Hash, Eq)]
        struct A(i32);

        impl A {
            fn f(v: i32) -> A {
                A(v)
            }
        }

        let hm = hash_map_of! [
            8 => A::f(4),
            4 => A::f(8),
        ];

        assert_eq!(Some(&A(4)), hm.get(&8));
        assert_eq!(Some(&A(8)), hm.get(&4));

        let hm = hash_map_of! [
            8 => A::f(4),
        ];
        assert_eq!(Some(&A(4)), hm.get(&8));

        let hm: HashMap<(), ()> = hash_map_of![];

        assert_eq!(0, hm.len());

        let hm = hash_map_of! [
            "test1" => hash_map_of! [
                A::f(9) => vec![4, 5, 3],
                A::f(7) => vec![4, 5, 3],
            ],
            "test2" => hash_map_of! [
                A::f(6) => vec![3, 9, 6],
                A::f(5) => vec![3, 9, 6],
                A::f(1) => vec![3, 9, 6],
            ]
        ];

        assert_eq!(2, hm.len());
        assert_eq!(Some(2), hm.get(&"test1").map(HashMap::len))
    }
}
