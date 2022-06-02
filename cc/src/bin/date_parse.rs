use std::num::ParseIntError;
use chrono::{NaiveDate};

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {

    #[derive(Debug)]
    struct ParseDate {
        year: i32,
        month: u32,
        date: u32
    }

    // decompose based on known delimiters
    // and analyse each part for either textual or numeric
    let mut parts = text.trim().splitn(3, &['.','/','-'])
        .into_iter()
        .map(|s| (s.len(), s, usize::from_str_radix(s,10)) )
        .collect::<Vec<(usize,&str,Result<usize,ParseIntError>)>>();

    // construct output structure
    let mut date = ParseDate { year:0,month:0,date:0 };
    // reverse sort so the year comes first
    // we imply here year in in YYYY format and not YY
    // expected order
    // YEAR:4, TXT MONTH:3, MONTH OR DATE
    parts.sort_by(|a,b| b.0.cmp(&a.0) );

    parts.into_iter()
        .for_each( |el| {
        match el {
            // year
            (4, _, res) if res.is_ok() => date.year = res.unwrap() as i32,
            // textual month, we check the non-numeric error
            (3, month, res) if res.is_err() => {
                date.month = match month {
                    "Jan" => 1,
                    "Feb" => 2,
                    "Mar" => 3,
                    "Apr" => 4,
                    "Jun" => 5,
                    "Jul" => 6,
                    // if garbage store 0 which will result to None
                    _ => { println!( "Invalid month: {:?}!!", &month ); 0 }
                }
            },
            // either month or day and is numeric
            (2, _, res) if res.is_ok() => {
                let num = res.unwrap();
                match &mut date {
                    ParseDate{year: _ , month: m, date: d} => {
                        match (&m, &d, num) {
                            (0, 1..=31, month@ 1..=12) => *m = month as u32,
                            (1..=12, 0, date@ 1..=31) => *d = date as u32,
                            (0, 0, month@ 1..=12) => *m = month as u32,
                            (0, 0, date@ 1..=31) => *d = date as u32,
                            _ => (),
                        }
                    }
                    _ => println!( "Invalid numeric: {:?}!!",num ),
                }
            },
            (_,_,_) => (),
        }
    });

    match date {
        ParseDate { year:0i32, ..} |
        ParseDate { year:_, month:0u32, ..} |
        ParseDate { year:_, month:_, date:0u32 } => None,
        ParseDate { year: y, month: m, date: d } => Some( NaiveDate::from_ymd( y, m, d) )
    }
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "Fab.05.2021",
        "Mar.35.2021",
        "Mar.05.21",
        "Mar.2021.05",
        "05.2021.Mar",
        "2021.05.Mar",
        "2021.Mar.05",
        "12-2010-31",
        "2010-12-31",
        "31-12-2010",
        "31-13-2010",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }

}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ymd_hyphen() {
        assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd(2010, 12, 11)))
    }

    #[test]
    fn ymd_slash() {
        assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd(1999, 3, 2)))
    }

    #[test]
    fn dmy_dot() {
        assert_eq!(flexible_date_parse("01.Mar.2021"), Some(NaiveDate::from_ymd(2021, 3, 1)))
    }

    #[test]
    fn mdy_dot() {
        assert_eq!(flexible_date_parse("Apr.05.2021"), Some(NaiveDate::from_ymd(2021, 4, 5)))
    }

    #[test]
    fn invalid() {
        assert_eq!(flexible_date_parse("not a date"), None)
    }
}