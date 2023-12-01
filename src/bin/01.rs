
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| {
        let mut s = l.as_bytes().iter().filter(|x| x.is_ascii_digit()).peekable();
        (*s.peek().unwrap() - b'0') as u32 * 10 + (*s.last().unwrap() - b'0') as u32
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    const NUMS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    Some(input
        .lines()
        .map(|l| {

            let mut pos_l = l
                .as_bytes()
                .iter()
                .position(|c| c.is_ascii_digit())
                .unwrap();
            let mut pos_r = l
                .as_bytes()
                .iter()
                .rposition(|c| c.is_ascii_digit())
                .unwrap();
            let mut left = 10 * (l.as_bytes()[pos_l] - b'0') as u32;
            let mut right = (l.as_bytes()[pos_r] - b'0') as u32;
            let found_rpos = pos_r;
            let found_lpos = pos_l;
            for (i, x) in NUMS.iter().enumerate(){
                if let Some(pos) = l[..found_lpos].find(x) {
                    if pos < pos_l {
                        pos_l = pos;
                        left = 10 * (i + 1) as u32;
                    }
                }
            }
            for (idx, num) in NUMS.iter().enumerate() {
                if let Some(pos) = l[found_rpos..].rfind(num) {
                    if pos + found_rpos > pos_r {
                        pos_r = pos + found_rpos;
                        right = (idx + 1) as u32;
                    }
                }
            }

            left+right
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(242));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
