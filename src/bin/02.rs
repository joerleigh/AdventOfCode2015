use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_area = 0;
    for line in input.lines() {
        let (width, height, depth) = line
            .split('x')
            .map(|x| x.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let front = width * height;
        let side = height * depth;
        let top = width * depth;
        let slack = front.min(side).min(top);
        let area = 2 * front + 2 * side + 2 * top + slack;
        total_area += area;
    }
    Some(total_area)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_length = 0;
    for line in input.lines() {
        let (width, height, depth) = line
            .split('x')
            .map(|x| x.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        let front = 2 * width + 2 * height;
        let side = 2 * height + 2 * depth;
        let top = 2 * width + 2 * depth;
        let bow = width * height * depth;
        let ribbon = front.min(side).min(top) + bow;
        total_length += ribbon;
    }
    Some(total_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58 + 43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34 + 14));
    }
}
