use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone)]
enum Instruction {
    Do,
    Dont,
    Mul(u32, u32),
}

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

    let mut append = true;
    let mut result = 0;
    for ins in instructions {
        match ins {
            Instruction::Dont => append = false,
            Instruction::Do => append = true,
            Instruction::Mul(x, y) => {
                if append {
                    result += x * y
                }
            }
        }
    }
    dbg!(result)
}

fn parse_instruction_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parse_instruction_mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many0(many_till(anychar, parse_instruction).map(|(_, ins)| ins))(input)
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
        // Take care the example changed !
        let input = parse_input(Some(indoc!(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 48);
    }
}
