advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut floor = 0;
    for c in input.chars() {
        if c == ')' {
            floor -= 1;
        } else if c == '(' {
            floor += 1;
        }
    }
    Some(floor)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == ')' {
            floor -= 1;
        } else if c == '(' {
            floor += 1;
        }
        if floor == -1 {
            return Some((i + 1) as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
