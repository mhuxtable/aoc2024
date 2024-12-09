use advent_of_code::grid::{Grid, NeighbourTypes};

advent_of_code::solution!(4, Some(2514), Some(1888));

static SEARCH: &str = "XMAS";

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::<char>::parse(input).expect("parsing grid");

    let mut xmas = 0;

    let mut candidates = Vec::new();
    let first = SEARCH.chars().next().unwrap();

    for (pos, value) in grid.iter() {
        if value == first {
            candidates.push(pos);
        }
    }

    while let Some(pos) = candidates.pop() {
        'neighbour: for (npos, nval) in
            grid.neighbours_iter(&pos, NeighbourTypes::BasisSet | NeighbourTypes::Diagonals)
        {
            if *nval != SEARCH.chars().nth(1).unwrap() {
                continue;
            }

            let (dx, dy) = (
                npos.0 as isize - pos.0 as isize,
                npos.1 as isize - pos.1 as isize,
            );

            for i in 2..SEARCH.len() {
                let (nx, ny) = (
                    pos.0 as isize + (dx * i as isize),
                    pos.1 as isize + (dy * i as isize),
                );

                if nx < 0 || ny < 0 || nx >= grid.width as isize || ny >= grid.height as isize {
                    continue 'neighbour;
                }

                let v = grid.get(nx.try_into().unwrap(), ny.try_into().unwrap());
                if v.is_none() || *v.unwrap() != SEARCH.chars().nth(i).unwrap() {
                    continue 'neighbour;
                }
            }

            xmas += 1;
        }
    }

    Some(xmas)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::<char>::parse(input).expect("parsing grid");

    let mut mas = 0;

    let mut candidates = Vec::new();

    for (pos, value) in grid.iter() {
        if value == 'A' {
            candidates.push(pos);
        }
    }

    'candidate: while let Some((x, y)) = candidates.pop() {
        for diagonal in [1, -1] {
            let up = (x as isize + diagonal, y as isize - 1);
            let down = (x as isize - diagonal, y as isize + 1);

            if up.0 < 0 || up.1 < 0 || up.0 >= grid.width as isize || up.1 >= grid.height as isize {
                continue 'candidate;
            }
            if down.0 < 0
                || down.1 < 0
                || down.0 >= grid.width as isize
                || down.1 >= grid.height as isize
            {
                continue 'candidate;
            }

            let search = match grid.get(up.0.try_into().unwrap(), up.1.try_into().unwrap()) {
                Some('M') => Some('S'),
                Some('S') => Some('M'),
                _ => continue 'candidate,
            };

            if grid.get(down.0.try_into().unwrap(), down.1.try_into().unwrap()) != search.as_ref() {
                continue 'candidate;
            }
        }

        mas += 1;
    }

    Some(mas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
