advent_of_code::solution!(3, Some(156388521), Some(75920122));

#[derive(Copy, Clone, Debug)]
enum Token {
    Char(char),
    OpenBracket,
    Digit,
    Comma,
    CloseBracket,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Op {
    Mul,
    Do,
    Dont,
}

use Op::*;
use Token::*;

fn solve(input: &str, conditionals: bool) -> Option<u32> {
    let mut token = None;
    let mut sum = 0;
    let (mut left, mut right) = (None, None);

    let mut enable = true;
    let mut op = None;

    for c in input.chars() {
        match token {
            _ if c == 'm' => {
                token = Some(Token::Char(c));
                op = None;
                left = None;
                right = None;
            }
            _ if c == 'd' => {
                token = Some(Token::Char(c));
                op = None;
                left = None;
                right = None;
            }
            Some(Char('d')) if c == 'o' => token = Some(Token::Char(c)),
            Some(Char('o')) if c == '(' => {
                token = Some(Token::OpenBracket);
                op = Some(Do);
            }
            Some(Char('o')) if c == 'n' => token = Some(Token::Char(c)),
            Some(Char('n')) if c == '\'' => token = Some(Token::Char(c)),
            Some(Char('\'')) if c == 't' => token = Some(Token::Char(c)),
            Some(Char('t')) if c == '(' => {
                token = Some(Token::OpenBracket);
                op = Some(Dont);
            }

            Some(Char('m')) if c == 'u' => token = Some(Token::Char(c)),
            Some(Char('u')) if c == 'l' => token = Some(Token::Char(c)),
            Some(Char('l')) if c == '(' => {
                token = Some(Token::OpenBracket);
                op = Some(Mul);
            }

            Some(OpenBracket) if c.is_ascii_digit() => {
                left = Some(c.to_digit(10).unwrap());
                token = Some(Token::Digit);
            }
            Some(Digit) if c == ',' => token = Some(Token::Comma),
            Some(Digit) if c.is_ascii_digit() => {
                if right.is_none() {
                    left = Some(left.unwrap() * 10 + c.to_digit(10).unwrap());
                } else {
                    right = Some(right.unwrap() * 10 + c.to_digit(10).unwrap());
                }
            }
            Some(Comma) if c.is_ascii_digit() => {
                right = Some(c.to_digit(10).unwrap());
                token = Some(Token::Digit);
            }
            Some(Digit) if c == ')' => token = Some(Token::CloseBracket),
            Some(OpenBracket) if c == ')' => {
                if op.unwrap() == Do || op.unwrap() == Dont {
                    token = Some(Token::CloseBracket);
                }
            }
            _ => {
                if token.is_some() {
                    //println!("Unexpected token: {} in state {:?}", c, token);
                }
                token = None;
                left = None;
                right = None;
                op = None;
            }
        };

        if let Some(CloseBracket) = token {
            if let (Some(Mul), true) = (op, enable) {
                sum += left.unwrap() * right.unwrap();
                token = None;
                left = None;
                right = None;
            } else if conditionals {
                if let Some(Do) = op {
                    enable = true;
                } else if let Some(Dont) = op {
                    enable = false;
                }
            }
        }
    }

    Some(sum)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
