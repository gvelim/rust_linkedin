extern crate core;
use std::str::FromStr;

#[derive(Debug)]
struct ISBN {
    isbn: String,
}

#[derive(Debug)]
enum ISBN_Err {
    InputTooLong,
    InputTooSort,
    FailedChecksum
}

impl FromStr for ISBN {
    type Err = ISBN_Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sum : u32 = s.splitn(5,'-')
            .fold("".to_string(),|c, i| c + i)
            .chars()
            .map(|s| if s.is_ascii_digit() { u32::from(s)-48 } else {panic!("non digit")} )
            .enumerate()
            .map(|(i,n)| if i % 2 == 1 { n*3 } else {n} )
            .sum();

        if (10-10-sum % 10) == 0 {
            Ok(ISBN { isbn: s.to_string() })
        } else {
            Err(ISBN_Err::FailedChecksum)
        }
    }
}

fn main() {
    let isbn = ISBN::from_str("978-3-16-148410-0");
    println!("{:?}",isbn);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_sort() {
        todo!()
    }
    #[test]
    fn test_input_long() {
        todo!()
    }
    #[test]
    fn test_checksup() {
        todo!()
    }
}