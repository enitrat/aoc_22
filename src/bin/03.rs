use std::collections::HashSet;
use std::fmt::Display;

struct Rucksack {
    id: i32,
    compartment1: Vec<String>,
    compartment2: Vec<String>,
    shared_items: Vec<String>,
}

struct ElfGroup {
    id: i32,
    rucksacks: Vec<Rucksack>,
    priority: i32,
}

impl Rucksack {
    fn new(id: i32) -> Self {
        Rucksack {
            id,
            compartment1: Vec::new(),
            compartment2: Vec::new(),
            shared_items: Vec::new(),
        }
    }

    fn new_from_items(id: i32, items: &str) -> Self {
        let mut rucksack = Rucksack::new(id);
        let nb_items = items.len();
        let (left, right) = items.split_at(nb_items / 2);
        Self::add_to_compartment(&mut rucksack, left, 1);
        Self::add_to_compartment(&mut rucksack, right, 2);
        Self::add_shared_items(&mut rucksack);

        rucksack
    }

    fn calculate_priority(&self) -> i32 {
        let shared_items_priorities = self.shared_items.iter().fold(
            0,
            |mut acc, item| {
                let priority = get_char_priority(item.as_str());
                acc += priority;
                acc
            },
        );
        shared_items_priorities
    }

    fn add_shared_items(&mut self) {
        let mut shared_items: Vec<String> = self.compartment1.iter().filter(|&x| self.compartment2.contains(x))
            .map(|x| x.to_string())
            .collect::<HashSet<_>>()
            .into_iter().collect();
        self.shared_items.extend(shared_items);
    }

    fn add_to_compartment(&mut self, items: &str, compartment: i32) {
        let mut chars: Vec<String> = items.chars().map(|c| c.to_string()).collect();
        match compartment {
            1 => self.compartment1.extend(chars),
            2 => self.compartment2.extend(chars),
            _ => panic!("Invalid compartment number"),
        }
    }

    fn collect_compartiments(&self) -> Vec<String> {
        let mut compartiments = self.compartment1.clone();
        compartiments.append(&mut self.compartment2.clone());
        compartiments
    }
}

impl Display for Rucksack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rucksack {} has {} items in compartment 1 and {} items in compartment 2", self.id, self.compartment1.len(), self.compartment2.len())
    }
}

impl ElfGroup {
    fn new(id: i32) -> Self {
        ElfGroup {
            id,
            rucksacks: Vec::new(),
            priority: 0,
        }
    }

    fn add_rucksack(&mut self, rucksack: Rucksack) {
        self.rucksacks.push(rucksack);
    }

    fn get_shared_items(&self) -> Vec<String> {
        let group_items = self.rucksacks.iter().map(|r| Rucksack::collect_compartiments(r)).collect::<Vec<Vec<String>>>();
        // Only keep that are shared by elf1, elf2 and elf3
        let mut shared_items: Vec<String> = group_items.iter().fold(
            group_items[0].clone(),
            |mut acc, elf| {
                acc.retain(|x| elf.contains(x));
                acc
            },
        );
        // no duplicates
        shared_items = shared_items.into_iter().collect::<HashSet<_>>().into_iter().collect();
        shared_items
    }

    fn calculate_priority(&mut self) -> i32 {
        let shared_items = self.get_shared_items();
        println!("Shared items: {:?}", shared_items);

        //Optimization : Use Iterator::sum instead of Iterator::fold
        let group_priority = shared_items.iter().map(|r| get_char_priority(r.as_str())).sum();
        group_priority
    }
}

// Using a &str reference avoids allocating a new String when get_char_priority only needs a reference to the original string.
fn get_char_priority(item: &str) -> i32 {
    let priority = match item.chars().next().unwrap() {
        c if c.is_lowercase() => c as i32 - 96,
        c if c.is_uppercase() => c as i32 - 64 + 26,
        _ => 0,
    };
    priority
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut rucksacks: Vec<Rucksack> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let rucksack = Rucksack::new_from_items(index as i32, line);
        println!("{:?} ", rucksack.shared_items);
        rucksacks.push(rucksack);
    }

    Some(rucksacks.iter().fold(0, |mut acc, rucksack| {
        acc += Rucksack::calculate_priority(rucksack);
        acc
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut elf_groups: Vec<ElfGroup> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if (index % 3) == 0 {
            let elf_group = ElfGroup::new(index as i32);
            elf_groups.push(elf_group);
        }
        let rucksack = Rucksack::new_from_items(index as i32, line);
        elf_groups.last_mut()?.add_rucksack(rucksack);
    }

    Some(elf_groups.iter_mut().fold(0, |mut acc, elf_group| {
        acc += elf_group.calculate_priority();
        acc
    }))
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
}
