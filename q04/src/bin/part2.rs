fn main() {
    let input = include_str!("input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let nails: Vec<u32> = input
        .lines()
        .map(|l| l.parse().expect("to parse the length"))
        .collect();

    let min_length = nails.iter().min().unwrap();

    nails
        .iter()
        .map(|length| {
            assert!(length >= min_length);

            length - min_length
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "3
4
7
8",
        );

        assert_eq!(result, "10");
    }
}
