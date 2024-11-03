fn parse_input(input: Option<&str>) -> Vec<String> {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };
    let output = input
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|o| o.to_string())
        .collect::<Vec<String>>();

    output
}

#[derive(Debug, PartialEq, Eq)]
enum Report {
    Unsafe,
    Safe,
}

#[derive(Debug, PartialEq, Eq)]
enum Trend {
    Increasing,
    Decreasing,
}

fn check(pair: &[i32], trend: &mut Option<Trend>) -> Report {
    let diff = pair[1] - pair[0];
    if diff.abs() > 3 || diff.abs() == 0 {
        Report::Unsafe
    } else {
        match trend {
            None => {
                *trend = Some(if diff > 0 {
                    Trend::Increasing
                } else {
                    Trend::Decreasing
                });
                Report::Safe
            }
            Some(x) => {
                if *x == Trend::Increasing && diff < 0 {
                    *trend = Some(Trend::Decreasing);
                    Report::Unsafe
                } else if *x == Trend::Decreasing && diff > 0 {
                    *trend = Some(Trend::Increasing);
                    Report::Unsafe
                } else {
                    Report::Safe
                }
            }
        }
    }
}

fn run(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut previous_trend: Option<Trend> = None;
            line.split_whitespace()
                .map(|o| o.parse().unwrap())
                .collect::<Vec<i32>>()
                .windows(2)
                .map(|pair| check(pair, &mut previous_trend))
                .collect::<Vec<Report>>()
        })
        .filter(|x| x.iter().all(|o| *o == Report::Safe))
        .count()
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
            "
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 2);
    }
}
