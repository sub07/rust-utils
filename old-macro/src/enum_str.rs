pub fn get_snake_case_from_pascal_case(input: &str) -> String {
    let mut indexes = input
        .chars()
        .enumerate()
        .filter_map(|(index, char)| {
            if char.is_ascii_uppercase() || char.is_ascii_digit() {
                Some(index)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    indexes.push(input.len());

    let word_splits = indexes.windows(2).collect::<Vec<_>>();
    let mut words = word_splits
        .iter()
        .map(|slice| input[slice[0]..slice[1]].to_ascii_lowercase())
        .fold(String::new(), |acc, word| acc + &word + "_");
    words.pop();
    words
}
