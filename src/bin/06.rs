use fancy_regex::Regex;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lights = vec![vec![false; 1000]; 1000];
    let turn_on = Regex::new(r"turn on (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let toggle = Regex::new(r"toggle (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let turn_off = Regex::new(r"turn off (\d+),(\d+) through (\d+),(\d+)").unwrap();
    for line in input.lines() {
        if let Ok(Some(captures)) = turn_on.captures(line) {
            let x1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let y1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let x2: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let y2: usize = captures.get(4).unwrap().as_str().parse().unwrap();
            for x in x1..=x2 {
                for y in y1..=y2 {
                    lights[x][y] = true;
                }
            }
        } else if let Ok(Some(captures)) = toggle.captures(line) {
            let x1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let y1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let x2: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let y2: usize = captures.get(4).unwrap().as_str().parse().unwrap();
            for x in x1..=x2 {
                for y in y1..=y2 {
                    lights[x][y] = !lights[x][y];
                }
            }
        } else if let Ok(Some(captures)) = turn_off.captures(line) {
            let x1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let y1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let x2: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let y2: usize = captures.get(4).unwrap().as_str().parse().unwrap();
            for x in x1..=x2 {
                for y in y1..=y2 {
                    lights[x][y] = false;
                }
            }
        }
    }

    let mut total_lights = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if lights[x][y] {
                total_lights += 1;
            }
        }
    }
    Some(total_lights)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lights: Vec<Vec<u64>> = vec![vec![0; 1000]; 1000];
    let turn_on = Regex::new(r"turn on (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let toggle = Regex::new(r"toggle (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let turn_off = Regex::new(r"turn off (\d+),(\d+) through (\d+),(\d+)").unwrap();
    for line in input.lines() {
        if let Ok(Some(captures)) = turn_on.captures(line) {
            let x1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let y1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let x2: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let y2: usize = captures.get(4).unwrap().as_str().parse().unwrap();
            for x in x1..=x2 {
                for y in y1..=y2 {
                    lights[x][y] += 1;
                }
            }
        } else if let Ok(Some(captures)) = toggle.captures(line) {
            let x1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let y1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let x2: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let y2: usize = captures.get(4).unwrap().as_str().parse().unwrap();
            for x in x1..=x2 {
                for y in y1..=y2 {
                    lights[x][y] += 2;
                }
            }
        } else if let Ok(Some(captures)) = turn_off.captures(line) {
            let x1: usize = captures.get(1).unwrap().as_str().parse().unwrap();
            let y1: usize = captures.get(2).unwrap().as_str().parse().unwrap();
            let x2: usize = captures.get(3).unwrap().as_str().parse().unwrap();
            let y2: usize = captures.get(4).unwrap().as_str().parse().unwrap();
            for x in x1..=x2 {
                for y in y1..=y2 {
                    lights[x][y] = lights[x][y].saturating_sub(1);
                }
            }
        }
    }

    let mut total_brightness = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            total_brightness += lights[x][y];
        }
    }
    Some(total_brightness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1000000 - 1000 - 4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1000000 + 2000 - 4));
    }
}
