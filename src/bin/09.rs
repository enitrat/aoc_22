use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Move {
    Up,
    Down,
    Right,
    Left,
    DiagUpRight,
    DiagUpLeft,
    DiagDownRight,
    DiagDownLeft,
}

impl Move {
    fn get_new_point(&self, point: Point) -> Point {
        match self {
            Move::Up => Point::new(point.x, point.y - 1),
            Move::Down => Point::new(point.x, point.y + 1),
            Move::Right => Point::new(point.x + 1, point.y),
            Move::Left => Point::new(point.x - 1, point.y),
            Move::DiagUpRight => Point::new(point.x + 1, point.y - 1),
            Move::DiagUpLeft => Point::new(point.x - 1, point.y - 1),
            Move::DiagDownRight => Point::new(point.x + 1, point.y + 1),
            Move::DiagDownLeft => Point::new(point.x - 1, point.y + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Grid {
    rope: Vec<Point>,
    visited: HashSet<Point>,
}

impl Grid {
    fn new() -> Grid {
        let mut grid = Grid {
            rope: Vec::new(),
            visited: HashSet::new(),
        };
        grid.visited.insert(Point { x: 0, y: 0 });
        grid
    }

    fn move_knots(&mut self, i: usize, move_dir: Move) {
        let curr_knot_pos = self.rope[i];
        let new_knot_pos = Move::get_new_point(&move_dir, curr_knot_pos);
        self.rope[i] = new_knot_pos;

        if i == self.rope.len() - 1 { // stop condition : last knot moved
            self.visited.insert(new_knot_pos);
            return;
        }
        let new_neighbor_distance = self.custom_distance_points(new_knot_pos, self.rope[i + 1]);
        if new_neighbor_distance > 1 {

            // Two cases :
            // - Either on same row or same column, move in that direction
            // - Or move diagonally next to the head (which is actually two moves on x and y)

            let dx = new_knot_pos.x - self.rope[i + 1].x;
            let dy = new_knot_pos.y - self.rope[i + 1].y;

            match (dx, dy) {
                (0, _) => { // same column
                    let move_dir = if dy > 0 { Move::Down } else { Move::Up };
                    self.move_knots(i + 1, move_dir);
                }
                (_, 0) => { // same row
                    let move_dir = if dx > 0 { Move::Right } else { Move::Left };
                    self.move_knots(i + 1, move_dir);
                }
                _ => { // diagonal
                    let move_x = if dx > 0 { Move::Right } else { Move::Left };
                    let move_y = if dy > 0 { Move::Down } else { Move::Up };
                    match (move_x, move_y) {
                        (Move::Right, Move::Down) => {
                            self.move_knots(i + 1, Move::DiagDownRight);
                        }
                        (Move::Right, Move::Up) => {
                            self.move_knots(i + 1, Move::DiagUpRight);
                        }
                        (Move::Left, Move::Down) => {
                            self.move_knots(i + 1, Move::DiagDownLeft);
                        }
                        (Move::Left, Move::Up) => {
                            self.move_knots(i + 1, Move::DiagUpLeft);
                        }
                        _ => panic!("Should not happen"),
                    }
                }
            }
        }
    }

    // returns 1 if it's a diagonal neighbor, manhattan distance otherwise
    fn custom_distance_points(&self, p1: Point, p2: Point) -> u8 {
        let Dx = (p1.x - p2.x).abs();
        let Dy = (p1.y - p2.y).abs();

        // check diagonals
        if Dx == 1 && Dy == 1 {
            return 1;
        }
        // manhattan distance
        (Dx + Dy) as u8
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new();
    grid.rope.push(Point { x: 0, y: 0 });
    grid.rope.push(Point { x: 0, y: 0 });
    run_algo(input, grid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = Grid::new();
    for i in 0..10 {
        grid.rope.push(Point { x: 0, y: 0 });
    }
    run_algo(input, grid)
}

fn run_algo(input: &str, mut grid: Grid) -> Option<u32> {
    let lines = input.lines();
    for line in lines {
        let instructions = line.split_whitespace().collect::<Vec<&str>>();
        let move_dir = match instructions[0] {
            "U" => Move::Up,
            "D" => Move::Down,
            "L" => Move::Left,
            "R" => Move::Right,
            _ => panic!("Invalid direction"),
        };
        for _ in 0..instructions[1].parse().unwrap_or(0) {
            grid.move_knots(0, move_dir);
        }
    }
    Some(grid.visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
