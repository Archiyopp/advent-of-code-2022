pub fn day_1(input: String) -> [i32; 3] {
    let mut vector: Vec<i32> = input
        .trim()
        .split("\r\n\r\n")
        .map(|s| s.lines().flat_map(str::parse::<i32>).sum())
        .collect();
    vector.sort();
    [
        vector.pop().unwrap_or_default(), // max value
        vector.pop().unwrap_or_default(),
        vector.pop().unwrap_or_default(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_file;

    #[test]
    fn test_day_1() {
        assert_eq!(day_1(read_file("examples", "first")), [24000, 11000, 10000]);
    }
}
