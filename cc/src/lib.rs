use chrono::prelude::*;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct ImportantEvent {
    name: String,
    date: Date<Utc>,
}

pub trait Deadline {
    fn lapsed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn lapsed(&self) -> bool {
        self.date < Utc::now().date()
    }
}

impl ImportantEvent {
    pub fn new(name: String, date: Date<Utc>) -> ImportantEvent {
        ImportantEvent {
            name,
            date
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
