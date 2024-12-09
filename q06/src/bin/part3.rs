use std::{borrow::BorrowMut, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{complete::alpha1, complete::line_ending},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("input3.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_, list) = parse(input).expect("should parse");

    dbg!(&list);

    let mut paths = list
        .iter()
        .filter(|(k, v)| v.contains(&"@".to_string()) && !matches!(k.as_str(), "BUG" | "ANT"))
        .inspect(|x| {
            dbg!(x);
        })
        .map(|(source, _)| {
            let mut path = Vec::new();
            let mut source = source.clone();
            path.push("@".to_string());
            path.push(source.clone());

            loop {
                source = list
                    .iter()
                    .filter(|(_, v)| v.contains(&source))
                    .map(|(k, _)| k.clone())
                    .next()
                    .unwrap();

                path.push(source.clone());

                if &source == "RR" {
                    break;
                }
            }
            path
        })
        .collect::<Vec<_>>();

    dbg!(&paths);

    paths.sort_by_key(|a| a.len());

    let unique = paths
        .chunk_by(|a, b| a.len() == b.len())
        .borrow_mut()
        .find(|chunk| chunk.len() == 1)
        .unwrap()
        .first()
        .unwrap();

    dbg!(&unique);

    let mut unique_path = String::new();

    for s in unique.iter().rev() {
        unique_path += &format!("{}", s.chars().next().unwrap());
    }

    unique_path
}

fn parse(input: &str) -> IResult<&str, HashMap<String, Vec<String>>> {
    dbg!(&input);

    let (input, res) = separated_list0(
        line_ending,
        separated_pair(
            alpha1,
            tag(":"),
            separated_list0(tag(","), alt((alpha1, tag("@")))),
        ),
    )(input)?;

    dbg!(&res);

    let map = res
        .iter()
        .map(|v| {
            let key = v.0.to_string();
            let val = v.1.iter().map(|s| s.to_string()).collect();

            (key, val)
        })
        .collect();

    Ok((input, map))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "RR:A,B,C
A:D,E
B:F,@
C:G,H
D:@
E:@
F:@
G:@
H:@",
        );

        assert_eq!(result, "RB@");
    }
}
