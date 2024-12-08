use std::collections::HashMap;

use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_input, data) = parse(input).expect("should parse");

    let mut dance_columns = transpose(&data);

    let cols = dance_columns.len();

    let mut i = 0;

    let mut shouts: HashMap<String, u64> = HashMap::new();

    loop {
        // dbg!(&dance_columns);
        let dancer = dance_columns[i % cols].remove(0);
        dance(dancer, &mut dance_columns[(i + 1) % cols]);

        let shout = format!(
            "{}{}{}{}",
            dance_columns[0][0], dance_columns[1][0], dance_columns[2][0], dance_columns[3][0]
        );

        shouts
            .entry(shout.clone())
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);

        println!(
            "round: {} shout: {} count: {}",
            i + 1,
            shout,
            shouts.get(&shout).unwrap()
        );

        if *shouts.get(&shout).unwrap() == 2024 {
            return (shout.parse::<u64>().unwrap() * (i as u64 + 1)).to_string();
        }

        i += 1;
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let (input, input_grid) =
        separated_list1(line_ending, separated_list1(space1, complete::u64))(input)?;

    Ok((input, input_grid))
}

fn transpose(data: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let width = data[0].len();

    let mut output: Vec<Vec<u64>> = vec![Vec::new(); width];

    for line in data {
        for (i, num) in line.iter().enumerate() {
            output[i].push(*num);
        }
    }

    output
}

fn dance(dancer: u64, column: &mut Vec<u64>) {
    let pos = (dancer as usize - 1) % (2 * column.len());

    // dbg!(&pos, &column.len());

    let absorb_idx = {
        if pos < column.len() {
            pos
        } else {
            2 * column.len() - pos
        }
    };

    column.insert(absorb_idx, dancer);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dance_test() {
        let data = vec![
            vec![2, 3, 4, 5],
            vec![3, 4, 5, 2],
            vec![4, 5, 2, 3],
            vec![5, 2, 3, 4],
        ];
        let mut columns = transpose(&data);

        dance(2, &mut columns[1]);

        assert_eq!(columns[1], vec![3, 2, 4, 5, 2]);

        dance(5, &mut columns[2]);

        assert_eq!(columns[2], vec![4, 5, 2, 3, 5]);
    }

    #[test]
    fn transpose_test() {
        let data = vec![
            vec![2, 3, 4, 5],
            vec![3, 4, 5, 2],
            vec![4, 5, 2, 3],
            vec![5, 2, 3, 4],
        ];
        let columns = transpose(&data);

        assert_eq!(columns[0], vec![2, 3, 4, 5]);
        assert_eq!(columns[3], vec![5, 2, 3, 4]);
    }

    #[test]
    fn example() {
        let result = process(
            "2 3 4 5
6 7 8 9",
        );

        assert_eq!(result, "50877075");
    }
}
