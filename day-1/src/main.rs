use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read from file");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &String) -> usize {
    input
        .lines()
        .map(|x| x.parse::<u32>().unwrap_or(0))
        .tuple_windows()
        .filter(|(x, y)| x < y)
        .count()
}

fn part_2(input: &String) -> usize {
    input
        .lines()
        .map(|x| x.parse::<u32>().unwrap_or(0))
        .tuple_windows()
        .filter(|(x, y, z, w)| x + y + z < y + z + w)
        .count()
}