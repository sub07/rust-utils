#[easy_ext::ext(OptionUtilsExt)]
pub impl<T> Option<T> {
    fn and_then_zip<F, R>(self, f: F) -> Option<(T, R)>
    where
        F: FnOnce(&T) -> Option<R>,
    {
        self.and_then(|value| f(&value).map(|r| (value, r)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zip_with_test() {
        let some_value = Some(5);

        let add_if_even = |x: &i32| {
            if *x % 2 == 0 {
                Some(x + 10)
            } else {
                None
            }
        };

        let res = some_value.and_then_zip(|value| add_if_even(value));
        assert_eq!(None, res);

        let res = Some(4).and_then_zip(|value| add_if_even(value));
        assert_eq!(Some((4, 14)), res);
    }
}
