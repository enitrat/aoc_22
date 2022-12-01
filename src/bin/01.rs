pub fn part_one(input: &str) -> Option<u32> {

    // Dumb solution - just build a vector, sort it and return the last element
    // let groups = input.split("\n\n");
    // let mut weights: Vec<u32> = Vec::new();
    // for group in groups {
    //     let lines = group.split("\n");
    //     let mut calories = 0;
    //     for line in lines {
    //         calories+= line.parse::<u32>().unwrap_or(0);
    //     };
    //     weights.push(calories);
    // }
    // weights.sort();
    // println!("{}", weights[weights.len()-1]);
    // Some(weights[weights.len()-1])

    // Optimized solution
    let mut best_sum = 0;
    let mut current_calories = 0;
    for line in input.lines() {
        // if line is an empty line, reset current_calories
        if line.is_empty() {
            if current_calories > best_sum {
                best_sum = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap_or(0);
        }
    };
    println!("{}", best_sum);
    Some(best_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Optimized solution
    let mut top_calories = [0,0,0];
    let mut current_calories = 0;
    for line in input.lines() {
        // if line is an empty line, reset current_calories
        if line.is_empty() {
            // store the current count in the best,second or third sum
            if current_calories > top_calories[0] {
                top_calories[2] = top_calories[1];
                top_calories[1] = top_calories[0];
                top_calories[0] = current_calories;
            } else if current_calories > top_calories[1] {
                top_calories[2] = top_calories[1];
                top_calories[1] = current_calories;
            } else if current_calories > top_calories[2] {
                top_calories[2] = current_calories;
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap_or(0);
        }
    };
    println!("{:?}",top_calories);
    Some(top_calories[0] + top_calories[1] + top_calories[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(60));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(120));
    }
}
