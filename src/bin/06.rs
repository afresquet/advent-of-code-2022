const ALPHABET_LENGTH: usize = b'z' as usize - b'a' as usize + 1;

fn process_message(input: &str, window_size: usize) -> u32 {
    let (index, _) = input
        .chars()
        .collect::<Vec<_>>()
        .windows(window_size)
        .enumerate()
        .find(|(_, window)| {
            let mut checked = [false; ALPHABET_LENGTH];

            window.iter().all(|c| {
                let c = *c as usize - b'a' as usize;
                if checked[c] {
                    false
                } else {
                    checked[c] = true;
                    true
                }
            })
        })
        .unwrap();

    index as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let index = process_message(input, 4);

    Some(index + 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let index = process_message(input, 14);

    Some(index + 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_one(input), Some(5));

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_one(input), Some(6));

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_one(input), Some(10));

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_one(input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(part_two(input), Some(23));

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(part_two(input), Some(23));

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(part_two(input), Some(29));

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_two(input), Some(26));
    }
}
