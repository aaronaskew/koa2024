use std::collections::HashMap;

fn main() {
    let input = include_str!("input3.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let nails: Vec<u32> = input
        .lines()
        .map(|l| l.parse().expect("to parse the length"))
        .collect();

    let length_hits: HashMap<u32, u32> = nails
        .iter()
        .map(|length| {
            let hits = nails
                .iter()
                .map(|test_length| length.abs_diff(*test_length))
                .sum();

            (*length, hits)
        })
        .collect();

    dbg!(&length_hits);

    length_hits.values().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "2
4
5
6
8",
        );

        assert_eq!(result, "8");
    }
}
