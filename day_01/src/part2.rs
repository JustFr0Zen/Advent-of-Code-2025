use crate::part1::Direction::{AntiClockwise, Clockwise};
use crate::part1::{parse, START_POSITION, UPPER_LIMIT};

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
            let new_value = move_a + move_b;

            num_clicks += new_value.abs() / UPPER_LIMIT;

            if new_value <= 0 && move_a != 0 {
                num_clicks += 1;
            }

            new_value.rem_euclid(UPPER_LIMIT)
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
        assert_eq!(6, process(input)?);
        Ok(())
    }

    #[test]
    fn test_right_click() -> miette::Result<()> {
        let input = "R1000";
        assert_eq!(10, process(input)?);
        Ok(())
    }

    #[test]
    fn test_left_click() -> miette::Result<()> {
        let input = "L1000";
        assert_eq!(10, process(input)?);
        Ok(())
    }
    #[test]
    fn test_small_left_click() -> miette::Result<()> {
        let input = "L50";
        assert_eq!(1, process(input)?);

        let input = "L50
L2";
        assert_eq!(1, process(input)?);
        Ok(())
    }

    #[test]
    fn test_right_left_click() -> miette::Result<()> {
        let input = "R50";
        assert_eq!(1, process(input)?);

        let input = "R52";
        assert_eq!(1, process(input)?);
        Ok(())
    }
}
