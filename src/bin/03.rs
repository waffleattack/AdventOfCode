use itertools::Itertools;
advent_of_code::solution!(3);
use rayon::prelude::*;

#[derive(Debug)]
struct Run {
    is_part: bool,
    num: String,
}
#[derive(Debug)]
struct RunP2 {
    num: Option<u32>,
    pos: (usize, usize),
    len: usize,
    is_num: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|f| f.chars().map(|f| f.to_string()).collect_vec())
        .collect_vec();
    Some(
        input
            .lines()
            .collect_vec()
            .into_par_iter()
            .enumerate()
            .map(|(y, l)| {
                let mut x = 0;
                l.chars()
                    .group_by(|x| x.is_ascii_digit())
                    .into_iter()
                    .map(|(_ge0, group)| {
                        let g = group.collect_vec();
                        let len = g.len();

                        let num = g.iter().map(|c| c.to_string()).join("");

                        let mut part = false;

                        if num.parse::<i32>().is_ok() {
                            part = ((x.max(1) - 1)..(x + len + 1))
                                .cartesian_product((y.max(1) - 1)..(y + 2))
                                .map(|(x, y)| {
                                    lines.get(y).map_or(false, |column| {
                                        column.get(x).map_or(false, |item| {
                                            *item != "." && !item.parse::<i32>().is_ok()
                                        })
                                    })
                                })
                                .any(|f| f);
                        }
                        x += len;

                        Run {
                            is_part: part,
                            num,

                        }
                    })
                    .filter(|r| r.is_part)
                    .map(|r| r.num.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .flatten()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .lines()
        .map(|f| f.chars().map(|f| f.to_string()).collect_vec())
        .collect_vec();
    Some(
        input
            .lines()
            //  .collect_vec()
            // .into_par_iter()
            .enumerate()
            .map(|(y, c)| {
                c.chars()
                    .map(|f| f.to_string())
                    .collect_vec()
                    .iter()
                    .enumerate()
                    .filter(|(_x, c)| *c == "*")
                    .map(|(x, _)| (x, y))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|(x, y)| {
                //println!("({x},{y})");
                (y - 1..y + 2)
                    .map(|y| {
                        let mut curr_x = 0;
                        lines.get(y).map_or(None, |f| {
                            //println!("{:?}",f);
                            Some(
                                f.iter()
                                    .group_by(|f| {
                                        f.chars()
                                            .next()
                                            .expect("string should have one char")
                                            .is_ascii_digit()
                                    })
                                    .into_iter()
                                    .map(|(g0, group)| {
                                        let g = group.collect_vec();
                                        let len = g.len();

                                        let num =
                                            g.iter().map(|c| c.to_string()).join("").parse().ok();
                                        curr_x += len;
                                        RunP2 {
                                            pos: (curr_x - len, y),
                                            num,
                                            len: len - 1,
                                            is_num: g0,
                                        }
                                    })
                                    .filter(|r| r.is_num)
                                    .filter(|r| x - 1 <= r.pos.0 + r.len && r.pos.0 <= x + 1)
                                    .collect_vec(),
                            )
                        })
                    })
                    .filter_map(|o| o.map(|v| v.iter().map(|r| r.num.unwrap()).collect_vec()))
                    .collect_vec()
            })
            .map(|v| {
                v.iter()
                    .filter(|v| !v.is_empty())
                    .map(|v| v[0])
                    .collect_vec()
            })
            .filter(|f| f.len() == 2)
            .map(|v| v[0] * v[1])
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
