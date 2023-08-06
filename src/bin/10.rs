use std::str::FromStr;

enum Operation {
    NoOp,
    AddX(i32),
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut op = s.split(' ');
        let op = (op.next(), op.next());

        match op {
            (Some("noop"), _) => Ok(Operation::NoOp),
            (Some("addx"), Some(value)) => {
                let value = value.parse().map_err(|_| "Can't parse value to i32")?;
                Ok(Operation::AddX(value))
            }
            _ => Err("Invalid operation"),
        }
    }
}

struct Cpu {
    register_x: i32,
}

impl Cpu {
    fn new() -> Self {
        Self { register_x: 1 }
    }

    fn cycle(&mut self, operation: &Operation) -> i32 {
        let x = self.register_x;

        match operation {
            Operation::NoOp => x,
            Operation::AddX(value) => {
                self.register_x += value;
                x
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = Cpu::new();
    let operations = input
        .lines()
        .map(|line| line.parse::<Operation>().unwrap())
        .flat_map(|op| match op {
            Operation::NoOp => vec![op],
            Operation::AddX(_) => vec![Operation::NoOp, op],
        });
    let signal_strength_cycles =
        operations
            .map(|op| cpu.cycle(&op))
            .enumerate()
            .filter_map(|(i, value)| {
                if (i + 1) % 40 == 20 {
                    Some(value * (i + 1) as i32)
                } else {
                    None
                }
            });

    Some(signal_strength_cycles.sum())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cpu = Cpu::new();
    let operations = input
        .lines()
        .map(|line| line.parse::<Operation>().unwrap())
        .flat_map(|op| match op {
            Operation::NoOp => vec![op],
            Operation::AddX(_) => vec![Operation::NoOp, op],
        });
    let crt_drawing = operations
        .enumerate()
        .map(|(i, op)| {
            let register_x = cpu.cycle(&op);

            let i = i % 40;

            let char = if i as i32 >= register_x - 1 && i as i32 <= register_x + 1 {
                "#".to_string()
            } else {
                ".".to_string()
            };

            if i == 39 {
                char + "\n"
            } else {
                char
            }
        })
        .collect::<String>();

    Some(crt_drawing.trim().to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string();
        assert_eq!(part_two(&input), Some(expected));
    }
}
