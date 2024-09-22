use std::collections::HashSet;
use std::hash::Hash;
use std::time::Instant;
use thousands::Separable;

pub fn run() {
    println!("Day 4: Scratchcards");
    let input = include_str!("input.txt");

    for i in 1..=2 {
        let now = Instant::now();
        let result = if i % 2 == 1 {
            part_one(input)
        } else {
            part_two(input)
        };
        let elapsed = now.elapsed();
        println!(
            "Part {}: {} ({:.2?})",
            i,
            result.separate_with_commas(),
            elapsed
        );
    }
    println!();
}
fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, numbers) = line.split_once(": ").unwrap();
            let (a, b) = numbers.split_once(" | ").unwrap();
            let a: HashSet<u32> = a
                .split_whitespace()
                .map(|n| n.parse().expect("invalid number"))
                .collect();
            let b: HashSet<u32> = b
                .split_whitespace()
                .map(|n| n.parse().expect("invalid number"))
                .collect();
            let intersections = a.intersection(&b).copied().collect::<Vec<u32>>().len();
            if intersections == 0 {
                0
            } else {
                2_u32.pow((intersections - 1).try_into().unwrap())
            }
        })
        .sum()
}
fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = include_str!("example.txt");
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("input.txt");
        assert_eq!(part_one(input), 21_568);
    }

    // #[test]
    // fn test_part_two_example() {
    //     let input = include_str!("example.txt");
    //     assert_eq!(part_two(input), 467_835);
    // }
    //
    // #[test]
    // fn test_part_two() {
    //     let input = include_str!("input.txt");
    //     assert_eq!(part_two(input), 75_220_503);
    // }
}
