use std::collections::HashMap;

use advent_of_code_2022::read_file;

pub fn day_3() {
    println!(
        "Day 3, first answer: {}, second answer: {}",
        problem_one(read_file("inputs", "third")),
        problem_two(read_file("inputs", "third"))
    );
}

fn problem_one(input: String) -> u32 {
    let mut priorities = 0;
    for line in input.lines() {
        let half_len = line.split_at(line.len() / 2);
        let mut map = HashMap::new();
        for c in half_len.0.chars() {
            map.entry(c).or_insert(true);
        }
        for c in half_len.1.chars() {
            if map.contains_key(&c) {
                priorities += get_priority(c);
                break;
            }
        }
    }

    priorities
}

fn problem_two(input: String) -> u32 {
    let mut priorities = 0;
    let mut iter = 0;
    let mut map = HashMap::new();
    for line in input.lines() {
        if iter == 3 {
            iter = 0;
            map.clear();
        }
        iter += 1;
        for c in line.chars() {
            let count = map
                .entry(c)
                .and_modify(|c| {
                    if *c == iter - 1 {
                        *c += 1;
                    }
                })
                .or_insert_with(|| u32::from(iter == 1));
            if *count == 3 {
                priorities += get_priority(c);
                map.clear();
                break;
            }
        }
    }
    priorities
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
