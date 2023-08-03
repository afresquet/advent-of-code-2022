fn parse_calories(input: &str) -> Vec<u32> {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|elf| -> u32 {
            elf.split_whitespace()
                .map(|str| str.parse::<u32>().unwrap())
                .sum()
        })
        .collect();
    calories.sort();
    calories
}

pub fn part_one(input: &str) -> Option<u32> {
    let calories = parse_calories(input);

    calories.into_iter().last()
}

pub fn part_two(input: &str) -> Option<u32> {
    let calories = parse_calories(input);

    Some(calories.into_iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24_000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45_000));
    }
}
