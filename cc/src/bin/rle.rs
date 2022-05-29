
mod run_length_encoding {

    pub fn encode(text: &str) -> String {
        let mut out : Vec<(u8,char)> = vec![];

        let mut cur : (u8,char) = (1, text.chars().nth(0).unwrap());
        text.chars()
            .skip(1)
            .for_each(|c| {
                if c == cur.1 && cur.0 < 9 {
                    cur.0 += 1;
                } else {
                    out.push(cur);
                    cur = (1u8,c);
                }
            });
        out.push(cur);

        out.into_iter()
            .fold(String::new(), |s, (n, c)| s + &format!("{}{}",n,c) )
    }

    pub fn decode(text: &str) -> String {
        let v : Vec<char> = text.chars().collect();
        let mut output = String::new();
        v.chunks(2)
            .map(|s| (s[0].to_digit(10).unwrap(), s[1]))
            .for_each(|(n, c)| {
                (0..n).for_each( |_| output.push(c))
            });
        output
    }
}

fn main() {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn abc() {
        use run_length_encoding::*;

        assert_eq!(encode("abc"), "1a1b1c");
    }

    #[test]
    fn round_trip() {
        use run_length_encoding::*;

        let input = "LinkedIn";
        println!("{}", encode(input));
        assert_eq!(decode(&encode(input)), input);
    }

    #[test]
    fn long_run() {
        use run_length_encoding::*;

        let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
        assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
    }
}