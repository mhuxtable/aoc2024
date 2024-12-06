use std::{
    cmp,
    collections::HashSet,
};

advent_of_code::solution!(5, Some(5452), Some(4598));

fn parse(
    input: &str,
) -> (
    HashSet<(usize, usize)>,
    impl Iterator<Item = Vec<usize>> + '_,
) {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules: HashSet<(usize, usize)> = rules
        .lines()
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            (before.parse().unwrap(), after.parse().unwrap())
        })
        .collect();

    let pages = pages
        .lines()
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect());

    (rules, pages)
}

fn compare_rules(rules: &HashSet<(usize, usize)>, a: &usize, b: &usize) -> cmp::Ordering {
    if rules.contains(&(*a, *b)) {
        cmp::Ordering::Less
    } else if rules.contains(&(*b, *a)) {
        cmp::Ordering::Greater
    } else {
        cmp::Ordering::Equal
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, manuals) = parse(input);

    manuals
        .filter_map(|manual| {
            if manual.is_sorted_by(|a, b| compare_rules(&rules, a, b).is_le()) {
                Some(manual[manual.len() / 2])
            } else {
                None
            }
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (rules, manuals) = parse(input);

    manuals
        .into_iter()
        .filter_map(|mut manual| {
            if !manual.is_sorted_by(|a, b| compare_rules(&rules, a, b).is_le()) {
                manual.sort_unstable_by(|a, b| compare_rules(&rules, a, b));
                Some(manual[manual.len() / 2])
            } else {
                None
            }

            //let mut did_swap = false;
            //for i in (0..manual.len() - 1).rev() {
            //    for j in i..manual.len() - 1 {
            //        if rules.contains(&(manual[j + 1], manual[j])) {
            //            manual.swap(j, j + 1);
            //            did_swap = true;
            //        }
            //    }
            //}

            //did_swap.then_some(manual[manual.len() / 2])
        })
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
