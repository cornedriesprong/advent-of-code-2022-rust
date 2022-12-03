use std::collections::{HashMap, HashSet};
use std::fs;

fn day1_1() -> i32 {
    let contents = fs::read_to_string("./input").unwrap();
    return contents
        .split("\n\n")
        .map(|s| s.lines().map(|f| f.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap();
}

fn day1_2() -> i32 {
    let contents = fs::read_to_string("./input").unwrap();
    let mut xs: Vec<i32> = contents
        .split("\n\n")
        .map(|s| s.lines().map(|f| f.parse::<i32>().unwrap()).sum())
        .collect();
    xs.sort();

    xs[(xs.len() - 3)..xs.len()].into_iter().sum()
}

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref SCORE_MAP: HashMap<&'static str, i32> =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    static ref WIN_MAP: HashMap<&'static str, i32> = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);
}

const RPC_TABLE: [(i32, i32, i32); 9] = [
    (1, 1, 3),
    (1, 2, 6),
    (1, 3, 0),
    (2, 1, 0),
    (2, 2, 3),
    (2, 3, 6),
    (3, 1, 6),
    (3, 2, 0),
    (3, 3, 3),
];

fn rock_paper_scissors(move1: &str, move2: &str) -> i32 {
    for e in RPC_TABLE {
        if e.0 == SCORE_MAP[move1] && e.1 == SCORE_MAP[move2] {
            return e.2;
        }
    }
    return 0;
}

fn day2_1() -> i32 {
    return fs::read_to_string("./input2")
        .unwrap()
        .split("\n")
        .map(|e| e.split_once(" ").unwrap())
        .map(|(move1, move2)| rock_paper_scissors(move1, move2) + SCORE_MAP[move2])
        .sum();
}

fn rock_paper_scissors_outcome(move1: &str, outcome: &str) -> i32 {
    // get required response (rock, paper or scissors) for a desired outcome score
    for e in RPC_TABLE {
        if e.0 == SCORE_MAP[move1] && e.2 == WIN_MAP[outcome] {
            return e.1;
        }
    }
    return 0;
}

fn day2_2() -> i32 {
    return fs::read_to_string("./input2")
        .unwrap()
        .split("\n")
        .map(|e| e.split_once(" ").unwrap())
        .map(|(move1, outcome)| rock_paper_scissors_outcome(move1, outcome) + WIN_MAP[outcome])
        .sum();
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn day3_1() -> i32 {
    let str = fs::read_to_string("./input3").unwrap();
    return str
        .split("\n")
        .map(|e| e.split_at(e.chars().count() / 2))
        .map(|(a, b)| {
            for c1 in a.chars() {
                for c2 in b.chars() {
                    if c1 == c2 {
                        return (ALPHABET.chars().position(|x| x == c1).unwrap() + 1) as i32;
                    }
                }
            }
            return 0;
        })
        .sum();
}

fn day3_2() -> i32 {
    let items = fs::read_to_string("./input3").unwrap().split("\n");
    return 0;
}

fn main() {
    day1_1();
    day1_2();
    day2_1();
    day2_2();
    day3_1();
    day3_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_1() {
        let result = day1_1();
        assert_eq!(result, 72718);
    }

    #[test]
    fn test_day1_2() {
        let result = day1_2();
        assert_eq!(result, 213089);
    }

    #[test]
    fn test_day2_1() {
        let result = day2_1();
        assert_eq!(result, 13268);
    }

    #[test]
    fn test_day2_2() {
        let result = day2_2();
        assert_eq!(result, 15508);
    }

    #[test]
    fn test_day3_1() {
        let result = day3_1();
        assert_eq!(result, 7785);
    }

    #[test]
    fn test_day3_2() {
        let result = day3_2();
        assert_eq!(result, 0);
    }
}
