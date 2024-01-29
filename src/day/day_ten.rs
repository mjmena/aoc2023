fn main() {
    let file = std::fs::read_to_string("day_ten_input.txt").unwrap();
    let grid = file
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (0..).fold((18, 74, "N", 0), |(x, y, direction, count), _| {
        let new_coords = match direction {
            "N" => (x, y - 1),
            "S" => (x, y + 1),
            "E" => (x + 1, y),
            "W" => (x - 1, y),
            _ => (0, 0),
        };
        let new_direction = match grid[new_coords.1][new_coords.0] {
            'F' => {
                if direction == "W" {
                    "S"
                } else {
                    "E"
                }
            }
            'J' => {
                if direction == "E" {
                    "N"
                } else {
                    "W"
                }
            }
            'L' => {
                if direction == "S" {
                    "E"
                } else {
                    "N"
                }
            }
            '7' => {
                if direction == "E" {
                    "S"
                } else {
                    "W"
                }
            }
            '|' => {
                if direction == "S" {
                    "S"
                } else {
                    "N"
                }
            }
            '-' => {
                if direction == "W" {
                    "W"
                } else {
                    "E"
                }
            }
            'S' => panic!("{}", (count + 1) / 2),
            _ => "",
        };
        (new_coords.0, new_coords.1, new_direction, count + 1)
    });
}

mod day;

fn main() {
    let file = std::fs::read_to_string("day_ten_input.txt").unwrap();
    let mut grid = file
        .lines()
        .map(|line| line.chars().map(|c| (c, Tile::Empty)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut x = 18;
    let mut y = 74;
    let mut direction = "N";
    let mut polarity = 0;

    loop {
        (x, y) = match direction {
            "N" => (x, y - 1),
            "S" => (x, y + 1),
            "E" => (x + 1, y),
            "W" => (x - 1, y),
            _ => (0, 0),
        };
        let (new_direction, delta_polarity) = match grid[y][x].0 {
            'F' => {
                if direction == "W" {
                    grid[y][x].1 = Tile::Loop;
                    ("S", -1)
                } else {
                    grid[y][x].1 = Tile::Loop;
                    ("E", 1)
                }
            }
            'J' => {
                if direction == "E" {
                    grid[y][x].1 = Tile::EastLoop;

                    ("N", -1)
                } else {
                    grid[y][x].1 = Tile::Loop;
                    ("W", 1)
                }
            }
            'L' => {
                if direction == "S" {
                    grid[y][x].1 = Tile::EastLoop;
                    ("E", -1)
                } else {
                    grid[y][x].1 = Tile::Loop;
                    ("N", 1)
                }
            }
            '7' => {
                if direction == "N" {
                    grid[y][x].1 = Tile::Loop;
                    ("W", -1)
                } else {
                    grid[y][x].1 = Tile::Loop;
                    ("S", 1)
                }
            }
            '|' => {
                if direction == "S" {
                    grid[y][x].1 = Tile::Loop;
                    ("S", 0)
                } else {
                    grid[y][x].1 = Tile::Loop;
                    ("N", 0)
                }
            }
            '-' => {
                if direction == "W" {
                    grid[y][x].1 = Tile::Loop;
                    ("W", 0)
                } else {
                    grid[y][x].1 = Tile::EastLoop;
                    ("E", 0)
                }
            }
            'S' => break,
            _ => ("", 0),
        };
        direction = new_direction;
        polarity += delta_polarity;
    }

    let mut count = 0;

    grid.clone().into_iter().enumerate().for_each(|(i, row)| {
        row.into_iter().enumerate().for_each(|(j, (symbol, kind))| {
            if dbg!(kind) == Tile::EastLoop {
                let x = j;
                let mut y = i;
                dbg!((x, y));
                loop {
                    y += 1;
                    match grid[y][x].1 {
                        Tile::Empty => count += 1,
                        _ => break,
                    }
                }
            }
        })
    });
    dbg!(count);
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum Tile {
    Empty,
    Loop,
    EastLoop,
}
