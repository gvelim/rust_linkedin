use chrono::prelude::*;

#[derive(Debug, PartialOrd, PartialEq)]
struct ImportantEvent {
    name: String,
    date: Date<Utc>,
}

trait Deadline {
    fn lapsed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn lapsed(&self) -> bool {
        self.date < Utc::now().date()
    }
}

impl ImportantEvent {
    fn new(name: String, date: Date<Utc>) -> ImportantEvent {
        ImportantEvent {
            name,
            date
        }
    }
}


fn main() {
    todo!()
}

#[cfg(test)]
mod test {
    use chrono::Duration;
    use super::*;

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