extern crate core;

use std::cmp::Ordering::{Greater, Less, Equal};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
enum Token {
    LBracket,
    RBracket,
    Number(usize),
}

fn parser(string: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    //holds multiple digit numbers
    let mut temp_number = String::new();
    for c in string.chars() {
        // if we encounter an opening bracket, we need to get what's inside the brackets
        // if we encounter a closing bracket, it means that we're done with the current number
        // and we can push it to the tokens
        // same if we encounter a comma
        match c {
            '[' => tokens.push_back(Token::LBracket),
            ']' => {
                if temp_number.len() > 0 {
                    tokens.push_back(Token::Number(temp_number.parse().unwrap()));
                }
                temp_number.clear();
                tokens.push_back(Token::RBracket);
            }
            ',' => {
                if temp_number.len() > 0 {
                    tokens.push_back(Token::Number(temp_number.parse().unwrap()));
                }
                temp_number.clear()
            }
            _ => temp_number.push(c)
        }
    }
    tokens
}


fn compare_packets(left: &str, right: &str) -> std::cmp::Ordering {
    let mut left_tokens = parser(left);
    let mut right_tokens = parser(right);

    loop {
        let l_token = left_tokens.pop_front();
        let r_token = right_tokens.pop_front();

        if l_token.is_none() {
            return Less;
        }

        if r_token.is_none() {
            return Greater;
        }

        let (l_token, r_token) = (l_token.unwrap(), r_token.unwrap());
        match (l_token.clone(), r_token.clone()) {
            (Token::LBracket, Token::LBracket) => (),
            (Token::RBracket, Token::RBracket) => (),

            // One token is the beginning of an inner array, the other is a number
            // => Wrap the number inside a size one array
            (Token::LBracket, Token::Number(_)) => {
                right_tokens.push_front(Token::RBracket);
                right_tokens.push_front(r_token.clone());
            }
            (Token::Number(_), Token::LBracket) => {
                left_tokens.push_front(Token::RBracket);
                left_tokens.push_front(l_token.clone());
            }
            //Both tokens are numbers - basic comparison
            (Token::Number(l), Token::Number(r)) => {
                match l {
                    x if x > r => {
                        return Greater;
                    }
                    x if r > x => {
                        return Less;
                    }
                    _ => {
                        continue;
                    }
                }
            }
            (Token::RBracket, _) => { return Less; }
            (_, Token::RBracket) => { return Greater; }
            _ => panic!("unknown pair {:?} {:?}", l_token, r_token)
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let packets = input.lines().filter(|line| !line.is_empty()).collect::<Vec<&str>>();
    let sum_correct_indexes: usize = packets.chunks(2)
        .enumerate()
        .filter(|(_, p)| compare_packets(p[0], p[1]) == Less)
        .map(|(i, _)| i + 1)
        .sum();

    Some(sum_correct_indexes as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets = input.lines().filter(|line| !line.is_empty()).collect::<Vec<&str>>();
    const DIVIDER_1: &str = "[[2]]";
    const DIVIDER_2: &str = "[[6]]";
    packets.push(DIVIDER_1);
    packets.push(DIVIDER_2);
    packets.sort_by(|l, r| compare_packets(l, r));


    let location_divider_1 = packets.iter().position(|r| *r == DIVIDER_1).unwrap() + 1;
    let location_divider_2 = packets.iter().position(|r| *r == DIVIDER_2).unwrap() + 1;

    Some((location_divider_1 * location_divider_2) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
