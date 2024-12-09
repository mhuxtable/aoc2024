use std::collections::HashMap;

advent_of_code::solution!(1, Some(1110981), Some(24869388));

fn parse_list(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>()
        .into_iter()
        .unzip()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut l1, mut l2) = parse_list(input);

    l1.sort();
    l2.sort();

    l1.iter()
        .zip(l2.iter())
        .map(|(a, &b)| a.abs_diff(b))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (l1, l2) = parse_list(input);

    let mut map = HashMap::new();
    for item in l2.iter() {
        map.entry(item).and_modify(|e| *e += 1).or_insert(1);
    }

    l1.iter()
        .map(|item| item * map.get(item).unwrap_or(&0))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
