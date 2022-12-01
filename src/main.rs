use std::fs;
use std::time::Instant;

fn day1_1() -> i32 {
    let contents = fs::read_to_string("./input").unwrap();
    let mut xs: Vec<i32> = contents
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|e| e.split("\n").collect::<Vec<&str>>())
        .map(|e| e.into_iter().map(|f| f.parse::<i32>().unwrap_or(0)))
        .map(|e| e.reduce(|a, b| a + b).unwrap_or(0))
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
        .map(|e| e.reduce(|a, b| a + b).unwrap_or(0))
        .collect();
    xs.sort();

    let top3_values = xs[(xs.len() - 3)..xs.len()].into_iter();
    let mut result = 0;
    for e in top3_values {
        result += e;
    }
    result
}

fn main() {
    let start = Instant::now();
    day1_1();
    let elapsed_time = start.elapsed().as_micros();
    println!("day1_1 executed in {} microseconds", elapsed_time);

    let start = Instant::now();
    day1_2();
    let elapsed_time = start.elapsed().as_micros();
    println!("day1_2 executed in {} microseconds", elapsed_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_1() {
        let start = Instant::now();
        let result = day1_1();
        let elapsed_time = start.elapsed().as_micros();
        println!("executed in {} microseconds", elapsed_time);
        assert_eq!(result, 72718);
    }

    // #[bench]
    // fn bench_add_two(b: &mut Bencher) {
    //     b.iter(|| day1_1());
    // }

    #[test]
    fn test_day1_2() {
        let start = Instant::now();
        let result = day1_2();
        let elapsed_time = start.elapsed().as_micros();
        println!("Executed in {} microseconds", elapsed_time);
        assert_eq!(result, 213089);
    }
}
