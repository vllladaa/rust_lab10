fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let shift = (n % len + len) % len;
    let split_index = len - shift;
    let (first, second) = s.split_at(split_index as usize);
    format!("{}{}", second, first)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)|
                assert_eq!(
                    rotate(s.clone(), *n),
                    exp.to_string()
                )
            );
    }
}

fn main() {
    let s = "abcdefgh".to_string();
    let rotated = rotate(s, 2);
    println!("Rotated string: {}", rotated);
}