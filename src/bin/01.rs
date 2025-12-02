advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut res = 0;
    let mut acc = 50;
    let bytes = input.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    unsafe {
        while i < len {
            let dir = *bytes.get_unchecked(i);
            i += 1;

            let mut amount = 0;
            while i < len && *bytes.get_unchecked(i) != b'\n' {
                amount = amount * 10 + (*bytes.get_unchecked(i) - b'0') as i32;
                i += 1;
            }
            i += 1;

            let sign = 1 - 2 * (dir == b'L') as i32;
            acc += sign * amount;

            let mod_amount = acc.rem_euclid(100);
            res += (mod_amount == 0) as u64;
            acc = mod_amount;
        }
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = 0;
    let mut acc = 50;
    let bytes = input.as_bytes();
    let mut i = 0;
    let len = bytes.len();

    unsafe {
        while i < len {
            let dir = *bytes.get_unchecked(i);
            i += 1;

            let mut amount = 0;
            while i < len && *bytes.get_unchecked(i) != b'\n' {
                amount = amount * 10 + (*bytes.get_unchecked(i) - b'0') as i32;
                i += 1;
            }
            i += 1;

            let sign = 1 - 2 * (dir == b'L') as i32;
            let no_mod_amount = acc + sign * amount;

            let l_crossed = (acc - 1).div_euclid(100) - (no_mod_amount - 1).div_euclid(100);
            let r_crossed = no_mod_amount.div_euclid(100);
            let is_l = (dir == b'L') as i32;
            let crossed_zero = is_l * l_crossed + (1 - is_l) * r_crossed;

            res += crossed_zero as u32;
            acc = no_mod_amount.rem_euclid(100);
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
