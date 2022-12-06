use std::collections::HashMap;

use advent_of_code_2022::read_file;

pub fn day_5() {
    let input = read_file("inputs", "5");
    println!(
        "Day 5, first answer: {}, second answer: {}",
        problem_one(&input),
        problem_two(&input)
    );
}

fn problem_one(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").expect("Invalid input");
    let stacks = instructions.lines().map(parse_moves).fold(
        parse_init_crates(crates),
        |mut stacks, moves| {
            let mut crates = String::new();
            stacks
                .entry(moves[1] - 1)
                .and_modify(|s| crates = s.drain((s.len() - moves[0])..).collect());
            stacks
                .entry(moves[2] - 1)
                .and_modify(|to| to.extend(crates.chars().rev()));
            stacks
        },
    );

    get_solution(stacks)
}

fn problem_two(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").expect("Invalid input");
    let stacks = instructions.lines().map(parse_moves).fold(
        parse_init_crates(crates),
        |mut stacks, moves| {
            let mut crates = String::new();
            stacks
                .entry(moves[1] - 1)
                .and_modify(|s| crates = s.drain((s.len() - moves[0])..).collect());
            stacks
                .entry(moves[2] - 1)
                .and_modify(|to| to.push_str(&crates));
            stacks
        },
    );

    get_solution(stacks)
}

fn parse_init_crates(input: &str) -> HashMap<usize, String> {
    let (input, cols) = input.rsplit_once('\n').expect("No cols");
    let mut cols = HashMap::from_iter(parse_columns(cols));
    input
        .lines()
        .rev()
        .map(|l| l.chars().enumerate().filter(|c| c.1.is_alphabetic()))
        .for_each(|l| {
            l.for_each(|(i, c)| {
                cols.entry((i - 1) / 4).and_modify(|v| v.push(c));
            })
        });
    cols
}

fn parse_columns(input: &str) -> impl Iterator<Item = (usize, String)> + '_ {
    input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(u, _)| (u, String::new()))
}

fn parse_moves(isntruction: &str) -> [usize; 3] {
    isntruction
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .collect::<Vec<_>>()
        .try_into()
        .expect("Invalid movement")
}

fn get_solution(stacks: HashMap<usize, String>) -> String {
    let mut tmp_vec = stacks.into_iter().collect::<Vec<_>>();
    tmp_vec.sort_by_key(|v| v.0);
    tmp_vec
        .into_iter()
        .flat_map(|e| e.1.chars().last())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_one() {
        let input = read_file("examples", "5");
        assert_eq!(problem_one(&input), "CMZ");
    }

    #[test]
    fn test_problem_two() {
        let input = read_file("examples", "5");
        assert_eq!(problem_two(&input), "MCD");
    }
}
