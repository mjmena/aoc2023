use std::{
    cmp::{max, min},
    collections::HashMap,
    i128,
};

fn main() {
    let file = std::fs::read_to_string("day_eight_input.txt").unwrap();

    let directions = file.lines().take(1).collect::<String>();

    let map = file
        .lines()
        .skip(2)
        .map(|line| {
            let (hash, value) = line.split_once('=').unwrap();
            let hash = hash.trim();

            let (left, right) = value[2..value.len() - 1].split_once(",").unwrap();
            (hash, (left, right.trim()))
        })
        .collect::<HashMap<_, _>>();

    let starts = map
        .clone()
        .into_keys()
        .filter(|key| key[key.len() - 1..] == String::from("A"))
        .collect::<Vec<_>>();

    let ends = starts
        .into_iter()
        .map(|start| {
            directions
                .chars()
                .cycle()
                .scan(start, |location, direction| {
                    let (left, right) = map[location];
                    *location = match direction {
                        'L' => left,
                        'R' => right,
                        _ => "",
                    };
                    Some(*location)
                })
                .enumerate()
                .filter(|(_, location)| location[location.len() - 1..] == *"Z")
                .nth(0)
                .unwrap()
                .0
                + 1
        })
        .collect::<Vec<_>>();

    let gcf = ends
        .clone()
        .into_iter()
        .reduce(|acc, current| greatest_common_factor(acc, current))
        .unwrap();
    let answer = ends.into_iter().map(|end| end / gcf).product::<usize>() * gcf;

    dbg!(answer);
}

fn greatest_common_factor(a: usize, b: usize) -> usize {
    let max = max(a, b);
    let min = min(a, b);
    if min == 0 {
        max
    } else {
        greatest_common_factor(min, max % min)
    }
}
