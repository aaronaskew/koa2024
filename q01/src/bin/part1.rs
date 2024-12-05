fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    input
        .chars()
        .map(|ch| match ch {
            'B' => 1,
            'C' => 3,
            _ => 0,
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("ABBAC");

        assert_eq!(result, "5");
    }
}
