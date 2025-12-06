use crate::part1::Direction::{AntiClockwise, Clockwise};
use std::str::FromStr;

const UPPER_LIMIT: i32 = 100;
const START_POSITION: i32 = 50;

#[derive(Debug)]
pub(self) enum Direction {
    Clockwise(i32),
    AntiClockwise(i32),
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(
        input: &str,
    ) -> Result<Direction, Self::Err> {
        let (first_char, value) = input.split_at(1);
        let numeric_value = value.parse::<i32>();

        if numeric_value.is_err() {
            return Err(());
        }

        match first_char {
            "L" => {
                Ok(AntiClockwise(numeric_value.unwrap()))
            }
            "R" => Ok(Clockwise(numeric_value.unwrap())),
            _ => Err(()),
        }
    }
}

#[tracing::instrument]
fn parse(input: &str) -> Result<Vec<Direction>, ()> {
    let parsed_values = input
        .lines()
        .map(|line| line.parse::<Direction>().unwrap())
        .collect::<Vec<Direction>>();

    if parsed_values.is_empty() {
        Err(())
    } else {
        Ok(parsed_values)
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<i32> {
    let mut directions = parse(input).unwrap();
    directions.insert(0, Clockwise(START_POSITION));

    let mut num_clicks = 0;

    directions
        .iter()
        .map(|direction| match direction {
            Clockwise(value) => *value,
            AntiClockwise(value) => -value,
        })
        .reduce(|move_a, move_b| {
            let new_value = (move_a + move_b) % UPPER_LIMIT;

            if new_value == 0 {
                num_clicks += 1;
            }

            new_value
        });
    Ok(num_clicks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(3, process(input)?);
        Ok(())
    }
}
