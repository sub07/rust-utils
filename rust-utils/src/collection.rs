#[macro_export]
macro_rules! hash_map_of {
    ($($key:expr => $value:expr,)+) => {
        std::collections::HashMap::from([
            $(
                ($key, $value),
            )+
        ])
    };
}
