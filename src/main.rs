use std::char;
/// Excel Index Problem
///
/// Problem Statement
///
/// Given a number, one per line on the standard input, print the corresponding "Excel"
/// index. For example 1 would print A, 3 would print C 27 would print AA, etc.
///
/// Algorithm
///
/// Remove chunks of 26 (base 10 to base 26 conversion) but allow for an offset of 1
/// because the number system starts at 1 not 0. eg. A == 1, AA = 27 (26 + 1)
use std::io::{self, BufRead};
use std::num::ParseIntError;

fn excel_index(index: u32) -> String {
    let mut quotient = index;
    let mut excel_index = String::new();

    loop {
        // Subtract 1 because A is equal to 1 not 0
        let remainder = (quotient - 1) % 26;
        // By subtracting the remainder we're removing "1" digit's worth from
        // the quotient which handles A = 1 for the remaining value
        quotient = (quotient - remainder) / 26;

        // We use insert to prepend characters
        excel_index.insert(
            0,
            char::from_u32(65 + remainder).expect("error converting value to character"),
        );

        if quotient == 0 {
            break;
        }
    }

    excel_index
}

fn main() -> Result<(), ParseIntError> {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Error parsing stdin");
        let index = line.trim().parse()?;
        println!("{}", excel_index(index));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::excel_index;

    #[test]
    fn test_excel_index() {
        assert_eq!("A", excel_index(1));
        assert_eq!("C", excel_index(3));
        assert_eq!("Z", excel_index(26));
        assert_eq!("AA", excel_index(27));
        assert_eq!("AH", excel_index(34));
        assert_eq!("XFD", excel_index(16384));
    }
}
