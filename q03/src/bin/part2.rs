use std::collections::HashSet;

use glam::IVec2;

fn main() {
    let input = include_str!("input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let earth: HashSet<IVec2> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, ch)| match ch {
                    '#' => Some(IVec2::new(col as i32, row as i32)),
                    _ => None,
                })
        })
        .collect();

    let mut map = Map { earth };

    // dbg!(map);

    let mut total = 0;

    loop {
        if map.earth.is_empty() {
            break;
        } else {
            total += map.earth.len();
        }

        map.step();
    }

    total.to_string()
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

#[derive(Debug)]
struct Map {
    earth: HashSet<IVec2>,
}

impl Map {
    fn step(&mut self) {
        let new_earth = self
            .earth
            .iter()
            .filter_map(|&pos| {
                if self.earth.contains(&(pos + DIRECTIONS[0]))
                    && self.earth.contains(&(pos + DIRECTIONS[1]))
                    && self.earth.contains(&(pos + DIRECTIONS[2]))
                    && self.earth.contains(&(pos + DIRECTIONS[3]))
                {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect();

        self.earth = new_earth;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "..........
..###.##..
...####...
..######..
..######..
...####...
..........",
        );

        assert_eq!(result, "35");
    }
}
