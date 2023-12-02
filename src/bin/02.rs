use std::cmp::max;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let i: u32 = input
        .lines()
        .filter(|l| {
            l.split(":")
                .last()
                .unwrap()
                .split(";")
                .map(|x| {
                    x.split(",")
                        .map(|y| {
                            let mut nums = y.split(" ");
                            nums.next();
                            let amount = nums.next().unwrap().parse::<u32>().unwrap();
                            let color = nums.next().unwrap();
                            match color {
                                "green" => amount < 14,
                                "blue" => amount < 15,
                                "red" => amount < 13,
                                _ => {
                                    println!("{color}");
                                    true
                                }
                            }
                        })
                        .all(|i| i)
                })
                .all(|x| x)
        })
        .map(|n| {
            n.split(":")
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .sum();
    Some(i)
}

pub fn part_two(input: &str) -> Option<u32> {
    let i: u32 = input
        .lines()
        .map(|l| {

            let (mut r, mut g, mut b) = (0, 0, 0);
            l.split(":").last().unwrap().split(";").map(|round| {
                let _ = round
                    .split(",")
                    .map(|throw| {
                        let mut nums = throw.split(" ");
                        nums.next();
                        let amount = nums.next().unwrap().parse::<u32>().unwrap();
                        let color = nums.next().unwrap();
                        match color {
                            "green" => g = max(g, amount),
                            "blue" => b = max(b, amount),
                            "red" => r = max(r, amount),
                            _ => {
                                println!("{color}");
                            }
                        }
                    })
                    .count();

            }
            ).count();
            r * b * g
        })
        .sum();
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
