#[derive(Clone, Copy, Debug)]
enum Command {
    Noop,
    Addx(isize),
}

#[derive(Clone, Copy, Debug)]
struct Cpu {
    cycle: isize,
    value: isize,
    current_command: Option<(Command, u8)>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            cycle: 1,
            value: 1,
            current_command: None,
        }
    }

    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Noop => self.current_command = Some((Command::Noop, 1)),
            Command::Addx(x) => {
                self.current_command = Some((Command::Addx(x), 2));
            }
        }
    }

    fn tick(&mut self) {
        if let Some((command, remaining_cycles)) = &self.current_command {
            self.cycle += 1;
            match remaining_cycles - 1 {
                0 => {
                    match command {
                        Command::Noop => {}
                        Command::Addx(x) => self.value += x,
                    }
                    self.current_command = None;
                }
                _ => {
                    self.current_command = Some((*command, remaining_cycles - 1));
                }
            }
        }
    }

    fn get_signal_strength(&self) -> isize {
        self.value * self.cycle
    }

    fn render_pixels(&self) -> &str {
        match (((self.cycle-1) % 40)-self.value).abs() {
            0 | 1 => "#",
            _ => "."
        }
    }

    fn get_distance(x1:isize, y1:isize){
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut cpu = Cpu::new();
    let mut signal_strength_sum = 0;
    for line in input.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        let (command, argument) = (instruction.first(), instruction.get(1));
        let command = match command {
            Some(&"noop") => Command::Noop,
            Some(&"addx") => Command::Addx(argument.unwrap().parse().unwrap()),
            _ => panic!("Invalid command"),
        };
        cpu.execute_command(command);
        while cpu.current_command.is_some() {
            if (cpu.cycle + 20) % 40 == 0 {
                signal_strength_sum += cpu.get_signal_strength();
            }
            cpu.tick();
        }

    }
    Some(signal_strength_sum)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut cpu = Cpu::new();
    let mut signal_strength_sum = 0;
    for line in input.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        let (command, argument) = (instruction.first(), instruction.get(1));
        let command = match command {
            Some(&"noop") => Command::Noop,
            Some(&"addx") => Command::Addx(argument.unwrap().parse().unwrap()),
            _ => panic!("Invalid command"),
        };
        cpu.execute_command(command);
        while cpu.current_command.is_some() {
            print!("{}", cpu.render_pixels());
            if (cpu.cycle + 40) % 40 == 0 {
                print!("\n");
            }
            cpu.tick();
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
