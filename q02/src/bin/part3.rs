use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::rest,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("input3.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_input, (words, sentence)) = parse(input).unwrap();

    let rows = sentence.lines().count();
    let cols = sentence.lines().next().unwrap().len();

    let grid: HashMap<IVec2, char> = sentence
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| (IVec2::new(x as i32, y as i32), ch))
        })
        .collect();

    // dbg!(&grid, &rows, &cols);

    let mut scales = HashSet::<IVec2>::new();

    const DIRECTIONS: [IVec2; 4] = [
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
        IVec2::new(0, 1),
        IVec2::new(0, -1),
    ];

    for word in words {
        // dbg!(word);
        for direction in DIRECTIONS {
            // dbg!(direction);
            for loc in find_overlapping_grid(&grid, rows, cols, word, direction) {
                // dbg!(&word, &direction);
                scales.insert(loc);
                // dbg!(&scales);
            }
        }
    }

    scales.len().to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    let (input, words) = preceded(tag("WORDS:"), separated_list1(tag(","), alpha1))(input)?;
    let (input, sentence) = preceded(many1(line_ending), rest)(input)?;
    // dbg!(&words, &sentence);
    Ok((input, (words, sentence)))
}

fn find_overlapping_grid(
    grid: &HashMap<IVec2, char>,
    rows: usize,
    cols: usize,
    needle: &str,
    direction: IVec2,
) -> Vec<IVec2> {
    let mut match_locs = Vec::new();
    let needle = needle.chars().collect::<Vec<_>>();

    // dbg!(&needle, &direction);

    for row in 0..rows {
        for col in 0..cols {
            let pos = IVec2::new(col as i32, row as i32);
            // dbg!(&pos);
            if grid.get(&pos).unwrap() == &needle[0] {
                // println!("match of {} at {:?}", &needle[0], &pos);
                let mut matches = true;
                for (idx, _) in needle.iter().enumerate().skip(1) {
                    let mut new_col = col as i32 + direction.x * idx as i32;
                    if new_col >= cols as i32 {
                        new_col -= cols as i32;
                    }
                    if new_col < 0 {
                        new_col += cols as i32;
                    }
                    let new_row = row as i32 + direction.y * idx as i32;
                    // dbg!(&new_col, &new_row);

                    let new_grid_pos = IVec2::new(new_col, new_row);

                    if let Some(ch) = grid.get(&new_grid_pos) {
                        // dbg!(&ch, needle[idx]);
                        if *ch != needle[idx] {
                            matches = false;
                        } else {
                            println!("match of {} at {:?}", &needle[idx], &new_grid_pos);
                        }
                    } else {
                        matches = false;
                    }

                    if !matches {
                        break;
                    }
                }

                // dbg!(&matches);

                if matches {
                    // dbg!("matches!");
                    for i in 0..needle.len() {
                        let mut loc = IVec2::new(col as i32, row as i32) + direction * i as i32;
                        loc.x = (loc.x + cols as i32) % cols as i32;
                        match_locs.push(loc);
                    }
                    // dbg!(&match_locs);
                }

            }
        }
    }

    match_locs
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "WORDS:THE,OWE,MES,ROD,RODEO

HELWORLT
ENIGWDXL
TRODEOAL",
        );

        assert_eq!(result, "10");
    }
}
