use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5, Some(5452), None);

struct Rule(usize, usize);

fn parse(input: &str) -> (Vec<Rule>, Vec<Vec<usize>>) {
    let rules: Vec<Rule> = input
        .lines()
        .clone()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
            let (before, after) = (before.parse().unwrap(), after.parse().unwrap());

            Rule(before, after)
        })
        .collect();

    let pages: Vec<Vec<usize>> = input
        .lines()
        .skip(rules.len() + 1)
        .map(|line| line.split(',').map(|page| page.parse().unwrap()).collect())
        .collect();

    (rules, pages)
}

fn filter_valid_order(seen: &mut HashSet<usize>, rules_map: &HashMap<usize, HashSet<usize>>, manual: &[usize]) -> bool {
    seen.clear();

    for page in manual {
        if let Some(set) = rules_map.get(page) {
            if !set.is_disjoint(seen) {
                return false;
            }
        }

        seen.insert(*page);
    }

    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, manuals) = parse(input);

    let mut rules_map = HashMap::new();
    let mut all_pages = HashSet::new();

    for rule in rules {
        rules_map
            .entry(rule.0)
            .or_insert_with(HashSet::new)
            .insert(rule.1);
        all_pages.extend([rule.0, rule.1]);
    }

    let mut seen = HashSet::with_capacity(all_pages.len());

    manuals
        .iter()
        .filter(|&manual| {
            seen.clear();

            for page in manual {
                if let Some(set) = rules_map.get(page) {
                    if !set.is_disjoint(&seen) {
                        return false;
                    }
                }

                seen.insert(*page);
            }

            true
        })
        .map(|manual| manual[manual.len() / 2])
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
