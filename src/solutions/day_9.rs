use std::collections::HashSet;

use advent_of_code_2022::read_file;

pub fn day_9() {
    let input = read_file("inputs", "9");
    println!(
        "Day 9 solution 1: {}, solution 2: {}",
        solution(&input, 2),
        solution(&input, 10)
    );
}

fn solution(input: &str, length: usize) -> usize {
    let mut map: HashSet<(i32, i32)> = HashSet::new();
    let mut init_vector: Vec<(i32, i32)> = Vec::with_capacity(length);
    for _ in 0..length {
        init_vector.push((0, 0));
    }
    input
        .lines()
        .flat_map(|l| l.split_once(' '))
        .map(|(a, b)| (Direction::from(a), b.parse().unwrap_or(0)))
        .fold(init_vector, |mut coords, (d, n)| {
            for _ in 0..n {
                if let Some(last) = coords.last_mut() {
                    *last = d.move_coord(*last);
                }
                for (i, head) in coords.clone().iter().enumerate().rev() {
                    // prevent overflow
                    if i == 0 {
                        break;
                    }
                    match coords.get_mut(i - 1) {
                        Some(tail) => {
                            *tail = move_tail(*tail, *head);
                            if i == 1 {
                                map.insert(*tail);
                            };
                        }
                        None => break,
                    };
                }
            }
            coords
        });
    map.len()
}

fn move_tail(tail: (i32, i32), head: (i32, i32)) -> (i32, i32) {
    let (x, y) = tail;
    let (x2, y2) = head;
    if x >= x2 + 2 {
        (x2 + 1, y2)
    } else if x <= x2 - 2 {
        (x2 - 1, y2)
    } else if y >= y2 + 2 {
        (x2, y2 + 1)
    } else if y <= y2 - 2 {
        (x2, y2 - 1)
    } else {
        tail
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Direction {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            u => panic!("Invalid input {u}"),
        }
    }
}

impl Direction {
    fn move_coord(&self, coords: (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (coords.0, coords.1 + 1),
            Direction::Down => (coords.0, coords.1 - 1),
            Direction::Right => (coords.0 + 1, coords.1),
            Direction::Left => (coords.0 - 1, coords.1),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn problem_one() {
        assert_eq!(solution(&read_file("examples", "9"), 2), 13);
    }

    #[test]
    fn problem_two() {
        assert_eq!(solution(&read_file("examples", "9"), 10), 1);
        assert_eq!(solution(&read_file("examples", "9_2"), 10), 36);
    }
}
