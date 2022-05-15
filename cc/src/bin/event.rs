
fn main() {
    todo!()
}

#[cfg(test)]
mod test {
    use chrono::{prelude::*, Duration};
    use rust_linkedin::*;

    #[test]
    fn test_not_lapsed() {
        let dt = Utc::today() + Duration::days(30);
        let event = ImportantEvent::new( "George".to_string(),dt );

        assert_eq!( event.lapsed(), false);
    }
    #[test]
    fn test_lapsed() {
        let dt = Utc.ymd(2022,3,31);
        let event = ImportantEvent::new( "George".to_string(), dt );

        assert_eq!( event.lapsed(), true);
    }
}