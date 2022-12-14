use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq)]
enum State {
    Resting,
    Moving,
}

enum Move {
    Down,
    DownLeft,
    DownRight,
}

impl Move {
    fn get_next_position(&self, origin: &Point) -> Point {
        let mut next = origin.clone();
        match self {
            Move::Down => next.y += 1,
            Move::DownLeft => {
                next.x -= 1;
                next.y += 1
            }
            Move::DownRight => {
                next.x += 1;
                next.y += 1
            }
        }
        next
    }
}

fn get_next_position(mut sand: Point, rocks: &HashSet<Point>) -> Point {
    let mut next_position = Move::Down.get_next_position(&sand);

    if rocks.contains(&next_position) {
        let left = Move::DownLeft.get_next_position(&sand);
        let right = Move::DownRight.get_next_position(&sand);
        if rocks.contains(&left) {
            next_position = right;
        } else {
            next_position = left;
        }
    }
    if rocks.contains(&next_position) {
        next_position = sand;
    }
    next_position
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn create_line(source: &Point, dest: &Point) -> Vec<Point> {
    let mut line = Vec::new();
    let mut current_point_x = source.clone();
    let mut current_point_y = source.clone();
    line.push(current_point_x.clone());
    let dx = dest.x - source.x;
    let dy = dest.y - source.y;

    while current_point_x.x != dest.x {
        current_point_x.x += dx.signum();
        line.push(current_point_x.clone());
    }
    while current_point_y.y != dest.y {
        current_point_y.y += dy.signum();
        line.push(current_point_y.clone());
    }
    line
}

fn parse_input(input: &str) -> HashSet<Point> {
    println!("Parsing input");
    let mut rocks: Vec<Point> = Vec::new();
    for line in input.lines() {
        let mut straight_lines = line.split(" -> ").collect::<VecDeque<&str>>();
        let mut begin = straight_lines.pop_front().unwrap();
        while straight_lines.len() > 0 {
            let end = straight_lines.pop_front().unwrap_or(begin);
            let source = Point {
                x: begin.split(",").collect::<Vec<&str>>()[0].parse().unwrap(),
                y: begin.split(",").collect::<Vec<&str>>()[1].parse().unwrap(),
            };
            let dest = Point {
                x: end.split(",").collect::<Vec<&str>>()[0].parse().unwrap(),
                y: end.split(",").collect::<Vec<&str>>()[1].parse().unwrap(),
            };
            rocks.append(&mut create_line(&source, &dest));
            begin = end;
        }
    }
    let unique_rocks: HashSet<Point> = HashSet::from_iter(rocks);
    unique_rocks
}

fn pour_sand(mut rocks: HashSet<Point>) -> u64 {
    const ORIGIN: Point = Point { x: 500, y: 0 };
    let mut counter: u64 = 0;

    loop {
        let mut current_point = ORIGIN.clone();
        let mut current_state = State::Moving;
        let mut inf_loop_detector = 0;

        while current_state == State::Moving && inf_loop_detector < 1500 {
            let next_position = get_next_position(current_point.clone(), &rocks);
            if next_position == current_point {
                inf_loop_detector = 0;
                current_state = State::Resting;
                rocks.insert(next_position.clone());
                counter += 1;
            } else {
                inf_loop_detector += 1;
            }
            current_point = next_position;
        }
        if inf_loop_detector == 1500 {
            break;
        }
    }
    counter
}

fn display_rocks(rocks: &HashSet<Point>) {
    let mut ordered_rocks_y = rocks.iter().collect::<Vec<&Point>>();
    ordered_rocks_y.sort_by(|a, b| a.y.cmp(&b.y));
    let bottom = ordered_rocks_y.last().unwrap().y;
    println!(" Bottom {}", bottom);

    let mut order_rocks_x = rocks.iter().collect::<Vec<&Point>>();
    order_rocks_x.sort_by(|a, b| a.x.cmp(&b.x));
    let left = order_rocks_x.first().unwrap().x - 2;
    let right = order_rocks_x.last().unwrap().x + 2;
    for y in 0..bottom+1 {
        for x in left-1..right+1 {
            let point = Point { x, y };
            if rocks.contains(&point) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn pour_sand_part_2(mut rocks: HashSet<Point>) -> u64 {
    const ORIGIN: Point = Point { x: 500, y: 0 };
    let mut counter: u64 = 0;

    // find bottom location
    let mut ordered_rocks_y = rocks.iter().collect::<Vec<&Point>>();
    ordered_rocks_y.sort_by(|a, b| a.y.cmp(&b.y));
    let bottom = ordered_rocks_y.last().unwrap().y + 2;

    let steps = loop {

        //spawn a new sand
        let mut current_point = ORIGIN.clone();
        let mut current_state = State::Moving;

        while current_state == State::Moving {
            // get the next sand position. Either same, down, downleft or downright
            let next_position = get_next_position(current_point.clone(), &rocks);

            // case 1: the sand is still at spawn point
            if next_position.y == 0 {
                current_state = State::Resting;
                rocks.insert(next_position.clone());
                counter += 1;

                // we're done and return the amount of resting sands
                return counter;
            }

            // case 2: the sand is just above the bottom and can't go down any further
            if next_position.y == bottom {
                current_state = State::Resting;
                rocks.insert(current_point.clone());
                counter += 1;
                break;
            //case 3: the sand can't go down anymore
            } else if next_position == current_point {
                current_state = State::Resting;
                rocks.insert(next_position.clone());
                counter += 1;
            }

            // the sand can go down, so we move it
            current_point = next_position;
        }
    };
    steps
}


pub fn part_one(input: &str) -> Option<u64> {
    let rocks = parse_input(input);
    Some(pour_sand(rocks))
}

pub fn part_two(input: &str) -> Option<u64> {
    let rocks = parse_input(input);

    Some(pour_sand_part_2(rocks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
