advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    find_hash_with_prefix(input, "00000")
}

fn find_hash_with_prefix(input: &str, prefix: &str) -> Option<u64> {
    let mut i = 0;
    loop {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with(prefix) {
            return Some(i);
        }
        i += 1;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    find_hash_with_prefix(input, "000000")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
