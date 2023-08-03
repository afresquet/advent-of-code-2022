use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct Turn(Move, Move);

impl Turn {
    fn instruction(self) -> Self {
        let m = match self {
            Turn(m, Move::Rock) => m.lose(),
            Turn(m, Move::Paper) => m.draw(),
            Turn(m, Move::Scissors) => m.win(),
        };
        Self(self.0, m)
    }

    fn resolve(self) -> u32 {
        let points = match self {
            Turn(x, y) if x < y => 6,
            Turn(x, y) if x == y => 3,
            Turn(x, y) if x > y => 0,
            _ => unreachable!(),
        };
        points + self.1 as u32
    }
}

impl From<&str> for Turn {
    fn from(value: &str) -> Self {
        value.split_once(' ').unwrap().into()
    }
}

impl From<(&str, &str)> for Turn {
    fn from(value: (&str, &str)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn win(&self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn draw(&self) -> Self {
        *self
    }

    fn lose(&self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            s => panic!("Can't convert {s:?} to Move"),
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Move::Rock, Move::Paper) => Ordering::Less,
            (Move::Rock, Move::Scissors) => Ordering::Greater,
            (Move::Paper, Move::Rock) => Ordering::Greater,
            (Move::Paper, Move::Scissors) => Ordering::Less,
            (Move::Scissors, Move::Rock) => Ordering::Less,
            (Move::Scissors, Move::Paper) => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input.split('\n').map(|turn| -> u32 {
        let turn: Turn = turn.into();
        turn.resolve()
    });

    Some(input.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.split('\n').map(|turn| -> u32 {
        let turn: Turn = turn.into();
        turn.instruction().resolve()
    });

    Some(input.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }

    #[test]
    fn test_into_move() {
        assert_eq!(Move::Rock, "A".into());
        assert_eq!(Move::Rock, "X".into());
        assert_eq!(Move::Paper, "B".into());
        assert_eq!(Move::Paper, "Y".into());
        assert_eq!(Move::Scissors, "C".into());
        assert_eq!(Move::Scissors, "Z".into());
    }

    #[test]
    fn test_into_turn() {
        assert_eq!(Turn(Move::Rock, Move::Paper), "A Y".into());
        assert_eq!(Turn(Move::Paper, Move::Scissors), "B Z".into());
        assert_eq!(Turn(Move::Scissors, Move::Rock), "C X".into());
    }

    #[test]
    fn test_turn_resolve() {
        assert_eq!(Turn(Move::Rock, Move::Paper).resolve(), 8);
        assert_eq!(Turn(Move::Paper, Move::Rock).resolve(), 1);
        assert_eq!(Turn(Move::Scissors, Move::Scissors).resolve(), 6);
    }

    #[test]
    fn test_turn_instruction() {
        assert_eq!(
            Turn(Move::Rock, Move::Paper).instruction(),
            Turn(Move::Rock, Move::Rock)
        );
        assert_eq!(
            Turn(Move::Paper, Move::Rock).instruction(),
            Turn(Move::Paper, Move::Rock)
        );
        assert_eq!(
            Turn(Move::Scissors, Move::Scissors).instruction(),
            Turn(Move::Scissors, Move::Rock)
        );
    }
}
