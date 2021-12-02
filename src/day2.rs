pub fn generator(input: &str) -> Vec<(&str, u32)> {
    input
        .lines()
        .map(|line| { 
            let mut d = line.splitn(2, ' ');
            (d.next().unwrap(), d.next().unwrap().parse::<u32>().unwrap())
        })
        .collect()
}

pub fn part_1(input: &[(&str, u32)]) -> u32 {
    let (horizontal, depth) = input.iter().cloned().fold((0, 0), |position, movement| {
        let (mut horizontal, mut depth) = position;
        let (direction, value) = movement;
        if direction == "forward" {
            horizontal += value;
        } else if direction == "down" {
            depth += value;
        } else if direction == "up" {
            depth -= value;
        }

        (horizontal, depth)
    });

    horizontal * depth
}

pub fn part_2(input: &[(&str, u32)]) -> u32 {
    let (horizontal, depth, _aim) = input.iter().cloned().fold((0, 0, 0), |position, movement| {
        let (mut horizontal, mut depth, mut aim) = position;
        let (direction, value) = movement;
        if direction == "forward" {
            horizontal += value;
            depth += aim * value;
        } else if direction == "down" {
            aim += value;
        } else if direction == "up" {
            aim -= value;
        }

        (horizontal, depth, aim)
    });

    horizontal * depth
}
