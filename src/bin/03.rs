use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut houses = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    houses.insert((x, y), 1);
    for direction in input.chars() {
        move_santa(&mut x, &mut y, direction);
        houses
            .entry((x, y))
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    Some(houses.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut houses = HashMap::new();
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    let mut santas_turn = true;
    for direction in input.chars() {
        if santas_turn {
            move_santa(&mut santa_x, &mut santa_y, direction);
            houses
                .entry((santa_x, santa_y))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        } else {
            move_santa(&mut robo_x, &mut robo_y, direction);
            houses
                .entry((robo_x, robo_y))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        };

        santas_turn = !santas_turn;
    }
    Some(houses.len() as u64)
}

fn move_santa(x: &mut i32, y: &mut i32, direction: char) {
    match direction {
        '^' => *y += 1,
        'v' => *y -= 1,
        '>' => *x += 1,
        '<' => *x -= 1,
        _ => panic!("Invalid direction"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
