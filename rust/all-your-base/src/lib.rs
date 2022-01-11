use core::num;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    if number.is_empty() || number.iter().all(|n| n == &0) {
        return Ok(vec![0]);
    }

    let trimmed_number: Vec<u32> = number.iter().cloned().skip_while(|n| n == &0).collect();

    match from_base.cmp(&to_base) {
        Ordering::Equal => Ok(number.to_vec()),
        _ => {
            let decimal = to_decimal(trimmed_number, from_base)?;
            Ok(change_base(decimal, to_base))
        }
    }
}

fn to_decimal(number: Vec<u32>, from_base: u32) -> Result<u32, Error> {
    if let Some(x) = number.iter().find(|&&n| n >= from_base) {
        return Err(Error::InvalidDigit(*x));
    }

    Ok(number
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, n)| acc + n * from_base.pow(i as u32)))
}

fn change_base(number: u32, to_base: u32) -> Vec<u32> {
    let mut result = number;
    let mut res = Vec::new();

    while result != 0 {
        res.push(result % to_base);
        result /= to_base;
    }

    res.reverse();
    res
}
