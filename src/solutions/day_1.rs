pub fn day_1(input: String) -> [i32; 3] {
    let mut vector = input
        .trim()
        .split("\r\n\r\n")
        .map(|s| {
            s.lines()
                .map(|x| x.parse::<i32>().unwrap_or_default())
                .sum()
        })
        .collect::<Vec<i32>>();
    vector.sort();
    [
        vector.pop().unwrap_or_default(),
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
