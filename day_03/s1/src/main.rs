use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

fn parse_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };
    input.to_string()
}

fn run(input: String) -> u32 {
    let (_input, instructions) = parse(&input).unwrap();

    dbg!(&instructions);

    instructions.iter().map(|(x, y)| x * y).sum()
    // todo!();
}

fn instruction(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, (pair.0, pair.1)))
}
fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    many0(many_till(anychar, instruction).map(|(_, ins)| ins))(input)
}

fn main() {
    let input = parse_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 161);
    }
}
