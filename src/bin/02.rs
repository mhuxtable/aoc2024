advent_of_code::solution!(2, Some(559), Some(601));

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn matcher(
    direction: Option<std::cmp::Ordering>,
    (a, b): (&u32, &u32),
) -> Option<std::cmp::Ordering> {
    match (1..=3).contains(&a.abs_diff(*b)) {
        true => {
            let cmp = a.cmp(b);
            match direction {
                None => Some(cmp),
                Some(ord) if ord == cmp => Some(cmp),
                Some(_) => None,
            }
        }
        false => None,
    }
}

fn valid_report(report: &[u32]) -> bool {
    report
        .iter()
        .zip(report.iter().skip(1))
        .try_fold(None, |direction, (a, b)| {
            let result = matcher(direction, (a, b));
            match result {
                Some(ord) => Ok(Some(ord)),
                None => Err(()),
            }
        })
        .is_ok()
}

pub fn part_one(input: &str) -> Option<usize> {
    let reports = parse(input);

    reports
        .iter()
        .filter(|report| valid_report(report))
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let reports = parse(input);

    let mut count = 0;
    count += reports.iter().filter(|report| valid_report(report)).count();
    assert_eq!(count, part_one(input)?);

    let invalid_reports = reports.iter().filter(|report| !valid_report(report));

    for report in invalid_reports {
        for i in 0..report.len() {
            let mut new = report.clone()[..i].to_vec();
            new.extend(&report[i + 1..]);

            if valid_report(&new) {
                count += 1;
                break;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
