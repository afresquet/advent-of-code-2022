// This one is also a mess

const TOTAL_SPACE_AVAILABLE: u32 = 70000000;
const SPACE_NEEDED_FOR_UPDATE: u32 = 30000000;

#[derive(Debug)]
struct File(u32);

#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    items: Option<Vec<FSItem<'a>>>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Self { name, items: None }
    }
}

#[derive(Debug)]
enum FSItem<'a> {
    File(File),
    Directory(Directory<'a>),
}

impl<'a> FSItem<'a> {
    fn new(s: &'a str) -> Self {
        match s.split_once(' ') {
            Some(("dir", dir)) => Self::Directory(Directory::new(dir)),
            Some((size, _)) => Self::File(File(size.parse().unwrap())),
            None => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Command<'a> {
    CD(&'a str),
    LS(Vec<&'a str>),
}

fn flatten_directories<'a>(item: &'a FSItem<'a>) -> Vec<&'a FSItem<'a>> {
    match item {
        FSItem::File(_) => Vec::new(),
        FSItem::Directory(d) => {
            let mut directories = Vec::new();
            for item in d.items.as_ref().unwrap() {
                if let FSItem::Directory(_) = item {
                    directories.push(item);
                }
                directories.extend(flatten_directories(item));
            }
            directories
        }
    }
}

fn calculate_directory_size(item: &FSItem) -> u32 {
    match item {
        FSItem::File(File(size)) => *size,
        FSItem::Directory(d) => d
            .items
            .as_ref()
            .unwrap()
            .iter()
            .map(|item| match item {
                FSItem::File(File(size)) => *size,
                FSItem::Directory(_) => calculate_directory_size(item),
            })
            .sum(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands: Vec<Command> = input
        .split('$')
        .skip(1)
        .map(|cmd| {
            let mut iter = cmd.trim().lines();
            match iter.next() {
                Some("ls") => Command::LS(iter.collect()),
                Some(cd) => Command::CD(cd.split_once(' ').unwrap().1),
                None => unreachable!(),
            }
        })
        .skip(1)
        .collect();

    let mut root = FSItem::Directory(Directory::new("/"));
    let mut path: Vec<&mut FSItem> = vec![&mut root];

    for command in commands {
        match command {
            Command::CD("..") => {
                path.pop();
            }
            Command::CD(name) => {
                let Some(FSItem::Directory(dir)) = path.last_mut() else {
                    unreachable!();
                };

                let dir = dir
                    .items
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|item| match item {
                        FSItem::Directory(d) => d.name == name,
                        FSItem::File(_) => false,
                    })
                    .unwrap();

                // SAFETY: we only ever mutate the directory at the LS command to add
                // items to it, it will never be mutated elsewhere, and it will also never
                // be a null pointer since we don't get rid of it in the entire
                // execution, so we are safe to have this pointer.
                unsafe {
                    let const_ptr = dir as *const FSItem;
                    let mut_ptr = const_ptr as *mut FSItem;

                    path.push(&mut *mut_ptr);
                }
            }
            Command::LS(items) => {
                let Some(FSItem::Directory(dir)) = path.last_mut() else {
                    unreachable!();
                };

                let _ = dir
                    .items
                    .insert(items.iter().map(|item| FSItem::new(item)).collect());
            }
        }
    }

    let directories = flatten_directories(&root);

    Some(
        directories
            .into_iter()
            .map(calculate_directory_size)
            .filter(|size| *size <= 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands: Vec<Command> = input
        .split('$')
        .skip(1)
        .map(|cmd| {
            let mut iter = cmd.trim().lines();
            match iter.next() {
                Some("ls") => Command::LS(iter.collect()),
                Some(cd) => Command::CD(cd.split_once(' ').unwrap().1),
                None => unreachable!(),
            }
        })
        .skip(1)
        .collect();

    let mut root = FSItem::Directory(Directory::new("/"));
    let mut path: Vec<&mut FSItem> = vec![&mut root];

    for command in commands {
        match command {
            Command::CD("..") => {
                path.pop();
            }
            Command::CD(name) => {
                let Some(FSItem::Directory(dir)) = path.last_mut() else {
                    unreachable!();
                };

                let dir = dir
                    .items
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|item| match item {
                        FSItem::Directory(d) => d.name == name,
                        FSItem::File(_) => false,
                    })
                    .unwrap();

                unsafe {
                    let const_ptr = dir as *const FSItem;
                    let mut_ptr = const_ptr as *mut FSItem;

                    path.push(&mut *mut_ptr);
                }
            }
            Command::LS(items) => {
                let Some(FSItem::Directory(dir)) = path.last_mut() else {
                    unreachable!();
                };

                let _ = dir
                    .items
                    .insert(items.iter().map(|item| FSItem::new(item)).collect());
            }
        }
    }

    let directories = flatten_directories(&root);
    let used_space = calculate_directory_size(&root);
    let unused_space = TOTAL_SPACE_AVAILABLE - used_space;
    let needed_space = SPACE_NEEDED_FOR_UPDATE - unused_space;

    let mut sizes = directories
        .into_iter()
        .map(calculate_directory_size)
        .collect::<Vec<u32>>();
    sizes.sort();
    sizes.into_iter().find(|size| *size > needed_space)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
