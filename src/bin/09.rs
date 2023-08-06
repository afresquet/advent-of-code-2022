use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Motion {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Motion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, amount) = s.split_once(' ').ok_or("Invalid format")?;
        let amount: u32 = amount
            .parse()
            .map_err(|_| "Can't parse amount of movements")?;
        match direction {
            "U" => Ok(Self::Up(amount)),
            "D" => Ok(Self::Down(amount)),
            "L" => Ok(Self::Left(amount)),
            "R" => Ok(Self::Right(amount)),
            _ => Err("Invalid direction"),
        }
    }
}

struct MotionIterator {
    motion: Motion,
    index: u32,
    range: u32,
}

impl Iterator for MotionIterator {
    type Item = Motion;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.range {
            return None;
        }

        self.index += 1;

        Some(self.motion)
    }
}

impl IntoIterator for Motion {
    type Item = Motion;

    type IntoIter = MotionIterator;

    fn into_iter(self) -> Self::IntoIter {
        let (motion, range) = match self {
            Motion::Up(amount) => (Motion::Up(1), amount),
            Motion::Down(amount) => (Motion::Down(1), amount),
            Motion::Left(amount) => (Motion::Left(1), amount),
            Motion::Right(amount) => (Motion::Right(1), amount),
        };

        MotionIterator {
            motion,
            index: 0,
            range,
        }
    }
}

trait Move {
    fn set_x(&mut self, amount: i32);
    fn set_y(&mut self, amount: i32);
}

#[derive(Debug, PartialEq, Eq)]
struct Knot {
    x: i32,
    y: i32,
    positions: BTreeSet<(i32, i32)>,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        let mut positions = BTreeSet::new();
        positions.insert((x, y));
        Self { x, y, positions }
    }

    fn move_to(&mut self, motion: &Motion) {
        match motion {
            Motion::Left(amount) => self.x -= *amount as i32,
            Motion::Right(amount) => self.x += *amount as i32,
            Motion::Up(amount) => self.y += *amount as i32,
            Motion::Down(amount) => self.y -= *amount as i32,
        }
    }

    fn follow(&mut self, head: &Knot) {
        match (self.x, self.y, head.x, head.y) {
            (tx, ty, hx, hy)
                if (tx == hx && ty == hy)
                    || ((tx - hx).abs() == 1 && (ty - hy).abs() == 1)
                    || (ty == hy && (tx - hx).abs() == 1)
                    || (tx == hx && (ty - hy).abs() == 1) =>
            {
                return;
            }
            (tx, ty, hx, hy) if ty == hy && (tx - hx).abs() > 1 => {
                if tx < hx {
                    self.move_to(&Motion::Right(1));
                } else {
                    self.move_to(&Motion::Left(1));
                }
            }
            (tx, ty, hx, hy) if tx == hx && (ty - hy).abs() > 1 => {
                if ty < hy {
                    self.move_to(&Motion::Up(1));
                } else {
                    self.move_to(&Motion::Down(1));
                }
            }
            (tx, ty, hx, hy)
                if (ty != hy && (tx - hx).abs() == 1)
                    || (tx != hx && (ty - hy).abs() == 1)
                    || ((tx - hx).abs() == 2 && (ty - hy).abs() == 2) =>
            {
                if ty < hy {
                    self.move_to(&Motion::Up(1));
                } else {
                    self.move_to(&Motion::Down(1));
                }
                if tx < hx {
                    self.move_to(&Motion::Right(1));
                } else {
                    self.move_to(&Motion::Left(1));
                }
            }
            _ => {
                eprintln!("{} - {} | {} - {}", self.x, self.y, head.x, head.y);
                unreachable!();
            }
        }

        self.positions.insert((self.x, self.y));
        self.follow(head);
    }
}

impl Move for Knot {
    fn set_x(&mut self, amount: i32) {
        self.x += amount;
    }

    fn set_y(&mut self, amount: i32) {
        self.y += amount;
    }
}

impl Default for Knot {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl From<(i32, i32)> for Knot {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut head = Knot::default();
    let mut tail = Knot::default();

    for m in input.lines().map(|line| line.parse::<Motion>().unwrap()) {
        head.move_to(&m);
        tail.follow(&head);
    }

    Some(tail.positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut knots: Vec<Knot> = (0..10).map(|_| Knot::default()).collect();

    for motion in input.lines().map(|line| line.parse::<Motion>().unwrap()) {
        for m in motion {
            for i in 1..knots.len() {
                if i == 1 {
                    let head = &mut knots[i - 1];
                    head.move_to(&m);
                }

                let head = &knots[i - 1];
                let head = Knot::from((head.x, head.y));
                let tail = &mut knots[i];

                tail.follow(&head);
            }
        }
    }

    Some(knots.last().unwrap().positions.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(part_two(input), Some(36));
    }

    #[test]
    fn test_move() {
        let mut head = Knot::default();
        head.move_to(&Motion::Up(5));
        assert_eq!((head.x, head.y), (0, 5));

        head.move_to(&Motion::Right(5));
        assert_eq!((head.x, head.y), (5, 5));

        head.move_to(&Motion::Down(5));
        assert_eq!((head.x, head.y), (5, 0));

        head.move_to(&Motion::Left(5));
        assert_eq!((head.x, head.y), (0, 0));
    }

    #[test]
    fn test_follow_axis() {
        let mut head = Knot::default();
        let mut tail = Knot::default();
        head.move_to(&Motion::Up(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (0, 4));
        head.move_to(&Motion::Down(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (0, 1));

        let mut head = Knot::default();
        let mut tail = Knot::default();
        head.move_to(&Motion::Right(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (4, 0));
        head.move_to(&Motion::Left(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (1, 0));
    }

    #[test]
    fn test_follow_diagonal() {
        let mut head = Knot::default();
        let mut tail = Knot::default();
        head.move_to(&Motion::Up(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (0, 4));
        head.move_to(&Motion::Right(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (4, 5));
        head.move_to(&Motion::Down(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (5, 1));
        head.move_to(&Motion::Left(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (1, 0));
    }

    #[test]
    fn test_ignore_same_place() {
        let mut head = Knot::default();
        let mut tail = Knot::default();
        head.move_to(&Motion::Up(5));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (0, 4));
        head.move_to(&Motion::Down(1));
        tail.follow(&head);
        assert_eq!((tail.x, tail.y), (0, 4));
    }
}
