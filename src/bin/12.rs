use std::collections::HashMap;


#[derive(Clone, Debug)]
struct HeightMap {
    width: usize,
    height: usize,
    data: Vec<Vec<char>>,
    parents: HashMap<Point, Point>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
struct Point {
    x: usize,
    y: usize,
}

struct Dijkstra {
    distance: HashMap<Point, i32>,
    previous: HashMap<Point, Option<Point>>,
    path: Vec<Point>,
}

impl HeightMap {
    fn get(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    fn get_point(&self, point: &Point) -> char {
        self.data[point.y][point.x]
    }

    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();
        let x = point.x;
        let y = point.y;
        if x > 0 {
            neighbors.push(Point { x: x - 1, y });
        }
        if x < self.width - 1 {
            neighbors.push(Point { x: x + 1, y });
        }
        if y > 0 {
            neighbors.push(Point { x, y: y - 1 });
        }
        if y < self.height - 1 {
            neighbors.push(Point { x, y: y + 1 });
        }
        neighbors
    }

    fn get_first_position(&self, c: char) -> Point {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == c {
                    return Point { x, y };
                }
            }
        }
        panic!("Could not find position for char {c}");
    }

    fn get_manhattan_distance(&self, a: &Point, b: &Point) -> i32 {
        ((a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as i32
    }

    fn get_all_positions(&self, c: char) -> Vec<Point> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == c {
                    positions.push(Point { x, y });
                }
            }
        }
        positions
    }

    fn render(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.get(x, y));
            }
            output.push('\n');
        }
        output
    }
}

#[derive(Clone, Debug)]
struct Graph {
    nodes: Vec<Point>,
    edges: HashMap<Point, Vec<Point>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, node1: Point, node2: Point) {
        self.edges.entry(node1).or_insert(Vec::new()).push(node2);
    }

    fn get_neighbors(&self, node: &Point) -> Option<&Vec<Point>> {
        self.edges.get(node)
    }

    fn dijkstra(&self, source: &Point, destination: Option<&Point>) -> Dijkstra {
        let mut distances = HashMap::new();
        let mut previous = HashMap::new();
        let mut nodes = self.nodes.clone();
        let mut path = Vec::new();
        let mut smallest;
        let mut alt;


        // start with D=INF for all nodes, and pi = None
        let binding = nodes.clone();
        for node in binding.iter() {
            distances.insert(*node, i32::MAX);
            previous.insert(*node, None);
        }

        distances.insert(*source, 0);

        while !nodes.is_empty() {
            // find the node with the smallest distance
            nodes.sort_by(|a, b| distances.get(a).unwrap().cmp(distances.get(b).unwrap()));
            smallest = nodes.remove(0);

            // case 1 : destination found -> depile path
            if let Some(dest) = destination {
                if smallest == *dest {
                    path = Vec::new();
                    while previous.get(&smallest).unwrap().is_some() {
                        path.push(smallest);
                        smallest = previous.get(&smallest).unwrap().unwrap();
                    }
                    break;
                }
            }

            // case 2 : no path found
            if distances.get(&smallest).unwrap() == &i32::MAX {
                break;
            }

            let default = Vec::new();
            let neighbors = self.get_neighbors(&smallest).unwrap_or(&default);
            // case 3 : continue and evaluate current distance
            for neighbor in neighbors {
                alt = distances.get(&smallest).unwrap() + 1;
                if alt <= *distances.get(neighbor).unwrap() {

                    distances.insert(*neighbor, alt);
                    previous.insert(*neighbor, Some(smallest));
                }
            }
        }
        path.push(*source);
        path.reverse();
        Dijkstra {
            distance: distances,
            previous,
            path,
        }
    }
}

fn parse_heightmap(input: &str) -> HeightMap {
    let data: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    HeightMap {
        width: data[0].len(),
        height: data.len(),
        data,
        parents: HashMap::new(),
    }
}

fn char_to_num(mut c: char) -> i32 {
    if c == 'S' { c = 'a'; }
    if c == 'E' { c = 'z'; }
    c as i32 - 48
}

fn is_reachable(from: char, to: Option<&char>) -> u8 {
    match to {
        Some(to) => if char_to_num(from) + 1 >= char_to_num(*to) {
            1
        } else {
            0
        },
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let heightmap = parse_heightmap(input);
    let mut graph = Graph::new();
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            let point = Point { x, y };
            let neighbors = heightmap.get_neighbors(&point);
            for neighbor in neighbors {
                if (is_reachable(heightmap.get_point(&point), Some(&heightmap.get_point(&neighbor)))) == 1 {
                    graph.add_edge(point, neighbor);
                }
            }
            graph.nodes.push(point);
        }
    }
    let source = heightmap.get_first_position('S');
    let destination = heightmap.get_first_position('E');
    let dijkstra = graph.dijkstra(&source, Some(&destination));
    Some((dijkstra.path.len() - 1) as u32)
    // println!("{:?}", graph);
}

pub fn part_two(input: &str) -> Option<i32> {
    let heightmap = parse_heightmap(input);
    let mut graph = Graph::new();
    for y in 0..heightmap.height {
        for x in 0..heightmap.width {
            let point = Point { x, y };
            let neighbors = heightmap.get_neighbors(&point);
            for neighbor in neighbors {
                if is_reachable(heightmap.get_point(&neighbor), Some(&heightmap.get_point(&point))) == 1 {
                    graph.add_edge(point, neighbor);
                }
            }
            graph.nodes.push(point);
        }
    }
    let source = heightmap.get_first_position('E');
    let lowest_points = heightmap.get_all_positions('a');
    let dijkstra = graph.dijkstra(&source, None);
    let min_distance = *lowest_points.iter().map(|point| dijkstra.distance.get(point).unwrap()).min().unwrap();
    Some(min_distance)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
