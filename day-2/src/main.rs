use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read from file");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &String) -> i32 {
    let (h, d) = input
        .lines()
        .map(|line| {
            let (dir, amount) = line.split_once(" ").unwrap();
            let amount = amount.parse::<i32>().unwrap_or(0);
            (dir, amount)
        })
        .fold((0, 0), |acc, (dir, amount)| {
            match dir {
                "forward" => (acc.0 + amount, acc.1),
                "up" => (acc.0, acc.1 - amount),
                "down" => (acc.0, acc.1 + amount),
                _ => panic!("Unknown movement!")
            }
        });
    h * d
}

fn part_2(input: &String) -> i32 {
    let (h, d, _) = input
        .lines()
        .map(|line| {
            let (dir, amount) = line.split_once(" ").unwrap();
            let amount = amount.parse::<i32>().unwrap_or(0);
            (dir, amount)
        })
        .fold((0, 0, 0), |(horiz, depth, aim), (dir, amount)| {
            match dir {
                "forward" => (horiz + amount, depth + aim * amount, aim),
                "up" => (horiz, depth, aim - amount),
                "down" => (horiz, depth, aim + amount),
                _ => panic!("Unknown movement!")
            }
        });
    h * d
}