/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(&[' '][..], "");
    if code.len() <= 1 {
        return false;
    }
    let mut final_sum = 0;
    let base = 10;
    for (i, c) in code.chars().rev().enumerate() {
        match c.is_ascii_digit() {
            false => return false,
            _ => {
                let digit = c.to_digit(base).unwrap();
                match i % 2 {
                    1 => {
                        final_sum += digit * 2;
                        if digit * 2 > 9 {
                            final_sum -= 9;
                        }
                    },
                    _ => final_sum += digit
                }
            }
        }
    }
    return final_sum % 10 == 0;
}
