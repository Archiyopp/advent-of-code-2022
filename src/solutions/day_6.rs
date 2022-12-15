use std::collections::HashSet;

use advent_of_code_2022::read_file;

pub fn day_6() {
    let input = read_file("inputs", "6");
    // let mut sum = 0;
    // let times = 100;
    // let start = std::time::Instant::now();
    // for _ in 0..times {
    //     sum += solution(&input, 14);
    // }
    // println!("Day 6, answer: {}, time: {:?}", sum, start.elapsed());
    // let mut sum = 0;
    // let start = std::time::Instant::now();
    // for _ in 0..times {
    //     sum += solution2(&input, 14);
    // }
    // println!("Day 6, sol2 answer: {}, time: {:?}", sum, start.elapsed());
    // let mut sum = 0;
    // let start = std::time::Instant::now();
    // for _ in 0..times {
    //     sum += solution3(&input, 14);
    // }
    // println!("Day 6, sol3 answer: {}, time: {:?}", sum, start.elapsed());
    // sol3 >> sol1 >> sol2 fastest to slowest
    println!(
        "Day 6, first answer: {}, second answer: {}, using sol2: {}, sol3: {}",
        solution(&input, 4),
        solution(&input, 14),
        solution2(&input, 14),
        solution3(&input, 14)
    );
}

fn solution(input: &str, length: usize) -> usize {
    let mut marker = String::new();
    for (i, c) in input.chars().enumerate() {
        if let Some(end) = marker.find(c) {
            marker.drain(..end + 1);
        }
        if marker.len() == length - 1 {
            return i + 1;
        }
        marker.push(c);
    }
    0
}

fn solution2(input: &str, length: usize) -> usize {
    input
        .as_bytes()
        .windows(length)
        .position(|w| w.iter().collect::<HashSet<_>>().len() == length)
        .map_or(0, |f| f + length)
}

fn solution3(input: &str, length: usize) -> usize {
    input
        .as_bytes()
        .windows(length)
        .position(move |set| {
            let mut data: u32 = 0;
            for &c in set {
                let prev = data;
                data |= 1 << (c - b'a');
                if prev == data {
                    return false;
                }
            }
            true
        })
        .map_or(0, |f| f + length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_one_test() {
        assert_eq!(solution(&read_file("examples", "6"), 4), 7);
        assert_eq!(solution2("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(solution3("nppdvjthqldpwncqszvftbrmjlhg", 4), 6)
    }

    #[test]
    fn problem_two_test() {
        assert_eq!(solution(&read_file("examples", "6"), 14), 19);
        assert_eq!(solution2(&read_file("examples", "6"), 14), 19);
        assert_eq!(solution3("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    }
}
