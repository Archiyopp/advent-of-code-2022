use advent_of_code_2022::read_file;

pub fn day_4() {
    println!("Day 4");
    let input = read_file("inputs", "fourth");
    println!(
        "Day 4: First answer: {}, Second answer: {}",
        problem_one(input.clone()),
        problem_two(input)
    );
}

fn problem_one(input: String) -> u32 {
    input
        .lines()
        .flat_map(parse_line)
        .map(|[a, b]| u32::from(check_ranges(a, b) || check_ranges(b, a)))
        .sum()
}

fn problem_two(input: String) -> u32 {
    input
        .lines()
        .flat_map(parse_line)
        .map(|[a, b]| u32::from(check_overlap(a, b)))
        .sum()
}

fn parse_line(line: &str) -> Option<[[u32; 2]; 2]> {
    line.split_once(',').map(|(a, b)| {
        [a, b].map(|s| {
            s.split_once('-')
                .map(|(a, b)| [a, b].map(|n| str::parse::<u32>(n).unwrap_or(0)))
                .expect("Invalid input")
        })
    })
}

fn check_ranges(a: [u32; 2], b: [u32; 2]) -> bool {
    a[0] >= b[0] && a[1] <= b[1]
}

fn check_overlap(a: [u32; 2], b: [u32; 2]) -> bool {
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
