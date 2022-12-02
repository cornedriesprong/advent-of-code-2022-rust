use std::collections::HashMap;
use std::fs;

fn day1_1() -> i32 {
    let contents = fs::read_to_string("./input").unwrap();
    let mut xs: Vec<i32> = contents
        .split("\n\n")
        .map(|e| e.split("\n"))
        .map(|e| e.map(|f| f.parse::<i32>().unwrap_or(0)))
        .map(|e| e.sum())
        .collect();
    xs.sort();

    *xs.last().unwrap()
}

fn day1_2() -> i32 {
    let contents = fs::read_to_string("./input").unwrap();
    let mut xs: Vec<i32> = contents
        .split("\n\n")
        .map(|e| e.split("\n"))
        .map(|e| e.map(|f| f.parse().unwrap_or(0)))
        .map(|e| e.sum())
        .collect();
    xs.sort();

    xs[(xs.len() - 3)..xs.len()].into_iter().sum()
}

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref SCORE_MAP: HashMap<&'static str, i32> =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
}

fn rock_paper_scissors(a: &str, b: &str) -> i32 {
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
    for e in RPC_TABLE {
        if e.0 == SCORE_MAP[a] && e.1 == SCORE_MAP[b] {
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
        .map(|e| rock_paper_scissors(e.0, e.1) + SCORE_MAP[e.1])
        .sum();
}

fn main() {
    day1_1();
    day1_2();
    day2_1();
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
}
