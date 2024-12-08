use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_input, data) = parse(input).expect("should parse");

    let mut dance_columns = transpose(&data);

    let cols = dance_columns.len();

    for i in 0_usize..10 {
        let dancer = dance_columns[i % cols].remove(0);
        dance(dancer, &mut dance_columns[(i + 1) % cols]);

        println!(
            "round: {} number: {}{}{}{}",
            i + 1,
            dance_columns[0][0],
            dance_columns[1][0],
            dance_columns[2][0],
            dance_columns[3][0]
        );
    }

    format!(
        "{}{}{}{}",
        dance_columns[0][0], dance_columns[1][0], dance_columns[2][0], dance_columns[3][0]
    )
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, input_grid) =
        separated_list1(line_ending, separated_list1(space1, complete::u32))(input)?;

    Ok((input, input_grid))
}

fn transpose(data: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let width = data[0].len();

    let mut output: Vec<Vec<u32>> = vec![Vec::new(); width];

    for line in data {
        for (i, num) in line.iter().enumerate() {
            output[i].push(*num);
        }
    }

    output
}

fn dance(dancer: u32, column: &mut Vec<u32>) {
    let pos = dancer as usize - 1;

    let absorb_idx = {
        if (pos) < column.len() {
            pos
        } else {
            column.len() - (pos - column.len())
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
3 4 5 2
4 5 2 3
5 2 3 4",
        );

        assert_eq!(result, "2323");
    }
}
