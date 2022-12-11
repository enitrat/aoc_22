extern crate core;

use std::mem;

#[derive(Clone, Debug)]
enum Operation {
    Add(String),
    Subtract(String),
    Multiply(String),
    Divide(String),
    Modulo(String),
}

impl Operation {
    fn new(op: &str, arg: &str) -> Operation {
        match op {
            "+" => Operation::Add(arg.to_string()),
            "-" => Operation::Subtract(arg.to_string()),
            "*" => Operation::Multiply(arg.to_string()),
            "/" => Operation::Divide(arg.to_string()),
            "%" => Operation::Modulo(arg.to_string()),
            _ => panic!("Unknown operation {}", op),
        }
    }

    fn get_arg(&self) -> Option<u64> {
        match self {
            Operation::Add(arg) => arg.parse::<u64>().ok(),
            Operation::Subtract(arg) => arg.parse::<u64>().ok(),
            Operation::Multiply(arg) => arg.parse::<u64>().ok(),
            Operation::Divide(arg) => arg.parse::<u64>().ok(),
            Operation::Modulo(arg) => arg.parse::<u64>().ok(),
        }
    }

    fn execute(&self, value: u64) -> u64 {
        match &self {
            Operation::Add(arg) => value + arg.parse::<u64>().unwrap_or(value),
            Operation::Subtract(arg) => value - arg.parse::<u64>().unwrap_or(value),
            Operation::Multiply(arg) => value * arg.parse::<u64>().unwrap_or(value),
            Operation::Divide(arg) => value / arg.parse::<u64>().unwrap_or(value),
            Operation::Modulo(arg) => value % arg.parse::<u64>().unwrap_or(value),
            _ => panic!("Unknown operation"),
        }
    }
}

#[derive(Clone, Debug)]
struct Inspection {
    operation: Operation,
    destination_true: u64,
    destination_false: u64,
}

impl Inspection {
    fn new(raw_arg: &str, raw_dest_true: &str, raw_dest_false: &str) -> Inspection {
        let arg = raw_arg.split("by ").collect::<Vec<&str>>()[1].to_string();
        let dest_true = raw_dest_true.split("monkey ").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
        let dest_false = raw_dest_false.split("monkey ").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
        Inspection {
            operation: Operation::Modulo(arg.to_string()),
            destination_true: dest_true,
            destination_false: dest_false,
        }
    }

    fn execute(&self, value: u64) -> u64 {
        if self.operation.execute(value) == 0 {
            self.destination_true
        } else {
            self.destination_false
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    inspection: Inspection,
    count: u64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            items: vec![],
            operation: Operation::Add("1".to_string()),
            inspection: Inspection::new("by 1", "monkey 1", "monkey 1"),
            count: 0,
        }
    }

    fn new_from_text(raw_monkey: &[&str]) -> Monkey {
        let items = raw_monkey[1].split(":").collect::<Vec<&str>>()[1]
            .split(',')
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let raw_operation = raw_monkey[2].split("old ").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .collect::<Vec<&str>>();
        let operation = Operation::new(raw_operation[0], raw_operation[1]);
        let inspection = Inspection::new(raw_monkey[3], raw_monkey[4], raw_monkey[5]);
        Monkey {
            items,
            operation,
            inspection: inspection,
            count: 0,
        }
    }

    fn inspect_elements(&mut self, monkeys: &mut Vec<Monkey>, worry_management: Operation) {
        for item in self.items.drain(..) {
            let raised_worriness = &self.operation.execute(item as u64);
            let new_worry_level = worry_management.execute(*raised_worriness);
            let target_throw = &self.inspection.execute(new_worry_level);
            monkeys.get_mut(*target_throw as usize).unwrap().items.push(new_worry_level as u64);
            self.count += 1;
        }
        self.items.clear();
    }
}

fn parse_initial_state(raw_monkeys: Vec<&[&str]>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for raw_monkey in raw_monkeys {
        monkeys.push(Monkey::new_from_text(raw_monkey));
    }
    monkeys
}


pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let raw_monkeys = lines.chunks(7).collect::<Vec<&[&str]>>();
    let mut monkeys = parse_initial_state(raw_monkeys);
    for j in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = Monkey::new();
            mem::swap(&mut monkeys[i], &mut monkey);
            monkey.inspect_elements(&mut monkeys, Operation::Divide(3.to_string()));
            mem::swap(&mut monkeys[i], &mut monkey);
        }
    }
    monkeys.sort_by(|a, b| a.count.cmp(&b.count));
    let top_2_monkeys = monkeys.iter().rev().take(2).collect::<Vec<&Monkey>>();
    Some(top_2_monkeys[0].count * top_2_monkeys[1].count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let raw_monkeys = lines.chunks(7).collect::<Vec<&[&str]>>();
    let mut monkeys = parse_initial_state(raw_monkeys);
    let common_divisor = monkeys.iter().fold(
        1,
        |acc, monkey| acc * monkey.inspection.operation.get_arg().unwrap()
    );
    for j in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = Monkey::new();
            mem::swap(&mut monkeys[i], &mut monkey);
            monkey.inspect_elements(&mut monkeys, Operation::Modulo(common_divisor.to_string()));
            mem::swap(&mut monkeys[i], &mut monkey);
        }
    }
    monkeys.sort_by(|a, b| a.count.cmp(&b.count));
    let top_2_monkeys = monkeys.iter().rev().take(2).collect::<Vec<&Monkey>>();
    Some(top_2_monkeys[0].count * top_2_monkeys[1].count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
