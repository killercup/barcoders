pub mod ean13;
pub mod ean8;
pub mod ean_supp;
pub mod code39;
pub mod helpers;

use std::ops::Range;
use std::iter::Iterator;

pub type EncodedBarcode = Vec<u8>;

pub trait Parse {
    fn valid_chars() -> Vec<char>;
    fn valid_len() -> Range<u32>;

    fn parse(data: String) -> Result<String, String> {
        let valid_chars = Self::valid_chars();
        let valid_len = Self::valid_len();
        let data_len = data.len() as u32;

        if data_len < valid_len.start || data_len > valid_len.end {
            return Err(format!("Data does not fit within range of {}-{}", valid_len.start, valid_len.end - 1));
        }

        let bad_char = data.chars().find(|&c| valid_chars.iter().find(|&vc| *vc == c).is_none());

        match bad_char {
            Some(c) => Err(format!("Invalid character: {}", c)),
            None => Ok(data),
        }
    }
}
