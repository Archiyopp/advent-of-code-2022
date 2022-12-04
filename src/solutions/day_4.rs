use advent_of_code_2022::read_file;

pub fn day_4() {
    let input = read_file("inputs", "fourth");
    println!(
        "Day 4: First answer: {}, Second answer: {}",
        problem_one(input.clone()),
        problem_two(input)
    );
}

fn problem_one(input: String) -> usize {
    input
        .lines()
        .flat_map(parse_line)
        .filter(|[a, b]| check_ranges(a, b) || check_ranges(b, a))
        .count()
}

fn problem_two(input: String) -> usize {
    input
        .lines()
        .flat_map(parse_line)
        .filter(|[a, b]| check_overlap(a, b))
        .count()
}

fn parse_line(line: &str) -> Option<[[u32; 2]; 2]> {
    line.split_once(',').map(|(a, b)| {
        [a, b].map(|s| {
            s.split_once('-')
                .map(|(a, b)| [a, b].map(|n| n.parse::<u32>().expect("Not a number")))
                .expect("Invalid input")
        })
    })
}

fn check_ranges(a: &[u32; 2], b: &[u32; 2]) -> bool {
    a[0] >= b[0] && a[1] <= b[1]
}

fn check_overlap(a: &[u32; 2], b: &[u32; 2]) -> bool {
    a[1] >= b[0] && a[0] <= b[1]
}

#[cfg(test)]
mod day_4_tests {
    use super::*;

    #[test]
    fn test_problem_1() {
        assert_eq!(problem_one(read_file("examples", "fourth")), 2);
    }

    #[test]
    fn test_problem_2() {
        assert_eq!(problem_two(read_file("examples", "fourth")), 4);
    }
}
