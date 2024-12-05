use itertools::Itertools;

fn main() {
    let input = include_str!("input3.txt");
    let output = process(input);
    dbg!(output);
}

fn damage(ch: Option<char>) -> u32 {
    match ch {
        Some('B') => 1,
        Some('C') => 3,
        Some('D') => 5,
        _ => 0,
    }
}

fn process(input: &str) -> String {
    dbg!(input.len(), input.len() % 3 == 0);
    input
        .chars()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let a = group.next();
            let b = group.next();
            let c = group.next();

            let blanks = if a == Some('x') { 1 } else { 0 }
                + if b == Some('x') { 1 } else { 0 }
                + if c == Some('x') { 1 } else { 0 };
            dbg!(&blanks);

            damage(a)
                + damage(b)
                + damage(c)
                + match blanks {
                    0 => 6,
                    1 => 2,
                    _ => 0,
                }
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process("xBxAAABCDxCC");

        assert_eq!(result, "30");
    }
}
