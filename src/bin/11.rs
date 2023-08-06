use std::str::FromStr;

struct Item {
    monkey: usize,
    worry_level: u64,
}

impl Item {
    fn new(monkey: usize, worry_level: u64) -> Self {
        Self {
            monkey,
            worry_level,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    MultiplySelf,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, operation) = s.split_once('=').unwrap();
        let parts = operation.split_whitespace().collect::<Vec<_>>();
        match (parts[1], parts[2]) {
            ("*", "old") => Ok(Self::MultiplySelf),
            ("+", value) => Ok(Self::Add(value.parse::<u64>().unwrap())),
            ("*", value) => Ok(Self::Multiply(value.parse::<u64>().unwrap())),
            _ => Err("Invalid format"),
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible_by: u64,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Test {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .trim()
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<u64>>();

        Ok(Self {
            divisible_by: values[0],
            if_true: values[1] as usize,
            if_false: values[2] as usize,
        })
    }
}

#[derive(Debug)]
struct Monkey {
    number: usize,
    starting_items: Vec<u64>,
    operation: Operation,
    test: Test,
    monkey_business_level: u32,
}

impl Monkey {
    fn new(number: usize, starting_items: Vec<u64>, operation: Operation, test: Test) -> Self {
        Self {
            number,
            starting_items,
            operation,
            test,
            monkey_business_level: 0,
        }
    }

    fn round(&mut self, items: &mut Vec<Item>, worry_divider: u64, common_denominator: u64) {
        for item in items {
            if item.monkey != self.number {
                continue;
            }

            self.monkey_business_level += 1;

            let worry_level = item.worry_level;

            let worry_level = match self.operation {
                Operation::Add(value) => worry_level + value,
                Operation::Multiply(value) => worry_level * value,
                Operation::MultiplySelf => worry_level * worry_level,
            };

            let worry_level = worry_level % common_denominator;

            let worry_level = worry_level / worry_divider;

            item.worry_level = worry_level;

            let Test {
                divisible_by,
                if_true,
                if_false,
            } = self.test;

            if worry_level % divisible_by == 0 {
                item.monkey = if_true;
            } else {
                item.monkey = if_false;
            }
        }
    }
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let number = iter
            .next()
            .map(|line| {
                line.chars()
                    .nth(7)
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap()
            })
            .unwrap();
        let starting_items = iter
            .next()
            .map(|line| {
                let (_, items) = line.split_once(':').unwrap();
                items
                    .split(',')
                    .map(|item| item.trim().parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap();
        let operation = iter
            .next()
            .map(|line| line.parse::<Operation>().unwrap())
            .unwrap();
        let test = iter
            .map(|line| line.to_string() + "\n")
            .collect::<String>()
            .parse::<Test>()
            .unwrap();

        Ok(Self::new(number, starting_items, operation, test))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect();
    let mut items: Vec<Item> = monkeys
        .iter()
        .enumerate()
        .flat_map(|(i, monkey)| {
            monkey
                .starting_items
                .iter()
                .map(move |item| Item::new(i, *item))
                .collect::<Vec<_>>()
        })
        .collect();

    for _ in 0..20 {
        for monkey in &mut monkeys {
            monkey.round(&mut items, 3, u64::MAX);
        }
    }

    monkeys.sort_by_key(|monkey| monkey.monkey_business_level);

    Some(
        monkeys
            .iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.monkey_business_level)
            .product(),
    )
}

// Couldn't figure out what I had to do for this one so I watched this
// https://www.youtube.com/watch?v=0RkTrYDyzmE&t=2659s
pub fn part_two(input: &str) -> Option<u64> {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|monkey| monkey.parse::<Monkey>().unwrap())
        .collect();
    let mut items: Vec<Item> = monkeys
        .iter()
        .enumerate()
        .flat_map(|(i, monkey)| {
            monkey
                .starting_items
                .iter()
                .map(move |item| Item::new(i, *item))
                .collect::<Vec<_>>()
        })
        .collect();

    let common_denominator = monkeys
        .iter()
        .map(|monkey| monkey.test.divisible_by)
        .product();

    for _ in 0..10000 {
        for monkey in &mut monkeys {
            monkey.round(&mut items, 1, common_denominator);
        }
    }

    monkeys.sort_by_key(|monkey| monkey.monkey_business_level);

    Some(
        monkeys
            .iter()
            .rev()
            .take(2)
            .map(|monkey| monkey.monkey_business_level as u64)
            .product(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10_605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2_713_310_158));
    }
}
