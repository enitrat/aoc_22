
use std::ops::{Range};
use regex::Regex;
use itertools::Itertools;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Sensor {
    coords: Point,
    beacon: Point,
    beacon_distance: i64,
}

impl Sensor {
    fn coverage_at_line_y(&self, y: i64) -> Option<Range<i64>> {
        let distance_from_sensor = (y - self.coords.y).abs(); // distance from sensor to line
        let half_x_range = self.beacon_distance - distance_from_sensor; // half of the x range of the line y
        // - the further the sensor from the line, the smaller the x range
        // because we use manhattan distance, every `y` distance from sensor reduces the width range by 1
        match half_x_range {
            _ if half_x_range < 0 => None, //Doesnt cover line at all
            _ => Some((self.coords.x - half_x_range)..(self.coords.x + half_x_range + 1)), //Covers a certain range [x - half_x_range, x + half_x_range]
        }
    }
}

impl Point {

    fn manhattan_distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}


fn coverage_unions(mut ranges: Vec<Range<i64>>) -> (i64, Vec<Range<i64>>) {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut unions = Vec::new();
    let mut current_union = ranges[0].clone();
    for range in ranges {
        if range.start <= current_union.end {
            current_union.end = range.end.max(current_union.end);
        } else {
            unions.push(current_union);
            current_union = range;
        }
    }
    unions.push(current_union);
    let area = unions.iter().fold(0, |acc, range| acc + range.end - range.start);
    (area, unions)
}

fn solve_one(input: &str, row: i64) -> Option<i64> {
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in input.lines() {
        let pattern = Regex::new(r"x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let captures = pattern.captures(line).unwrap();
        let current_sensor = Point {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        };

        let closest_beacon = Point {
            x: captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap(),
        };

        let distance = current_sensor.manhattan_distance(&closest_beacon);
        let sensor_data = Sensor {
            coords: current_sensor,
            beacon: closest_beacon,
            beacon_distance: distance,
        };
        sensors.push(sensor_data.clone());
    }

    let coverage: Vec<Range<i64>> = sensors.iter()
        .filter_map(|sensor| sensor.coverage_at_line_y(row))
        .collect();


    let (area, _) = coverage_unions(coverage);
    let beacons_in_row: i64 = sensors.iter().filter(|sensor| sensor.beacon.y == row).map(|sensor| sensor.beacon.x).dedup().count() as i64;
    Some(area - beacons_in_row)
}

fn solve_two(input: &str, scan_range: Range<i64>) -> Option<i64> {
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in input.lines() {
        let pattern = Regex::new(r"x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        let captures = pattern.captures(line).unwrap();
        let current_sensor = Point {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
        };

        let closest_beacon = Point {
            x: captures[3].parse().unwrap(),
            y: captures[4].parse().unwrap(),
        };

        let distance = current_sensor.manhattan_distance(&closest_beacon);
        let sensor_data = Sensor {
            coords: current_sensor,
            beacon: closest_beacon,
            beacon_distance: distance,
        };
        sensors.push(sensor_data.clone());
    }


    let possible_points: Vec<Option<Point>> = scan_range.clone().map(|row| {

        // get the coverage of each sensor
        let coverage: Vec<Range<i64>> = sensors.iter()
            .filter_map(|sensor| sensor.coverage_at_line_y(row))
            .collect_vec();

        // get the unions of the coverages.
        let (_, unions) = coverage_unions(coverage);

        // if the whole line is covered by a single union then there's no room for an extra beacon
        if unions.len()<2{
            return None;
        }

        //otherwise, there is a gap in the coverage, and we can place a beacon there
        unions.iter()
            // remove all unions who are not in the scanned range
            .filter(|u| u.start <= scan_range.end && u.end >= scan_range.start)
            // sort unions to have the lowest start first
            .sorted_by(|a, b| a.start.cmp(&b.start))
            // creat tuples of (start, end) of the unions
            .tuple_windows()
            // find the first union that has a gap between it and the next union
            .find(|(a, b)| a.end != b.start)
            // take the end of the first union as a possible point
            .map(|(a, _)| Point { x: a.end, y: row })
        // only keep the Some values
    }).filter(|x| x.is_some()).collect_vec();
    match possible_points {
        // shouldn't happen if the problem is well formed
        _ if possible_points.is_empty() => None,
        _ => {
            // the first point should be the solution :))
            let target_point = possible_points[0].clone().unwrap();
            Some(target_point.x * 4000000 + target_point.y)
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    solve_one(input, 2000000)
}

pub fn part_one_test(input: &str) -> Option<i64> {
    solve_one(input, 10)
}

pub fn part_two(input: &str) -> Option<i64> {
    let scan_range: Range<i64> = 0..(4000000 + 1);
    solve_two(input, scan_range)
}

pub fn part_two_test(input: &str) -> Option<i64> {
    let scan_range: Range<i64> = 0..20 + 1;
    solve_two(input, scan_range)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_test(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_test(&input), Some(56000011));
    }
}
