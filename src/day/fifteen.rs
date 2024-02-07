use std::usize;

pub fn main() {
    let answer = part_two(std::fs::read_to_string("inputs/fifteen_small.txt").unwrap());
    assert_eq!(answer, 145);

    let answer = part_two(std::fs::read_to_string("inputs/fifteen_big.txt").unwrap());
    dbg!(answer);
}

fn part_one(content: String) -> usize {
    content.split(',').map(hash).sum()
}

fn hash(content: &str) -> usize {
    content
        .chars()
        .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

fn part_two(content: String) -> usize {
    let operations = content.split(',').map(|split| {
        let symbol = split.matches(['=', '-']).next().unwrap();
        let (label, length) = split.split_once(['=', '-']).unwrap();
        match symbol {
            "=" => Operation::Replace(label.to_string(), length.parse().unwrap()),
            _ => Operation::Remove(label.to_string()),
        }
    });

    let boxes = operations.fold(
        vec![Vec::<(String, u8)>::new(); 256],
        |mut acc, operation| match operation {
            Operation::Replace(label, length) => {
                let hash = hash(&label);
                let label_index = acc[hash].iter().position(|inside| inside.0 == label);
                match label_index {
                    Some(index) => {
                        acc[hash][index] = (label, length);
                        acc
                    }
                    None => {
                        acc[hash].push((label, length));
                        acc
                    }
                }
            }
            Operation::Remove(label) => {
                let hash = hash(&label);
                let label_index = acc[hash].iter().position(|inside| inside.0 == label);
                match label_index {
                    Some(index) => {
                        acc[hash].remove(index);
                        acc
                    }
                    None => acc,
                }
            }
        },
    );

    boxes
        .into_iter()
        .enumerate()
        .map(|(box_index, bax)| {
            bax.into_iter()
                .enumerate()
                .map(|(slot_index, slot)| (slot_index + 1) * slot.1 as usize)
                .sum::<usize>()
                * (box_index + 1)
        })
        .sum()
}

enum Operation {
    Remove(String),
    Replace(String, u8),
}
