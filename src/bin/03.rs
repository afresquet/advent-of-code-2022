struct Rucksack<'a>(&'a str, &'a str);

impl<'a> Rucksack<'a> {
    fn new(value: &'a str) -> Self {
        let (left, right) = value.split_at(value.len() / 2);
        Self(left, right)
    }

    fn find_repeated_item(&self) -> Option<usize> {
        let Rucksack(left, right) = self;
        let checked = check_items(left);
        right
            .chars()
            .map(item_to_priority_value)
            .find(|priority| checked[*priority - 1])
    }
}

struct RucksackGroup<'a>(&'a str, &'a str, &'a str);

impl<'a> RucksackGroup<'a> {
    fn find_badge(&self) -> Option<usize> {
        let RucksackGroup(one, two, three) = self;
        let checked_one = check_items(one);
        let checked_two = check_items(two);
        three
            .chars()
            .map(item_to_priority_value)
            .find(|priority| checked_one[*priority - 1] && checked_two[*priority - 1])
    }
}

fn check_items(items: &str) -> [bool; 52] {
    let mut checked = [false; 52];
    for item in items.chars().map(item_to_priority_value) {
        checked[item - 1] = true;
    }
    checked
}

fn item_to_priority_value(item: char) -> usize {
    let item = item as u8;
    let priority = if item > b'Z' {
        // 96 offsets 'a' to 1
        item - 96
    } else {
        // 38 offsets 'A' to 27
        item - 38
    };
    priority as usize
}

pub fn part_one(input: &str) -> Option<u32> {
    let value = input
        .split('\n')
        .map(Rucksack::new)
        .map(|rucksack| rucksack.find_repeated_item().unwrap_or(0) as u32)
        .sum();

    Some(value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let value = input
        .trim()
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| match chunk {
            [one, two, three] => RucksackGroup(one, two, three),
            _ => unreachable!(),
        })
        .map(|group| group.find_badge().unwrap_or(0) as u32)
        .sum();

    Some(value)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_find_repeated_item_in_rucksack() {
        let value = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rucksack = Rucksack::new(value);
        assert_eq!(rucksack.find_repeated_item(), Some(16));

        let value = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rucksack = Rucksack::new(value);
        assert_eq!(rucksack.find_repeated_item(), Some(38));

        let value = "PmmdzqPrVvPwwTWBwg";
        let rucksack = Rucksack::new(value);
        assert_eq!(rucksack.find_repeated_item(), Some(42));

        let value = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let rucksack = Rucksack::new(value);
        assert_eq!(rucksack.find_repeated_item(), Some(22));

        let value = "ttgJtRGJQctTZtZT";
        let rucksack = Rucksack::new(value);
        assert_eq!(rucksack.find_repeated_item(), Some(20));

        let value = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksack = Rucksack::new(value);
        assert_eq!(rucksack.find_repeated_item(), Some(19));
    }

    #[test]
    fn test_item_to_priority_value() {
        assert_eq!(item_to_priority_value('a'), 1);
        assert_eq!(item_to_priority_value('z'), 26);
        assert_eq!(item_to_priority_value('A'), 27);
        assert_eq!(item_to_priority_value('Z'), 52);
    }

    #[test]
    fn test_check_items() {
        let mut expected = [false; 52];
        expected[4] = true;
        expected[11] = true;
        expected[14] = true;
        expected[33] = true;
        assert_eq!(check_items("Hello"), expected);
    }

    #[test]
    fn test_find_badge_in_rucksack_group() {
        let group = RucksackGroup(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        );
        assert_eq!(group.find_badge(), Some(18));

        let group = RucksackGroup(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(group.find_badge(), Some(52));
    }
}
