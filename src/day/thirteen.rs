use std::usize;

pub fn main() {
    let answer = part_one(std::fs::read_to_string("inputs/thirteen_small.txt").unwrap());
    assert_eq!(answer, 400);

    let answer = part_one(std::fs::read_to_string("inputs/thirteen_big.txt").unwrap());
    dbg!(answer);
}

fn part_one(content: String) -> usize {
    let chunks = make_chunks(&content);
    chunks.map(parse_chunk).sum()
}

fn make_chunks(content: &String) -> impl Iterator<Item = &str> {
    content.split("\r\n\r\n")
}

fn parse_chunk(chunk: &str) -> usize {
    let horizontals = get_horizontal(chunk);
    let verticals = get_vertical(chunk);

    100 * find_reflection(&horizontals) + find_reflection(&verticals)
}

fn find_reflection(data: &[usize]) -> usize {
    match (1..data.len()).find(|i| {
        let left = &data[std::cmp::max(0, (*i as i32 * 2 - data.len() as i32)) as usize..*i];
        let right = &data[*i..std::cmp::min(i * 2, data.len())]
            .iter()
            .copied()
            .rev()
            .collect::<Vec<_>>();
        compare_smudged_slices(left, right)
    }) {
        Some(i) => i,
        _ => 0,
    }
}

fn compare_smudged_slices(left: &[usize], right: &[usize]) -> bool {
    let mut smudges = left.iter().zip(right).filter(|(left, right)| left != right);
    if smudges.clone().count() != 1 {
        return false;
    }

    let (left, right) = smudges.next().unwrap();

    let n = left ^ right;
    n & (n - 1) == 0
}

fn get_horizontal(chunk: &str) -> Vec<usize> {
    chunk
        .lines()
        .map(|line| {
            line.chars().fold(0 as usize, |sum, c| {
                sum << 1
                    | match c {
                        '#' => 1,
                        _ => 0,
                    }
            })
        })
        .collect::<Vec<_>>()
}

fn get_vertical(chunk: &str) -> Vec<usize> {
    let verticals = chunk
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (0..verticals[0].len())
        .map(|i| {
            (0..verticals.len()).fold(0, |sum, j| {
                sum << 1
                    | match verticals[j][i] {
                        '#' => 1,
                        _ => 0,
                    }
            })
        })
        .collect::<Vec<_>>()
}
