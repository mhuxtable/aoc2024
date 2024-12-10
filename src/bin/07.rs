advent_of_code::solution!(7, Some(4555081946288), Some(227921760109726));

use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Multiply,
    Concatenate,
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            let (solution, inputs) = line.split_once(": ").unwrap();
            let inputs = inputs
                .split(" ")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<u64>>();

            (solution.parse().unwrap(), inputs)
        })
        .filter_map(|(soln, ins)| {
            (0..ins.len() - 1)
                .map(|_| [Op::Add, Op::Multiply].iter())
                .multi_cartesian_product()
                .any(|ops| {
                    let mut i = 1;
                    let mut result = ins[0];

                    for op in ops {
                        match op {
                            Op::Add => result += ins[i],
                            Op::Multiply => result *= ins[i],
                            _ => panic!("Invalid operation"),
                        }

                        i += 1;
                    }

                    result == soln
                })
                .then_some(soln)
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            let (solution, inputs) = line.split_once(": ").unwrap();
            let inputs = inputs
                .split(" ")
                .map(|i| i.parse().unwrap())
                .collect::<Vec<u64>>();

            (solution.parse().unwrap(), inputs)
        })
        .filter_map(|(soln, ins)| {
            (0..ins.len() - 1)
                .map(|_| [Op::Add, Op::Multiply, Op::Concatenate].iter())
                .multi_cartesian_product()
                .any(|ops| {
                    let mut i = 1;
                    let mut result = ins[0];

                    for op in ops {
                        match op {
                            Op::Add => result += ins[i],
                            Op::Multiply => result *= ins[i],
                            Op::Concatenate => {
                                let mut digits = 0;
                                let mut pow = 1;
                                while ins[i] / pow > 0 {
                                    pow *= 10;
                                    digits += 1;
                                }

                                result = result
                                    .checked_mul(10u64.pow(digits as u32))
                                    .unwrap()
                                    .checked_add(ins[i])
                                    .unwrap();
                            }
                        }

                        i += 1;
                    }

                    result == soln
                })
                .then_some(soln)
        })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
