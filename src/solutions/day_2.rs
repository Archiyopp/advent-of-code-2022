pub fn day_2(input: String) {
    let first = problem_one(input.clone());
    let second = problem_two(input);
    println!("Day 2, first answer: {}, second answer: {}", first, second);
}

fn problem_one(input: String) -> u32 {
    input
        .lines()
        .map(|s| {
            let [other, mine]: [Hand; 2] = s
                .split_whitespace()
                .map(|x| Hand::from(x.to_string()))
                .collect::<Vec<Hand>>()
                .try_into()
                .expect("Invalid input");

            let outcome = mine.get_outcome(&other);
            outcome.get_points() + mine.get_points()
        })
        .sum()
}

fn problem_two(input: String) -> u32 {
    input
        .lines()
        .map(|s| {
            let [other, outcome]: [String; 2] = s
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
                .try_into()
                .expect("Invalid input");

            let other = Hand::from(other);
            let outcome = Outcome::from(outcome);

            outcome.get_points() + other.get_points_from_outcome(&outcome)
        })
        .sum()
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Hand {
    fn get_outcome(&self, other: &Hand) -> Outcome {
        match (self, other) {
            (Hand::Rock, Hand::Scissors) => Outcome::Win,
            (Hand::Rock, Hand::Paper) => Outcome::Lose,
            (Hand::Paper, Hand::Rock) => Outcome::Win,
            (Hand::Paper, Hand::Scissors) => Outcome::Lose,
            (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (Hand::Scissors, Hand::Rock) => Outcome::Lose,
            _ => Outcome::Draw,
        }
    }
    fn get_points_from_outcome(&self, outcome: &Outcome) -> u32 {
        match (self, outcome) {
            (Hand::Rock, Outcome::Win) => Hand::Paper.get_points(),
            (Hand::Rock, Outcome::Lose) => Hand::Scissors.get_points(),
            (Hand::Paper, Outcome::Win) => Hand::Scissors.get_points(),
            (Hand::Paper, Outcome::Lose) => Hand::Rock.get_points(),
            (Hand::Scissors, Outcome::Win) => Hand::Rock.get_points(),
            (Hand::Scissors, Outcome::Lose) => Hand::Paper.get_points(),
            (other, _) => other.get_points(),
        }
    }
}

impl Points for Hand {
    fn get_points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

impl Points for Outcome {
    fn get_points(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

impl From<String> for Hand {
    fn from(s: String) -> Self {
        match s.as_str() {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }
}

impl From<String> for Outcome {
    fn from(s: String) -> Self {
        match s.as_str() {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        }
    }
}

trait Points {
    fn get_points(&self) -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::read_file;

    #[test]
    fn problem_one_test() {
        assert_eq!(problem_one(read_file("examples", "second")), 15);
    }

    #[test]
    fn problem_two_test() {
        assert_eq!(problem_two(read_file("examples", "second")), 12);
    }
}
