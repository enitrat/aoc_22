fn find_unique_set(input: &str, size: usize) -> u32 {
    let mut counter = 0;
    let mut stored_chars: Vec<char> = Vec::new();
    for char in input.chars() {
        counter += 1;
        if let Some(index) = stored_chars.iter().position(|&x| x == char) {
            stored_chars.drain(0..index + 1);
        }
        stored_chars.push(char);
        if stored_chars.len() == size {
            break;
        };
    }
    counter
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_unique_set(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_unique_set(input, 14))
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
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
