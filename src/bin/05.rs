// This one is a mess

#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let mut iter = value.split_whitespace().flat_map(|s| s.parse());
        Instruction {
            amount: iter.next().unwrap(),
            from: iter.next().unwrap(),
            to: iter.next().unwrap(),
        }
    }
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose_vec<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters = v.into_iter().map(|n| n.into_iter()).collect::<Vec<_>>();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let stacks = stacks
        .split('\n')
        .map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| match chunk {
                    &[_, s, _, _] | &[_, s, _] if s != ' ' => Some(s),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .rev()
        .skip(1)
        .collect::<Vec<_>>();
    let stacks = transpose_vec(stacks);
    let mut stacks = stacks
        .iter()
        .map(|stack| stack.iter().flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let instructions: Vec<Instruction> = instructions
        .trim()
        .split('\n')
        .map(|line| line.into())
        .collect();

    for Instruction { amount, from, to } in instructions {
        for _ in 0..amount {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }

    let result: String = stacks
        .iter()
        .map(|stack| *stack.iter().next_back().unwrap_or(&&' '))
        .collect();

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let stacks = stacks
        .split('\n')
        .map(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| match chunk {
                    &[_, s, _, _] | &[_, s, _] if s != ' ' => Some(s),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .rev()
        .skip(1)
        .collect::<Vec<_>>();
    let stacks = transpose_vec(stacks);
    let mut stacks = stacks
        .iter()
        .map(|stack| stack.iter().flatten().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let instructions: Vec<Instruction> = instructions
        .trim()
        .split('\n')
        .map(|line| line.into())
        .collect();

    for Instruction { amount, from, to } in instructions {
        (0..amount)
            .map(|_| stacks[from - 1].pop().unwrap())
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .for_each(|c| stacks[to - 1].push(c));
    }

    let result: String = stacks
        .iter()
        .map(|stack| *stack.iter().next_back().unwrap_or(&&' '))
        .collect();

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        assert_eq!(part_one(input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";
        assert_eq!(part_two(input), Some("MCD".to_string()));
    }
}
