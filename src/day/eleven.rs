use std::cmp::{max, min};

pub fn main() {
    let answer = part_two(std::fs::read_to_string("inputs/eleven_small.txt").unwrap());
    // assert_eq!(answer, 374);

    let answer = part_two(std::fs::read_to_string("inputs/eleven_big.txt").unwrap());
    dbg!(answer);
}

fn part_one(content: String) -> usize {
    let universe: Vec<Vec<_>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("not a space"),
                })
                .collect()
        })
        .collect();

    let galaxies = find_all_galaxies(expand(universe));
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(index, galaxy)| {
            galaxies
                .clone()
                .into_iter()
                .skip(index + 1)
                .map(move |far_galaxy| {
                    // dbg!(&far_galaxy, &galaxy);
                    let difference = (max(far_galaxy.0, galaxy.0) - min(far_galaxy.0, galaxy.0))
                        + (max(far_galaxy.1, galaxy.1) - min(far_galaxy.1, galaxy.1));
                    difference
                })
        })
        .sum()
}

fn part_two(content: String) -> usize {
    let universe: Vec<Vec<_>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Space::Empty,
                    '#' => Space::Galaxy,
                    _ => panic!("not a space"),
                })
                .collect()
        })
        .collect();

    let rows: Vec<usize> = universe
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row.into_iter().all(|space| *space == Space::Empty))
        .map(|(index, _)| index)
        .collect();

    let columns: Vec<usize> = (0..universe[0].len())
        .filter(|column| (0..universe.len()).all(|row| universe[row][*column] == Space::Empty))
        .collect();

    let galaxies = find_all_galaxies(universe);
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(index, galaxy)| {
            galaxies
                .clone()
                .into_iter()
                .skip(index + 1)
                .map(|far_galaxy| {
                    // dbg!(&far_galaxy, &galaxy);
                    galaxy.0.abs_diff(far_galaxy.0)
                        + galaxy.1.abs_diff(far_galaxy.1)
                        + (rows
                            .iter()
                            .filter(|row| {
                                min(far_galaxy.0, galaxy.0) < **row
                                    && max(far_galaxy.0, galaxy.0) > **row
                            })
                            .count()
                            + columns
                                .iter()
                                .filter(|column| {
                                    min(far_galaxy.1, galaxy.1) < **column
                                        && max(far_galaxy.1, galaxy.1) > **column
                                })
                                .count())
                            * 999999
                })
        })
        .sum()
}

fn find_all_galaxies(universe: Vec<Vec<Space>>) -> Vec<(usize, usize)> {
    let columns = universe[0].len();

    universe
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(move |(index, space)| match space {
            Space::Empty => None,

            Space::Galaxy => Some(((index / columns), index % columns)),
        })
        .collect()
}

fn expand(universe: Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    let rows: Vec<usize> = universe
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row.into_iter().all(|space| *space == Space::Empty))
        .map(|(index, _)| index)
        .collect();
    let columns: Vec<usize> = (0..universe[0].len())
        .filter(|column| (0..universe.len()).all(|row| universe[row][*column] == Space::Empty))
        .collect();
    let empty_row = vec![Space::Empty; universe[0].len()];
    let mut expanded_universe = universe;

    rows.into_iter().rev().for_each(|row| {
        expanded_universe.insert(row, empty_row.clone());
    });

    expanded_universe.iter_mut().for_each(|row| {
        columns.iter().rev().for_each(|column| {
            row.insert(*column, Space::Empty);
        });
    });

    expanded_universe
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Space {
    Empty,
    Galaxy,
}
