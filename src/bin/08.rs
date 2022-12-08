fn init_height_matrix(input: &str) -> Vec<Vec<u32>> {
    let mut heights: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<u32> = Vec::new();
        for num in line.chars()
        {
            row.push(num.to_digit(10).unwrap());
        }
        heights.push(row);
    }
    heights
}

fn solve_problem_1(heights: &Vec<Vec<u32>>) -> usize {
    let mut counter = 2 * heights.len() + 2 * heights[0].len() - 4;
    for i in 1..heights.len() - 1 {
        for j in 1..heights[0].len() - 1 {
            let current_height = heights[i][j];
            let max_height_right = heights[i][j + 1..].iter().max().unwrap();
            let max_height_left = heights[i][0..j].iter().max().unwrap();
            let max_height_up = heights[0..i].iter().map(|row| row[j]).max().unwrap();
            let max_height_down = heights[i + 1..].iter().map(|row| row[j]).max().unwrap();

            if current_height > max_height_up || current_height > max_height_down || current_height > *max_height_left || current_height > *max_height_right {
                counter += 1;
            }
        }
    }
    counter
}

fn solve_problem_2(heights: &Vec<Vec<u32>>) -> usize {
    let mut max = 0;
    for i in 0..heights.len() {
        for j in 0..heights[0].len() {
            let current_height = heights[i][j];
            let heights_right: Vec<u32> = heights[i][j + 1..].to_vec();
            let heights_left: Vec<u32> = heights[i][0..j].iter().rev().copied().collect();
            let heights_up = heights[0..i].iter().rev().map(|row| row[j]).collect::<Vec<u32>>();
            let heights_down = heights[i + 1..].iter().map(|row| row[j]).collect::<Vec<u32>>();
            let directions = vec![heights_right, heights_left, heights_up, heights_down];

            let mut max_distances = [0; 4];
            for (dir_index,direction) in directions.iter().enumerate() {
                for (distance,height) in direction.iter().enumerate() {
                    max_distances[dir_index] = distance+1;
                    if *height >= current_height {
                        break;
                    }
                };
            }
            let distance = max_distances.iter().product();
            if distance > max {
                max = distance;
            }
        }
    }
    max
}

pub fn part_one(input: &str) -> Option<usize> {
    let heights = init_height_matrix(input);
    Some(solve_problem_1(&heights))
}

pub fn part_two(input: &str) -> Option<usize> {
    let heights = init_height_matrix(input);
    Some(solve_problem_2(&heights))
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
