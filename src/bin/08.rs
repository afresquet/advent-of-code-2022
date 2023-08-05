pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();

    let visible = (width * 2 + height * 2 - 4) as u32;

    let visible =
        grid.iter()
            .enumerate()
            .take(height - 1)
            .skip(1)
            .fold(visible, |acc, (i, row)| {
                acc + row
                    .iter()
                    .enumerate()
                    .take(width - 1)
                    .skip(1)
                    .fold(0, |acc, (j, tree)| {
                        let mut iters: [Box<dyn Iterator<Item = &u32>>; 4] = [
                            // TOP
                            Box::new(grid.iter().take(i).rev().map(|row| &row[j])),
                            // RIGHT
                            Box::new(grid[i].iter().take(width).skip(j + 1)),
                            // BOTTOM
                            Box::new(grid.iter().take(height).skip(i + 1).map(|row| &row[j])),
                            // LEFT
                            Box::new(grid[i].iter().take(j).rev()),
                        ];

                        let visible = iters
                            .iter_mut()
                            .any(|iter| iter.all(|other_tree| *tree > *other_tree));

                        if visible {
                            acc + 1
                        } else {
                            acc
                        }
                    })
            });

    Some(visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut scenic_scores = grid
        .iter()
        .enumerate()
        .take(height - 1)
        .skip(1)
        .map(|(i, row)| {
            let mut row_scores: Vec<u32> = row
                .iter()
                .enumerate()
                .take(width - 1)
                .skip(1)
                .map(|(j, tree)| {
                    let mut iters: [Box<dyn Iterator<Item = &u32>>; 4] = [
                        // TOP
                        Box::new(grid.iter().take(i).rev().map(|row| &row[j])),
                        // RIGHT
                        Box::new(grid[i].iter().take(width).skip(j + 1)),
                        // BOTTOM
                        Box::new(grid.iter().take(height).skip(i + 1).map(|row| &row[j])),
                        // LEFT
                        Box::new(grid[i].iter().take(j).rev()),
                    ];

                    let visible = iters.iter_mut().map(|iter| {
                        iter.scan(true, |state, other_tree| {
                            if !*state {
                                return None;
                            }

                            if *tree <= *other_tree {
                                *state = false;
                            }

                            Some(true)
                        })
                        .count() as u32
                    });

                    visible.product()
                })
                .collect();
            row_scores.sort();
            *row_scores.last().unwrap()
        })
        .collect::<Vec<_>>();
    scenic_scores.sort();

    Some(*scenic_scores.last().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
