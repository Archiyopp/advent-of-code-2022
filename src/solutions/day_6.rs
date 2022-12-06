use advent_of_code_2022::read_file;

pub fn day_6() {
    let input = read_file("inputs", "6");
    println!(
        "Day 6, first answer: {}, second answer: {}",
        solution(&input, 4),
        solution(&input, 14)
    );
}

fn solution(input: &str, length: usize) -> usize {
    let mut marker = String::new();
    for (i, c) in input.chars().enumerate() {
        if let Some(end) = marker.find(c) {
            marker.drain(..end + 1);
        }
        marker.push(c);

        if marker.len() == length {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_one_test() {
        assert_eq!(solution(&read_file("examples", "6"), 4), 7);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    }

    #[test]
    fn problem_two_test() {
        assert_eq!(solution(&read_file("examples", "6"), 14), 19);
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    }
}
