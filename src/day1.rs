use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input_day1(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn d1p1_solve(input: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for n in input.as_slice().windows(2) {
        if n[0] < n[1] {
            sum += 1;
        }
    }

    sum
}

#[aoc(day1, part2)]
pub fn d1p2_solve(input: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for n in input.as_slice().windows(4) {
        let a = n[0] + n[1] + n[2];
        let b = n[1] + n[2] + n[3];

        if a < b {
            sum += 1;
        }
    }

    sum
}
