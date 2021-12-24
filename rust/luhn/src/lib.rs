/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.chars().filter(|c| !c.is_ascii_whitespace()).count() <= 1 {
        return false;
    }

    if !code
        .chars()
        .all(|c| c.is_ascii_whitespace() || c.is_ascii_digit())
    {
        return false;
    }

    let sum: u32 = code
        .chars()
        .filter(|c| c.is_ascii_digit())
        .rev()
        .enumerate()
        .map(|(i, c)| {
            let mut digit = c.to_digit(10).unwrap();
            if i % 2 != 0 {
                digit *= 2;
                if digit > 9 {
                    digit -= 9;
                }
            }
            digit
        })
        .sum();

    sum % 10 == 0
}
