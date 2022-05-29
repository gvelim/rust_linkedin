use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
struct RGB(u8, u8, u8);

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for RGB {
    fn r(&self) -> u8 {
        self.0
    }

    fn g(&self) -> u8 {
        self.1
    }

    fn b(&self) -> u8 {
        self.2
    }
}

#[derive(Debug)]
enum RGBError {
    TooShort,
    InvalidLiterals,
    NoLeadingHash,
    NotAHexCode,
    OutOfBounds,
}

impl Display for RGB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2) )
    }
}

impl FromStr for RGB {
    type Err = RGBError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rgb) = s.strip_prefix("#") {

            let vch: Vec<Option<u32>> =  rgb.chars().map(|c| c.to_digit(16) ).collect();

            if vch.contains(&None) {
                return Err(RGBError::NotAHexCode)
            }
            if vch.len() != 6 {
                return Err(RGBError::TooShort)
            }

            let output: Vec<u8> = vch.chunks(2)
                .map(|ch| {
                    ((ch[0].unwrap() as u8) << 4) + ch[1].unwrap() as u8
                })
                .collect();

            Ok(RGB(output[0],output[1],output[2]))
        } else {
            Err(RGBError::NoLeadingHash)
        }

    }
}

fn main() {
    let colour = RGB(255,255,255);
    println!("{}",colour);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn every_color() {
        let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

        for ((r, g), b) in colors {
            let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
            let color: RGB = hex.parse().unwrap();
            assert_eq!(hex, format!("{}", color));
        }
    }

    #[test]
    #[should_panic]
    fn too_short () {
        let _: RGB = "#1234".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn not_a_hex_code () {
        let _: RGB = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_literals () {
        let _: RGB = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn no_leading_hash() {
        let _: RGB = "#aabbcc".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let _: RGB = "#00gg00".parse().unwrap();
    }

}