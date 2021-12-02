use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read from file");

    println!("{}", part_1(&input));

    println!("{}", part_2(&input));
}

fn part_1(input: &String) -> u32 {
    let mut previous_depth: Option<u32> = None;
    input
        .lines()
        .fold(0, |mut acc: u32, line| {
            let current_depth: u32 = line.parse().unwrap_or(0);

            if let Some(prev_depth) = previous_depth {
                if current_depth > prev_depth {
                    acc += 1;
                }
            }
            previous_depth = Some(current_depth);
            acc
        })
}

fn part_2(input: &String) -> u32 {
    let mut previous_depth: Option<u32> = None;
    input
        .lines()
        .tuple_windows()
        .fold(0, |mut acc, (a, b, c)| {
            let a: u32 = a.parse().unwrap_or(0);
            let b: u32 = b.parse().unwrap_or(0);
            let c: u32 = c.parse().unwrap_or(0);
            let sum = a + b + c;
            
            if let Some(prev_depth) = previous_depth {
                if sum > prev_depth {
                    acc += 1;
                }
            }
            previous_depth = Some(sum);
            acc
        })
}