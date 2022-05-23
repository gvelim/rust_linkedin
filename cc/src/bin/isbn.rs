extern crate core;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ISBN {
    isbn: String,
}

#[derive(Debug, PartialEq)]
enum ISBNErr {
    InputTooLong,
    InputTooSort,
    FailedChecksum
}

impl FromStr for ISBN {
    type Err = ISBNErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sum: u32 = s.chars()
            .take(15)
            .filter_map( |s| s.to_digit(10) )
            .enumerate()
            .map(|(i, n)| if i % 2 == 1 { n * 3 } else { n })
            .sum();

        match s.len() {
            0..=16 => Err(ISBNErr::InputTooSort),
            17 => {
                let checksum = s.chars().last().unwrap().to_digit(10).unwrap();
                let calc_csum = (10 - sum % 10) % 10;
                if calc_csum == checksum {
                    Ok(ISBN { isbn: s.to_string() })
                } else {
                    Err(ISBNErr::FailedChecksum)
                }
            },
            _ => Err(ISBNErr::InputTooLong),
        }
    }
}

fn main() {
    let isbn = ISBN::from_str("978-0-306-40615-7");
    println!("{:?}",isbn);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_sort() {
        assert_eq!(
            ISBN::from_str("978-3-16-14840-0"),
            Err(ISBNErr::InputTooSort)
        )
    }
    #[test]
    fn test_input_long() {
        assert_eq!(
            ISBN::from_str("978-3-16-1484110-0"),
            Err(ISBNErr::InputTooLong)
        )
    }
    #[test]
    fn test_checksup() {
        let tests = [
            ("978-0-306-40615-7",Ok(ISBN { isbn:"978-0-306-40615-7".to_string()})),
            ("978-3-16-148410-0",Ok(ISBN { isbn:"978-3-16-148410-0".to_string()})),
            ("978-1-86-197876-9",Ok(ISBN { isbn:"978-1-86-197876-9".to_string()})),
            ("978-0-306-40615-1",Err(ISBNErr::FailedChecksum)),
            ("978-3-16-148410-1",Err(ISBNErr::FailedChecksum)),
            ("978-1-86-197876-1",Err(ISBNErr::FailedChecksum))
        ];
        tests.into_iter()
            .for_each( |(inp,out)| {
                let isbn = ISBN::from_str(inp);
                assert_eq!(isbn, out)
            })
    }
}