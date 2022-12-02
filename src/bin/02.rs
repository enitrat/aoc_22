use std::collections::HashMap;
use std::fmt::{Display, Formatter};


enum Round {
    Win,
    Lose,
    Draw,
}

impl Display for Round {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        let description = match *self {
            Round::Win => "Won",
            Round::Lose => "Lost",
            _ => "Draw"
        };
        f.write_str(description)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut shapes_codes = HashMap::new();
    shapes_codes.insert("A", "Rock");
    shapes_codes.insert("X", "Rock");
    shapes_codes.insert("B", "Paper");
    shapes_codes.insert("Y", "Paper");
    shapes_codes.insert("C", "Scissors");
    shapes_codes.insert("Z", "Scissors");
    let mut shapes_points = HashMap::new();
    shapes_points.insert("Rock", 1);
    shapes_points.insert("Paper", 2);
    shapes_points.insert("Scissors", 3);

    let mut shapes_counters = HashMap::new();
    shapes_counters.insert("Rock", "Paper");
    shapes_counters.insert("Paper", "Scissors");
    shapes_counters.insert("Scissors", "Rock");

    let mut total_score: u32 = 0;

    for line in input.lines() {
        let round = line.split(" ").collect::<Vec<&str>>();
        let opponent = shapes_codes.get(round[0]).unwrap();
        let me = shapes_codes.get(round[1]).unwrap();
        let round_result = match shapes_counters.get(opponent).unwrap() {
            &x if &x == me => Round::Win,
            &x if (me == opponent) => Round::Draw,
            _ => Round::Lose
        };

        match round_result {
            Round::Win => total_score += (6 + shapes_points.get(me).unwrap()),
            Round::Draw => total_score += (3 + shapes_points.get(me).unwrap()),
            Round::Lose => total_score += (0 + shapes_points.get(me).unwrap()),
        }
        println!("Opponent played {} and you played {}. Result : {}",opponent,me,round_result );
    }
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut shapes_codes = HashMap::new();
    shapes_codes.insert("A", "Rock");
    shapes_codes.insert("X", "Rock");
    shapes_codes.insert("B", "Paper");
    shapes_codes.insert("Y", "Paper");
    shapes_codes.insert("C", "Scissors");
    shapes_codes.insert("Z", "Scissors");

    let mut shapes_points = HashMap::new();
    shapes_points.insert("Rock", 1);
    shapes_points.insert("Paper", 2);
    shapes_points.insert("Scissors", 3);

    let mut shapes_counters = HashMap::new();
    shapes_counters.insert("Rock", "Paper");
    shapes_counters.insert("Paper", "Scissors");
    shapes_counters.insert("Scissors", "Rock");

    let mut shape_losers = HashMap::new();
    shape_losers.insert("Paper", "Rock");
    shape_losers.insert("Rock", "Scissors");
    shape_losers.insert("Scissors", "Paper");

    let mut total_score: u32 = 0;

    for line in input.lines() {
        let round = line.split(" ").collect::<Vec<&str>>();
        let opponent = shapes_codes.get(round[0]).unwrap();
        let counter = shapes_counters.get(opponent).unwrap();
        let result_expected = match round[1] {
            x if x == "X" => Round::Lose,
            x if x == "Y" => Round::Draw,
            _ => Round::Win
        };

        match result_expected {
            Round::Win => { total_score += (6 + shapes_points.get(counter).unwrap()) }
            Round::Draw => total_score += (3 + shapes_points.get(opponent).unwrap()),
            Round::Lose => total_score += (0 + shapes_points.get(shape_losers.get(opponent).unwrap()).unwrap()),
        }
    }
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
