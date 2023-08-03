#[derive(Debug, PartialEq, Eq)]
struct Assignment(usize, usize);

impl From<&str> for Assignment {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once('-').unwrap();
        Self(left.parse().unwrap(), right.parse().unwrap())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AssignmentPair(Assignment, Assignment);

impl AssignmentPair {
    fn is_overlapping(&self) -> bool {
        match self {
            AssignmentPair(Assignment(a, _), Assignment(c, d)) if a == c || a == d => true,
            AssignmentPair(Assignment(_, b), Assignment(c, d)) if b == c || b == d => true,
            AssignmentPair(Assignment(a, b), Assignment(c, _)) if a <= c && b >= c => true,
            AssignmentPair(Assignment(a, _), Assignment(c, d)) if a >= c && a <= d => true,
            AssignmentPair(Assignment(a, b), Assignment(_, d)) if a <= d && b >= d => true,
            AssignmentPair(Assignment(_, b), Assignment(c, d)) if b >= c && b <= d => true,
            _ => self.is_fully_overlapping(),
        }
    }

    fn is_fully_overlapping(&self) -> bool {
        match self {
            AssignmentPair(Assignment(a, b), Assignment(c, d)) if a <= c && b >= d => true,
            AssignmentPair(Assignment(a, b), Assignment(c, d)) if c <= a && d >= b => true,
            _ => false,
        }
    }
}

impl From<&str> for AssignmentPair {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(',').unwrap();
        Self(left.into(), right.into())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let fully_overlapping_amount = input
        .trim()
        .split('\n')
        .map(|value| -> AssignmentPair { value.into() })
        .filter(|pair| pair.is_fully_overlapping())
        .count();

    Some(fully_overlapping_amount as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let overlapping_amount = input
        .trim()
        .split('\n')
        .map(|value| -> AssignmentPair { value.into() })
        .filter(|pair| pair.is_overlapping())
        .count();

    Some(overlapping_amount as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }

    #[test]
    fn test_into_assignment() {
        assert_eq!(Assignment(1, 2), "1-2".into())
    }

    #[test]
    fn test_into_assignment_pair() {
        assert_eq!(
            AssignmentPair(Assignment(1, 2), Assignment(3, 4)),
            "1-2,3-4".into()
        )
    }

    #[test]
    fn test_assignment_is_overlapping() {
        assert!(AssignmentPair(Assignment(5, 7), Assignment(7, 9)).is_overlapping());

        assert!(AssignmentPair(Assignment(2, 8), Assignment(3, 7)).is_overlapping());

        assert!(AssignmentPair(Assignment(6, 6), Assignment(4, 6)).is_overlapping());

        assert!(AssignmentPair(Assignment(2, 6), Assignment(4, 8)).is_overlapping());

        assert!(!AssignmentPair(Assignment(1, 2), Assignment(3, 4)).is_overlapping());
    }

    #[test]
    fn test_assignment_is_fully_overlapping() {
        assert!(AssignmentPair(Assignment(2, 8), Assignment(3, 7)).is_fully_overlapping());

        assert!(AssignmentPair(Assignment(6, 6), Assignment(4, 6)).is_fully_overlapping());

        assert!(!AssignmentPair(Assignment(1, 2), Assignment(3, 4)).is_fully_overlapping());
    }
}
