pub fn day_1(input: String) -> [i32; 3] {
    input
        .lines()
        .map(|x| x.parse::<i32>().unwrap_or_default())
        // sum the values separated by 0 anc collect them in a vector
        .fold(vec![0], |mut acc, x| {
            if x == 0 {
                acc.push(0);
            } else {
                let sum = acc.pop().expect("vector to have a length > 0") + x;
                acc.push(sum);
            }
            acc
        })
        // get the three biggest numbers in the vector
        .into_iter()
        .fold([0; 3], |acc, x| {
            if x > acc[0] {
                [x, acc[0], acc[1]]
            } else if x > acc[1] {
                [acc[0], x, acc[1]]
            } else if x > acc[2] {
                [acc[0], acc[1], x]
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use advent_of_code_2022::read_file;

    use super::*;

    #[test]
    fn test_day_1() {
        assert_eq!(day_1(read_file("examples", "first")), [24000, 11000, 10000]);
    }
}
