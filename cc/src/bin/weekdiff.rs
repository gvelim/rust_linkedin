mod graph_search;

use chrono::prelude::*;

fn main() {

}
fn parse_date(dt: String) -> NaiveDate {
    println!("{:?}",&dt);
    // Utc.datetime_from_str( &(dt+"00:00UTC"), "%F%R%Z" ).unwrap().date()
    NaiveDate::parse_from_str(&dt, "%F").unwrap()
}

fn weeks_between(a: String, b: String) -> i32 {
    let date_a = parse_date(a);
    let date_b = parse_date(b);

    ((date_b - date_a).num_days() / 7) as i32
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pos_diff() {
        assert_eq!(
            weeks_between("2022-03-31".to_string(),"2023-05-15".to_string() ),
            58
        );
    }
    #[test]
    fn test_neg_diff() {
        assert_eq!(
            weeks_between("2023-05-15".to_string(),"2022-03-31".to_string() ),
            -58
        );
    }
    #[test]
    fn test_no_diff() {
        assert_eq!(
            weeks_between("2023-05-15".to_string(),"2023-05-18".to_string() ),
            0
        );
    }
}