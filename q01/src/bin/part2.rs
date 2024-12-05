use itertools::Itertools;

fn main() {
    let input = include_str!("input2.txt");
    let output = process(input);
    dbg!(output);
}

fn damage(ch: char) -> u32 {
    match ch {
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}

fn process(input: &str) -> String {
    input
        .chars()
        .chunks(2)
        .into_iter()
        .map(|mut pair| {
            let a = pair.next().unwrap();
            let b = pair.next().unwrap();

            damage(a) + damage(b) + if a == 'x' || b == 'x' { 0 } else { 2 }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("AxBCDDCAxD");

        assert_eq!(result, "28");
    }
}
