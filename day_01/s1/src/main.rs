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

fn run(input: Vec<String>) -> u32 {
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: Vec<u32> = Vec::new();

    input.iter().for_each(|x| {
        let locations = x.split("   ").collect::<Vec<&str>>();
        l1.push(locations[0].parse::<u32>().unwrap());
        l2.push(locations[1].parse::<u32>().unwrap());
    });

    l1.sort();
    l2.sort();
    dbg!(&l1, &l2);

    let dist: u32 = l1
        .iter()
        .zip(l2.iter())
        .map(|(x, y)| if x >= y { x - y } else { y - x })
        .sum();
    dbg!(dist)
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
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 11);
    }
}
