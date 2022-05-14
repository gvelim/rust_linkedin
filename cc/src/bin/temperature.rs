#[derive(Debug,PartialEq)]
struct Temperature {
    degrees: f32,
    scale: Scale,
}

#[derive(Debug,PartialEq)]
enum Scale {
    Celcius,
    Farenheit,
}

impl Temperature {
    fn new(degrees: f32, scale: Scale) -> Temperature {
        Temperature { degrees, scale }
    }
    fn to_farenheit(&mut self) -> f32 {
        if self.scale == Scale::Celcius {
            self.degrees = self.degrees * 9.0/5.0 + 32.0;
            self.scale = Scale::Farenheit;
        }
        self.degrees
    }
    fn to_celcius(&mut self) -> f32 {
        if self.scale == Scale::Farenheit {
            self.scale = Scale::Celcius;
            self.degrees = (self.degrees - 32.0) * 5.0 / 9.0;
        }
        self.degrees
    }
}

fn main() {

}

#[cfg(test)]
mod test {
    use crate::Scale::{Celcius, Farenheit};
    use super::*;

    #[test]
    fn test_c2f() {
        let mut t = Temperature::new(0.0, Celcius);
        assert_eq!(t.to_farenheit(), 32.0);
        assert_eq!(t.to_celcius(), 0.0);
    }
    #[test]
    fn test_f2c() {
        let mut t = Temperature::new(32.0, Farenheit);
        assert_eq!(t.to_celcius(), 0.0f32);
        assert_eq!(t.to_farenheit(), 32.0f32);
    }
}