use std::fs;

fn day1_1() -> i32 {
    let contents = fs::read_to_string("./input").unwrap();
    let mut xs: Vec<i32> = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|e| e.split("\n").collect::<Vec<&str>>())
        .map(|e| e.into_iter().map(|f| f.parse::<i32>().unwrap_or(0)))
        .map(|e| e.sum())
        .collect();
    xs.sort();

    *xs.last().unwrap()
}

fn day1_2() -> i32 {
    let contents = fs::read_to_string("./input").unwrap();
    let mut xs: Vec<i32> = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|e| e.split("\n").collect::<Vec<&str>>())
        .map(|e| e.into_iter().map(|f| f.parse::<i32>().unwrap_or(0)))
        .map(|e| e.sum())
        .collect();
    xs.sort();

    xs[(xs.len() - 3)..xs.len()].into_iter().sum()
}

fn main() {
    day1_1();
    day1_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_1() {
        let result = day1_1();
        assert_eq!(result, 72718);
    }

    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     b.iter(|| day1_1());
    // }

    #[test]
    fn test_day1_2() {
        let result = day1_2();
        assert_eq!(result, 213089);
    }
}
