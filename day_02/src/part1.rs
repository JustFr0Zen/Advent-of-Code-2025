use std::str::FromStr;

struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once('-').ok_or(())?;

        Ok(Range {
            start: first.parse().expect(
                format!(
                    "Failed to parse starting value {first}"
                )
                .as_str(),
            ),
            end: second.parse().expect(
                format!(
                    "Failed to parse ending value {second}"
                )
                .as_str(),
            ),
        })
    }
}

pub(self) enum RoundingBehaviour {
    RoundDown,
    RoundUp,
}

#[tracing::instrument]
fn parse(input: &str) -> Vec<Range> {
    let ranges = input
        .split(',')
        .map(|range| range.parse::<Range>().unwrap())
        .collect();

    ranges
}

fn get_invalid_range_ending(
    value: &u64,
    rounding_behaviour: RoundingBehaviour,
) -> u64 {
    let rank = value.ilog10() + 1;

    if rank % 2 == 1 {
        match rounding_behaviour {
            RoundingBehaviour::RoundDown => {
                let new_rank = (rank + 1) / 2;

                10_u64.pow(new_rank - 1) - 1
            }
            RoundingBehaviour::RoundUp => {
                let new_rank = (rank + 1) / 2;

                10_u64.pow(new_rank - 1)
            }
        }
    } else {
        let left_part = value / (10_u64.pow(rank / 2));
        let right_part =
            value - (left_part * (10_u64.pow(rank / 2)));

        match rounding_behaviour {
            RoundingBehaviour::RoundDown => {
                if right_part >= left_part {
                    left_part
                } else {
                    left_part - 1
                }
            }
            RoundingBehaviour::RoundUp => {
                if left_part >= right_part {
                    left_part
                } else {
                    left_part + 1
                }
            }
        }
    }
}

fn duplicate_exp(value: &u64) -> u64 {
    10_u64.pow(value.ilog10() + 1) * value + value
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u64> {
    let parsed_ranges = parse(input);

    let sum_invalid_ids = parsed_ranges
        .iter()
        .map(|range| {
            let start = get_invalid_range_ending(
                &range.start,
                RoundingBehaviour::RoundUp,
            );
            let end = get_invalid_range_ending(
                &range.end,
                RoundingBehaviour::RoundDown,
            );

            (start..=end)
                .into_iter()
                .map(|value| duplicate_exp(&value))
                .sum::<u64>()
        })
        .sum();

    Ok(sum_invalid_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_range_uneven_length()
    -> miette::Result<()> {
        let num_value = 100;
        assert_eq!(
            get_invalid_range_ending(
                &num_value,
                RoundingBehaviour::RoundUp
            ),
            10
        );
        assert_eq!(
            get_invalid_range_ending(
                &num_value,
                RoundingBehaviour::RoundDown
            ),
            9
        );

        Ok(())
    }

    #[test]
    fn test_invalid_range_bigger_right_part()
    -> miette::Result<()> {
        let num_value = 1010;
        assert_eq!(
            get_invalid_range_ending(
                &num_value,
                RoundingBehaviour::RoundUp
            ),
            10
        );
        assert_eq!(
            get_invalid_range_ending(
                &num_value,
                RoundingBehaviour::RoundDown
            ),
            10
        );

        Ok(())
    }

    #[test]
    fn test_invalid_range_bigger_left_part()
    -> miette::Result<()> {
        let num_value = 1000;
        assert_eq!(
            get_invalid_range_ending(
                &num_value,
                RoundingBehaviour::RoundUp
            ),
            10
        );
        assert_eq!(
            get_invalid_range_ending(
                &num_value,
                RoundingBehaviour::RoundDown
            ),
            9
        );

        Ok(())
    }

    #[test]
    fn test_exp_duplication() -> miette::Result<()> {
        assert_eq!(duplicate_exp(&10), 1010);
        assert_eq!(duplicate_exp(&155), 155155);

        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(1227775554, process(input)?);
        Ok(())
    }
}
