use aoclib::measure_time;

const REAL_INPUT: &str = include_str!("../inputs/day01.txt");

pub fn part1(input: &str) -> usize {
    let mut dial: i32 = 50;
    let mut zero_ctr: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let move_dial: i32 = line[1..].parse().unwrap();
        let lr = line.as_bytes()[0] as char;

        match lr {
            'L' => {
                dial -= move_dial;
            }

            'R' => {
                dial += move_dial;
            }
            _ => unreachable!(),
        }

        // map to 0..99 using remainder operator
        // need to take remainder again to handle negative dial
        dial = ((dial % 100) + 100) % 100;

        if dial == 0 {
            zero_ctr += 1;
        }
    }
    zero_ctr as usize
}

pub fn part2(_input: &str) -> usize {
    0 // TODO: implement
}

pub fn run() {
    let input = REAL_INPUT;

    let p1 = measure_time("Part 1", || part1(input));
    println!("Part 1 result: {}", p1);

    let p2 = measure_time("Part 2", || part2(input));
    println!("Part 2 result: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_right() {
        let input = "\
R150
";
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn test_left() {
        let input = "\
L150
";
        assert_eq!(part1(input), 1);
    }
}
