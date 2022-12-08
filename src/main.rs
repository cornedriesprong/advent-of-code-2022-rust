use regex::Regex;
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
        .lines()
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
        .lines()
        .map(|e| e.split_once(" ").unwrap())
        .map(|(move1, outcome)| rock_paper_scissors_outcome(move1, outcome) + WIN_MAP[outcome])
        .sum();
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn day3_1() -> i32 {
    let str = fs::read_to_string("./input3").unwrap();
    return str
        .lines()
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
    return fs::read_to_string("./input3")
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|ch| {
            for c in ch[0].chars() {
                if ch[1].contains(c) && ch[2].contains(c) {
                    return (ALPHABET.chars().position(|x| x == c).unwrap() + 1) as i32;
                }
            }
            return 0;
        })
        .sum();
}

fn fully_contained(lhs: (i32, i32), rhs: (i32, i32)) -> bool {
    lhs.0 >= rhs.0 && lhs.1 <= rhs.1 || rhs.0 >= lhs.0 && rhs.1 <= lhs.1
}

fn day4_1() -> i32 {
    return fs::read_to_string("./input4")
        .unwrap()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            let lhs_min = a.split_once("-").unwrap().0.parse().unwrap();
            let lhs_max = a.split_once("-").unwrap().1.parse().unwrap();
            let rhs_min = b.split_once("-").unwrap().0.parse().unwrap();
            let rhs_max = b.split_once("-").unwrap().1.parse().unwrap();
            fully_contained((lhs_min, lhs_max), (rhs_min, rhs_max))
        })
        .filter(|b| *b)
        .collect::<Vec<bool>>()
        .len() as i32;
}

fn overlaps(lhs: (i32, i32), rhs: (i32, i32)) -> bool {
    !(lhs.1 < rhs.0 || lhs.0 > rhs.1)
}

fn day4_2() -> i32 {
    return fs::read_to_string("./input4")
        .unwrap()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            let lhs_min = a.split_once("-").unwrap().0.parse().unwrap();
            let lhs_max = a.split_once("-").unwrap().1.parse().unwrap();
            let rhs_min = b.split_once("-").unwrap().0.parse().unwrap();
            let rhs_max = b.split_once("-").unwrap().1.parse().unwrap();
            overlaps((lhs_min, lhs_max), (rhs_min, rhs_max))
        })
        .filter(|b| *b)
        .collect::<Vec<bool>>()
        .len() as i32;
}

fn get_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    input.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if !c.is_whitespace() {
                    stacks[i].push(c);
                };
            })
    });
    stacks
}

fn get_moves(input: &str) -> Vec<Vec<i32>> {
    return input
        .lines()
        .map(|line| {
            let re = Regex::new(r"\d+").unwrap();
            return re
                .find_iter(line)
                .map(|e| e.as_str().parse::<i32>().unwrap())
                .collect();
        })
        .collect();
}

fn move_stacks(stacks: &mut Vec<Vec<char>>, moves: Vec<Vec<i32>>, crane_model: i32) {
    for mv in moves {
        let amount = mv[0] as usize;
        let from = (mv[1] - 1) as usize;
        let to = (mv[2] - 1) as usize;
        if crane_model == 9000 {
            for _ in 0..amount {
                let crt = stacks[from].pop().unwrap();
                stacks[to].push(crt);
            }
        } else if crane_model == 9001 {
            let split_index = stacks[from].len() - amount;
            let crt = stacks[from].split_off(split_index);
            for e in crt {
                stacks[to].push(e);
            }
        }
    }
}

fn day5_1() -> String {
    let text = fs::read_to_string("./input5").unwrap();
    let (stack_str, moves_str) = text.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(stack_str);
    let moves = get_moves(moves_str);
    move_stacks(&mut stacks, moves, 9000);

    return stacks
        .iter()
        .map(|s| *s.last().unwrap())
        .collect::<String>();
}

fn day5_2() -> String {
    let text = fs::read_to_string("./input5").unwrap();
    let (stack_str, moves_str) = text.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(stack_str);
    let moves = get_moves(moves_str);
    move_stacks(&mut stacks, moves, 9001);

    return stacks
        .iter()
        .map(|s| *s.last().unwrap())
        .collect::<String>();
}

fn find_marker(signal: Vec<char>, marker_count: usize) -> usize {
    let offset = marker_count - 1;
    for i in 0..(signal.len() - offset) {
        let mut set: HashSet<char> = HashSet::new();
        for j in 0..marker_count {
            set.insert(signal[i + j]);
        }
        if set.len() == marker_count {
            return i + marker_count;
        }
    }
    return 0;
}

fn day6_1() -> i32 {
    let signal: Vec<char> = fs::read_to_string("./input6").unwrap().chars().collect();
    find_marker(signal, 4) as i32
}

fn day6_2() -> i32 {
    let signal: Vec<char> = fs::read_to_string("./input6").unwrap().chars().collect();
    find_marker(signal, 14) as i32
}

fn main() {
    day1_1();
    day1_2();
    day2_1();
    day2_2();
    day3_1();
    day3_2();
    day3_2();
    day4_1();
    day4_2();
    day5_1();
    day5_2();
    day6_1();
    println!("{}", day6_2());
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
        assert_eq!(result, 2633);
    }

    #[test]
    fn test_day4_1() {
        let result = day4_1();
        assert_eq!(result, 305);
    }

    #[test]
    fn test_day4_2() {
        let result = day4_2();
        assert_eq!(result, 811);
    }

    #[test]
    fn test_day5_1() {
        let result = day5_1();
        assert_eq!(result, "NTWZZWHFV");
    }

    #[test]
    fn test_day5_2() {
        let result = day5_2();
        assert_eq!(result, "BRZGFVBTJ");
    }

    fn test_day6_1() {
        let result = day6_1();
        assert_eq!(result, 1779);
    }

    fn test_day6_2() {
        let result = day6_2();
        assert_eq!(result, 0);
    }
}
