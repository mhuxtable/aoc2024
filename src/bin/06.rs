use std::collections::HashSet;
use std::iter::Iterator;

advent_of_code::solution!(6, Some(5269), Some(1957));

#[derive(Clone)]
struct Guard {
    position: (usize, usize),
    direction: usize, // 0 = north, 1 = east, 2 = south, 3 = west
}

fn parse(input: &str) -> ((usize, usize), Guard, HashSet<(usize, usize)>) {
    let mut guard = Guard {
        position: (0, 0),
        direction: 0, // guard always starts facing north?
    };

    let mut obstacles = HashSet::new();
    let (mut width, mut height) = (0, 0);

    for (y, line) in input.lines().enumerate() {
        height = y + 1;
        width = line.len();

        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((x, y));
                }
                '^' => {
                    guard.position = (x, y);
                }
                _ => {}
            }
        }
    }

    ((width, height), guard, obstacles)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (dims, guard, obstacles) = parse(input);
    guard_path(dims, &guard, &obstacles).len().into()
}

fn guard_path(
    dims: (usize, usize),
    guard: &Guard,
    obstacles: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut path = HashSet::new();
    let mut local_guard = guard.clone();

    loop {
        let (result, next, ndir) = tour_step(dims, &local_guard, |coord| obstacles.contains(coord));

        match result {
            TourResult::OutOfBounds => break,
            TourResult::Stepped => {
                path.insert(next);
            }
            _ => {}
        }

        local_guard.position = next;
        local_guard.direction = ndir;
    }

    path
}

pub fn part_two(input: &str) -> Option<u32> {
    let (dims, mut guard, obstacles) = parse(input);
    let mut visited = HashSet::new();
    let mut step = 0;
    let mut loops = 0;

    // Original solution was some naive code trying to put obstacles everywhere, ~6.4s runtime. I
    // wrote this because I couldn't figure out what was wrong with my initial (optimised)
    // solution, variant below. I'd omitted to consider that it's only valid to add an obstacle at
    // a point in the path if the guard is stepping onto that point for the first time.
    //
    // Later optimised to ~500ms once I had the right answer and could iterate towards it.
    //
    // for y in 0..dims.1 {
    //     for x in 0..dims.0 {
    //         if !obstacles.contains(&(x, y)) {
    //             // try to run the guard with an extra obstacle at this position and see if it loops
    //             let mut visited = HashSet::new();
    //             let looped = guard_tour(
    //                 dims,
    //                 &guard,
    //                 |&coord| obstacles.contains(&coord) || coord == (x, y),
    //                 &mut visited,
    //                 0,
    //             );

    //             if looped {
    //                 loops += 1;
    //             }
    //         }
    //     }
    // }

    // return Some(loops);

    loop {
        step += 1;

        // we fork at this point and recurse twice
        // 1. an extra obstacle appears ahead of the guard, so the guard is forced to turn
        //    In this case we iterate until the guard either loops on their path or leaves the grid
        // 2. guard steps without any extra obstacles in the path
        //
        // this is repeated until the guard's base path leaves the grid, i.e. where we've inserted
        // obstacles at every point they may have stepped onto

        // let's do (2) first by inserting an obstacle at the next position on the tour
        match tour_step(dims, &guard, |coord| obstacles.contains(coord)) {
            (TourResult::OutOfBounds, _, _) => {
                // if the guard would leave the grid on the next step, we cannot insert any more
                // obstacles in their path
                break;
            }
            (TourResult::Stepped, npos, ndir) => {
                // try to insert an obstacle at (nx, ny) and send the guard on tour to see if they
                // loop back to a point we've already visited. We use the same visited map for all
                // iterations. To separately track the paths walked for each attempted obstacle
                // added, there is an additional value 'step' stored in the visit map. This ensures
                // we can search the visit map for only steps on the 'base' path (steps taken prior
                // to encountering the additional obstacle) or steps taken on the current walk with
                // the extra obstacle by matching on the value of 'step'. Step 0 is the base path.
                //
                // we can only insert an obstacle though if this is the first time we are stepping
                // onto (x, y). If we've stepped here before (i.e. going in another direction)
                // we'll invalidate the past path by trying to introduce an obstacle where there
                // previously wasn't one. Obstacles are added at time=0 and not during the guard's
                // walk.
                if !(0..4).any(|dir| visited.contains(&(npos, dir, 0))) // visited in any dir?
                    && guard_tour( // check loops
                        dims,
                        &guard,
                        |&coord| obstacles.contains(&coord) || coord == npos,
                        &mut visited,
                        step,
                    )
                {
                    loops += 1;
                }

                // the guard now advances to the position they would have done if the new obstacle
                // had not been inserted
                visited.insert((npos, ndir, 0));
                guard.position = npos;
                guard.direction = ndir;
            }
            (TourResult::Turned, npos, ndir) => {
                // if the guard is forced to turn, we don't need to insert an obstacle at the
                // reported next position as the position will match our current one
                assert_eq!(guard.position, npos);

                visited.insert((npos, ndir, 0));
                guard.position = npos;
                guard.direction = ndir;
            }
        }
    }

    Some(loops)
}

enum TourResult {
    OutOfBounds,
    Stepped,
    Turned,
}

fn guard_tour<O: Fn(&(usize, usize)) -> bool>(
    dims: (usize, usize),
    guard: &Guard,
    obstacles: O,
    visited: &mut HashSet<((usize, usize), usize, usize)>,
    step: usize,
) -> bool {
    let mut local_guard = guard.clone();

    loop {
        let (result, next, ndir) = tour_step(dims, &local_guard, &obstacles);

        match result {
            TourResult::OutOfBounds => return false,
            TourResult::Stepped => {
                // We can always consider any step on the base path at step 0 without any extra
                // obstacles. Surprisingly this seems to save ~200ms, presumably because we can
                // terminate many walks early on the base path rather than building up the path
                // again in the visit map on the current step.
                if visited.contains(&(next, ndir, 0)) || visited.contains(&(next, ndir, step)) {
                    return true;
                }
            }
            _ => {}
        }

        visited.insert((next, ndir, step));

        local_guard.position = next;
        local_guard.direction = ndir;
    }
}

#[inline]
fn tour_step<O: Fn(&(usize, usize)) -> bool>(
    dims: (usize, usize),
    guard: &Guard,
    obstacles: O,
) -> (TourResult, (usize, usize), usize) {
    let (gx, gy) = guard.position;
    let gdir = guard.direction;

    let (nx, ny) = match gdir {
        0 => (gx as isize, gy as isize - 1),
        1 => (gx as isize + 1, gy as isize),
        2 => (gx as isize, gy as isize + 1),
        3 => (gx as isize - 1, gy as isize),
        _ => panic!("invalid direction"),
    };

    if nx < 0 || ny < 0 || nx >= dims.0 as isize || ny >= dims.1 as isize {
        // left the grid
        return (TourResult::OutOfBounds, (nx as usize, ny as usize), gdir);
    }

    if obstacles(&(nx as usize, ny as usize)) {
        // hit an obstacle
        let gdir = (gdir + 1) % 4;
        (TourResult::Turned, guard.position, gdir)
    } else {
        (TourResult::Stepped, (nx as usize, ny as usize), gdir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
