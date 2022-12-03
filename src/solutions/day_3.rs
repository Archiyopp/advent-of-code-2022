use itertools::Itertools;
use std::collections::BTreeSet;

use advent_of_code_2022::read_file;

pub fn day_3() {
    println!(
        "Day 3, first answer: {}, second answer: {}",
        problem_one(read_file("inputs", "third")),
        problem_two(read_file("inputs", "third"))
    );
}

fn problem_one(input: String) -> u32 {
    input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| {
            let a = BTreeSet::from_iter(a.chars());
            let b = BTreeSet::from_iter(b.chars());
            a.intersection(&b)
                .next()
                .map(|c| get_priority(*c))
                .expect("No intersection")
        })
        .sum()
}

fn problem_two(input: String) -> u32 {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(a, b, c)| [a, b, c].map(|l| BTreeSet::from_iter(l.chars())))
        .map(|[a, b, c]| {
            a.intersection(&b)
                .find(|ch| c.contains(ch))
                .map(|c| get_priority(*c))
                .expect("No intersection")
        })
        .sum()
}

fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_file;

    #[test]
    fn problem_one_test() {
        assert_eq!(problem_one(read_file("examples", "third")), 157);
    }

    #[test]
    fn problem_two_test() {
        assert_eq!(problem_two(read_file("examples", "third")), 70);
    }
}
