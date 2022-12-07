use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

pub type NodeIndex = usize;

#[derive(Debug, Clone)]
struct Tree {
    nodes: HashMap<String, Directory>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            nodes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Directory {
    path: String,
    parent: Option<String>,
    children: Vec<String>,
    files_size: usize,
}

#[derive(Debug, Clone)]
struct State {
    capacity: u64,
    current_directory: String,
    tree: Tree,
}

impl State {
    fn new() -> State {
        State {
            capacity: 70000000,
            current_directory: String::from("/"),
            tree: Tree::new(),
        }
    }

    fn change_directory(&mut self, directory: String) {
        match directory.as_str() {
            ".." => {
                let mut path = self.current_directory.split('/').collect::<Vec<&str>>();
                path.pop();
                self.current_directory = path.join("/");
            }
            d if d.starts_with("/") => {
                self.current_directory = directory;
            }
            _ => {
                if self.current_directory.ends_with("/") {
                    self.current_directory = format!("{}{}", self.current_directory, directory);
                } else {
                    self.current_directory = format!("{}/{}", self.current_directory, directory);
                }
            }
        }
    }


    fn store_current_dir(&mut self, size: usize) {
        let parent = Path::new(&self.current_directory).parent();
        let parent = parent.map(|p| p.to_str().unwrap().to_string());
        self.tree.nodes.insert(self.current_directory.clone(), Directory {
            path: self.current_directory.clone(),
            parent: parent.clone(),
            children: Vec::new(),
            files_size: size,
        });
        if let Some(p) = parent {
            self.tree.nodes.get_mut(&p).unwrap().children.push(self.current_directory.clone());
        }
    }

    fn sum_recursive_size(&mut self, dir: &Directory) -> usize {
        let mut sum = dir.files_size;
        for child in dir.children.iter() {
            if let Some(child) = self.tree.nodes.get(child) {
                sum += self.clone().sum_recursive_size(child);
            }
        }
        sum
    }
}

fn get_dir_filesize(files: Vec<String>) -> usize {
    files.iter().fold(0, |mut acc, file| {
        acc += match file {
            file if file.starts_with("dir") => 0,
            _ => file.split(' ').collect::<Vec<&str>>()[0].parse::<usize>().unwrap()
        };
        acc
    })
}

fn sum_sub_100k_dirs(rec_sums: HashMap<String, usize>) -> usize {
    rec_sums.iter().fold(0, |mut acc, (_, v)| {
        if v < &100000 {
            acc += v;
        }
        acc
    })
}

fn find_smallest_folder(rec_sums: HashMap<String, usize>, target: usize) -> usize {
    let mut max = 70000000;
    rec_sums.iter().for_each(|(_, v)| {
        if v < &max && v >= &target {
            max = *v;
        }
    });
    max
}

fn read_filesystem(input: &str) -> State {
    let mut state = State::new();
    let command_blocks = input.split('$');
    for mut block in command_blocks {
        block = block.trim();
        let lines = block.split('\n')
            .map(|line| { line.to_string() })
            .collect::<Vec<String>>();
        let command_line = lines[0].split(' ').collect::<Vec<&str>>();
        match command_line[0] {
            "cd" => state.change_directory(command_line[1].to_string()),
            "ls" => {
                let files_size = get_dir_filesize(lines[1..].to_vec());
                state.store_current_dir(files_size);
            }
            _ => println!("Unknown command"),
        };
    }
    state
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut state = read_filesystem(input);
    let mut rec_sums: HashMap<String, usize> = HashMap::new();
    state.tree.nodes.iter().for_each(|(k, v)| {
        rec_sums.insert(k.clone(), state.clone().sum_recursive_size(v));
    });
    Some(sum_sub_100k_dirs(rec_sums))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut state = read_filesystem(input);
    let mut rec_sums: HashMap<String, usize> = HashMap::new();
    state.tree.nodes.iter().for_each(|(k, v)| {
        rec_sums.insert(k.clone(), state.clone().sum_recursive_size(v));
    });
    let target = 70000000 - rec_sums.get("/").unwrap();
    Some(find_smallest_folder(rec_sums, target))
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
