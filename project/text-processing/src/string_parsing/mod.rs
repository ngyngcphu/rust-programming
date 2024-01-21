use std::str::FromStr;

struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let r = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(RGB { r, g, b })
    }
}

pub fn convert_hex_color_to_rgb_color(input: &str) {
    match RGB::from_str(input) {
        Ok(rgb) => {
            println!(
                r"The RGB color code is: R: {} G: {} B: {}",
                rgb.r, rgb.g, rgb.b
            );
        }
        Err(_) => {
            eprintln!("{} is not a valid color hex code!", input);
        }
    }
}
