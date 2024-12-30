use fancy_regex::Regex;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_nice = 0;
    for line in input.lines() {
        if has_three_vowels(line) && has_double_letter(line) && has_no_bad_strings(line) {
            total_nice += 1;
        }
    }
    Some(total_nice)
}

fn has_no_bad_strings(string: &str) -> bool {
    const BAD_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];
    for bad_string in BAD_STRINGS.iter() {
        if string.contains(bad_string) {
            return false;
        }
    }
    true
}

fn has_double_letter(string: &str) -> bool {
    let re = Regex::new(r"([a-z])\1").unwrap();
    re.is_match(string).unwrap()
}

fn has_three_vowels(string: &str) -> bool {
    let re = Regex::new(r"[aeiou]").unwrap();
    re.find_iter(string).count() >= 3
}

fn has_repeated_pair(string: &str) -> bool {
    let re = Regex::new(r"([a-z]{2}).*\1").unwrap();
    re.is_match(string).unwrap()
}

fn has_repeated_letter_with_gap(string: &str) -> bool {
    let re = Regex::new(r"([a-z]).\1").unwrap();
    re.is_match(string).unwrap()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_nice = 0;
    for line in input.lines() {
        if has_repeated_pair(line) && has_repeated_letter_with_gap(line) {
            total_nice += 1;
        }
    }
    Some(total_nice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
