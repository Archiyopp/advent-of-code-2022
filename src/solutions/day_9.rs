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
    let mut map: HashSet<Coordinate> = HashSet::new();
    let mut init_vector: Vec<Coordinate> = Vec::with_capacity(length);
    for _ in 0..length {
        init_vector.push(Coordinate::new(0, 0));
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

                for i in (1..coords.len()).rev() {
                    let head = coords[i];
                    match coords.get_mut(i - 1) {
                        Some(tail) => {
                            *tail = move_tail(*tail, head);
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

fn move_tail(tail: Coordinate, head: Coordinate) -> Coordinate {
    if tail.x.abs_diff(head.x) > 1 || tail.y.abs_diff(head.y) > 1 {
        Coordinate::new(compare_pos(head.x, tail.x), compare_pos(head.y, tail.y))
    } else {
        tail
    }
}

fn compare_pos(lead: i32, follow: i32) -> i32 {
    follow + (lead - follow).signum()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
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
    fn move_coord(&self, coords: Coordinate) -> Coordinate {
        match self {
            Direction::Up => Coordinate::new(coords.x, coords.y + 1),
            Direction::Down => Coordinate::new(coords.x, coords.y - 1),
            Direction::Right => Coordinate::new(coords.x + 1, coords.y),
            Direction::Left => Coordinate::new(coords.x - 1, coords.y),
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
