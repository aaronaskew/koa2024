use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::rest,
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};

fn main() {
    let input = include_str!("input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let (_input, (words, sentence)) = parse(input).unwrap();

    words
        .iter()
        .map(|word| {
            let matches = sentence.match_indices(*word).count();
            dbg!(&matches);
            matches
        })
        .sum::<usize>()
        .to_string()
}

fn parse(input: &str) -> IResult<&str, (Vec<&str>, &str)> {
    let (input, words) = preceded(tag("WORDS:"), separated_list1(tag(","), alpha1))(input)?;
    let (input, sentence) = preceded(many1(line_ending), rest)(input)?;
    dbg!(&words, &sentence);
    Ok((input, (words, sentence)))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example() {
        let result = process(
            "WORDS:THE,OWE,MES,ROD,HER

AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE",
        );

        assert_eq!(result, "4");
    }
}
