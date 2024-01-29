#![feature(iter_array_chunks)]

use std::usize;

fn main() {
    dbg!(part_one());
    dbg!(part_two());
}

fn part_one() -> usize {
    let file = std::fs::read_to_string("./day_six_input.txt").unwrap();

    let mut parsed = file.lines().map(|line| {
        let (_, times) = line.split_once(':').unwrap();
        times
            .trim()
            .split_whitespace()
            .map(|time| time.parse::<u64>().unwrap())
    });
    let data = parsed.next().unwrap().zip(parsed.next().unwrap());
    data.map(|(time, distance)| {
        (0..time)
            .filter(|seconds| distance < seconds * (time - seconds))
            .count()
    })
    .product()
}

fn part_two() -> usize {
    let file = std::fs::read_to_string("./input.txt").unwrap();

    let mut parsed = file.lines().map(|line| {
        let (_, times) = line.split_once(':').unwrap();
        times
            .trim()
            .split_whitespace()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    });
    let (time, distance) = (parsed.next().unwrap(), parsed.next().unwrap());
    dbg!(time);
    dbg!(distance);
    (0..time)
        .filter(|seconds| distance < seconds * (time - seconds))
        .count()
}
