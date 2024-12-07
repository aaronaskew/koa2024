use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::rest,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_input, (words, sentence)) = parse(input).unwrap();

    let mut symbol_indices = HashSet::new();

    words.iter().for_each(|word| {
        let mut matches = find_overlapping(sentence, word);

        // if word is a palindrome, do not reverse search for it
        if reverse(word) != *word {
            matches.append(&mut find_overlapping(sentence, &reverse(word)));
        }

        for idx in &matches {
            for i in *idx..(idx + word.len()) {
                symbol_indices.insert(i);
            }
        }

        dbg!(&word, &matches, matches.len(), &symbol_indices);
    });
    symbol_indices.len().to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    let (input, words) = preceded(tag("WORDS:"), separated_list1(tag(","), alpha1))(input)?;
    let (input, sentence) = preceded(many1(line_ending), rest)(input)?;
    dbg!(&words, &sentence);
    Ok((input, (words, sentence)))
}

fn find_overlapping(haystack: &str, needle: &str) -> Vec<usize> {
    let mut matches = Vec::new();
    for (i, _) in haystack.char_indices() {
        if haystack[i..].starts_with(needle) {
            matches.push(i);
        }
    }
    matches
}

fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "WORDS:THE,OWE,MES,ROD,HER,QAQ

AWAKEN THE POWE ADORNED WITH THE FLAMES BRIGHT IRE
THE FLAME SHIELDED THE HEART OF THE KINGS
POWE PO WER P OWE R
THERE IS THE END
QAQAQ",
        );

        assert_eq!(result, "42");
    }
}
