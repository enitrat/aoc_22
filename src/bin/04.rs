struct ElfPair {
    assignment1: (u32, u32),
    assignment2: (u32, u32),
}

impl ElfPair {
    fn new(assignment1: (u32, u32), assignment2: (u32, u32)) -> ElfPair {
        ElfPair { assignment1, assignment2 }
    }

    fn are_assignements_subsets(&self) -> bool {
        let (x1, y1) = self.assignment1;
        let (x2, y2) = self.assignment2;
        if (x1 <= x2 && y2 <= y1) || (x2 <= x1 && y1 <= y2) {
            return true;
        }
        false
    }

    fn are_sets_disjoints(&self) -> bool {
        let (x1, y1) = self.assignment1;
        let (x2, y2) = self.assignment2;
        if x1 > y2 || x2 > y1 {
            return true;
        }
        false
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut counter = 0;
    for line in input.lines() {
        let mut assignments = line.split(",").collect::<Vec<&str>>();
        let assignment1 = assignments[0].split("-").collect::<Vec<&str>>();
        let assignment2 = assignments[1].split("-").collect::<Vec<&str>>();
        let assignment1 = (assignment1[0].parse::<u32>().unwrap(), assignment1[1].parse::<u32>().unwrap());
        let assignment2 = (assignment2[0].parse::<u32>().unwrap(), assignment2[1].parse::<u32>().unwrap());
        let elf_pair = ElfPair::new(assignment1, assignment2);
        counter += if elf_pair.are_assignements_subsets() { 1 } else { 0 };
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter = 0;
    for line in input.lines() {
        let mut assignments = line.split(",").collect::<Vec<&str>>();
        let assignment1 = assignments[0].split("-").collect::<Vec<&str>>();
        let assignment2 = assignments[1].split("-").collect::<Vec<&str>>();
        let assignment1 = (assignment1[0].parse::<u32>().unwrap(), assignment1[1].parse::<u32>().unwrap());
        let assignment2 = (assignment2[0].parse::<u32>().unwrap(), assignment2[1].parse::<u32>().unwrap());
        let elf_pair = ElfPair::new(assignment1, assignment2);
        counter += if elf_pair.are_sets_disjoints() { 0 } else { 1 };
    }
    Some(counter)
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
}
