use std::collections::{HashMap, VecDeque};
use std::fmt::Formatter;

struct Ship {
    containers: HashMap<usize, Vec<String>>,
}

impl Ship {
    fn new() -> Self {
        Ship { containers: HashMap::new() }
    }

    fn add_container(&mut self, queue_id: usize, item: String) {
        let lifo = self.containers.entry(queue_id).or_insert(Vec::new());
        lifo.insert(0, item);
    }

    fn move_crate(&mut self, origin_index: &usize, destination_index: &usize) {
        let origin_queue = match self.containers.get_mut(origin_index) {
            Some(queue) => queue,
            None => return,
        };
        let popped_elem = match origin_queue.pop() {
            Some(elem) => elem,
            None => return,
        };
        let destination_queue = match self.containers.get_mut(destination_index) {
            Some(queue) => queue,
            None => return,
        };
        destination_queue.push(popped_elem);
    }

    fn move_multiple_crates(&mut self, origin_index: &usize, destination_index: &usize, amount: usize) {
        let origin_queue = match self.containers.get_mut(origin_index) {
            Some(queue) => queue,
            None => return,
        };
        let popped_elems=  origin_queue.split_off(origin_queue.len() - amount);
        let destination_queue = match self.containers.get_mut(destination_index) {
            Some(queue) => queue,
            None => return,
        };
        destination_queue.extend(popped_elems);
    }

    fn get_top_crates(&self) -> String {
        let mut output = String::from("");
        let vecs = self.containers.len();
        for i in 1..(vecs + 1) {
            let queue = match self.containers.get(&i) {
                Some(queue) => queue,
                None => {
                    println!("empty queue");
                    continue;
                }
            };
            output += queue.last().unwrap_or(&String::from(" "));
        }
        output
    }
}

impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.containers.get(&1).unwrap())
    }
}

fn parse_crates(ship: &mut Ship, line: &str) {
    for (index, char) in line.chars().enumerate() {
        if !char.is_alphabetic() { continue; }
        let queue_id = (index - 1) / 4 + 1;
        ship.add_container(queue_id, char.to_string());
    }
}

fn parse_move_instructions(ship: &mut Ship, line: &str, multiple_enabled: bool) {
    let x = line.to_string().replace("move ", "").replace(" from ", " ").replace(" to ", " ");
    let mut iter = x.split_whitespace();
    let amount = iter.next().unwrap().parse::<usize>().unwrap();
    let from = iter.next().unwrap().parse::<usize>().unwrap();
    let to = iter.next().unwrap().parse::<usize>().unwrap();
    println!("{} {} {}", amount, from, to);
    if multiple_enabled {
        ship.move_multiple_crates(&(from as usize), &(to as usize), amount);
    } else {
        for i in 0..amount {
            ship.move_crate(&(from as usize), &(to as usize));
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut ship = Ship::new();

    for line in input.lines() {
        match line {
            x if x.contains("[") => { parse_crates(&mut ship, line) }
            x if x.contains("move") => { parse_move_instructions(&mut ship, line, false) }
            _ => {}
        }
    }
    let output = ship.get_top_crates();
    Some(output)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut ship = Ship::new();

    for line in input.lines() {
        match line {
            x if x.contains("[") => { parse_crates(&mut ship, line) }
            x if x.contains("move") => { parse_move_instructions(&mut ship, line, true) }
            _ => {}
        }
    }
    let output = ship.get_top_crates();
    Some(output)
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
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
