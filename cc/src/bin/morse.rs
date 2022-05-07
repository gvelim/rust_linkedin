
#[derive(Debug, PartialOrd, PartialEq)]
enum Pulse {
    Long,
    Short,
}
type Message = Vec<Letter>;
type Letter = Vec<Pulse>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        use Pulse::*;
        let mut out: Message = Vec::with_capacity(self.len());

        for c in self.chars() {
            let letter = match c {
                'A' | 'a' => vec![Long, Short],
                'B' | 'b' => vec![Short,Short],
                _ => continue,
            };
            out.push(letter);
        }
        out
    }
}

fn main() {
    let s = "AaBb".to_string();

    println!("{:?}", s.to_morse_code() );

}


#[cfg(test)]
mod tests {
    use crate::MorseCode;
    use crate::Pulse::*;

    #[test]
    fn test_morsecode() {
        let s = String::from("AaBb");
        assert_eq!(
            s.to_morse_code(),
            vec![
                vec![Long,Short],
                vec![Long,Short],
                vec![Short,Short],
                vec![Short,Short]
            ]
        );
    }
}
