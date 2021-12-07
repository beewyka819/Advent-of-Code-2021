use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read from file");

    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

fn part_1(input: &String) -> isize {
    let mat: Vec<Vec<isize>> = transpose(input.lines()
        .map(|line| line.chars()
            .map(|number| number.to_string().parse().unwrap_or(0))
            .collect())
        .collect());

    let (gamma, epsilon) = mat.into_iter()
        .fold(("".to_string(), "".to_string()), |(mut gamma, mut epsilon), num| {
            let ones = num.clone().into_iter()
                .filter(|&x| x == 1)
                .count();
            let (gamma_bit, epsilon_bit) = if ones > num.len() - ones { (1, 0) } else { (0, 1) };
            gamma.push_str(&gamma_bit.to_string());
            epsilon.push_str(&epsilon_bit.to_string());
            (gamma, epsilon)
        });
    return isize::from_str_radix(gamma.as_str(), 2).unwrap() * isize::from_str_radix(epsilon.as_str(), 2).unwrap();
}

fn part_2(input: &String) -> isize {
    let mut oxy = input.clone();
    let mut co2 = input.clone();
    for i in 0..input.lines().next().unwrap().len() {
        oxy = oxy.lines()
            .filter(|line| {
                if oxy.lines().count() == 1 {
                    return true;
                }
                
                let nums: Vec<i32> = oxy.lines()
                    .map(|line| line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap())
                    .collect();
                let ones = nums.clone()
                    .into_iter()
                    .filter(|&x| x == 1)
                    .count();
                if ones == nums.len() - ones {
                    line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap() == 1
                } else {
                    let common = if ones > nums.len() - ones { 1 } else { 0 };
                    line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap() == common
                }
                
            })
            .join("\n");

        co2 = co2.lines()
            .filter(|line| {
                if co2.lines().count() == 1 {
                    return true;
                }

                let nums: Vec<i32> = co2.lines()
                    .map(|line| line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap())
                    .collect();
                let ones = nums.clone()
                    .into_iter()
                    .filter(|&x| x == 1)
                    .count();
                if ones == nums.len() - ones {
                    line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap() == 0
                } else {
                    let ncommon = if ones > nums.len() - ones { 0 } else { 1 };
                    line.chars().nth(i).unwrap().to_string().parse::<i32>().unwrap() == ncommon
                }
            })
            .join("\n");
    }

    return isize::from_str_radix(oxy.as_str(), 2).unwrap() * isize::from_str_radix(co2.as_str(), 2).unwrap();
}

fn transpose(mat: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut out = vec![vec![0; mat.len()]; mat[0].len()];
    for (row, vec) in mat.clone().into_iter().enumerate() {
        for (col, elem) in vec.into_iter().enumerate() {
            out[col][row] = elem;
        }
    }
    out
}