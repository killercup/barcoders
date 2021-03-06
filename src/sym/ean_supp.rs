//! This module provides types for encoding supplemental 2-digit and 5-digit EAN barcodes.
//! Supplemental EAN-2 barcodes are used in magazines and newspapers to indicate issue number and
//! EAN-5 barcodes are often used to indicate the suggested retail price of books.

use ::sym::Parse;
use ::sym::EncodedBarcode;
use ::sym::ean13::EAN_ENCODINGS;
use ::sym::helpers;
use std::ops::Range;
use std::char;

pub const EANSUPP_LEFT_GUARD: [u8; 4] = [1,0,1,1];

/// Maps parity (odd/even) for the EAN-5 barcodes based on the check digit.
const EAN5_PARITY: [[usize; 5]; 10] = [
    [0,0,1,1,1],
    [1,0,1,0,0],
    [1,0,0,1,0],
    [1,0,0,0,1],
    [0,1,1,0,0],
    [0,0,1,1,0],
    [0,0,0,1,1],
    [0,1,0,1,0],
    [0,1,0,0,1],
    [0,0,1,0,1],
];

/// Maps parity (odd/even) for the EAN-2 barcodes based on the check digit.
const EAN2_PARITY: [[usize; 5]; 4] = [
    [0,0,0,0,0],
    [0,1,0,0,0],
    [1,0,0,0,0],
    [1,1,0,0,0],
];

/// The Supplemental EAN barcode type.
pub enum EANSUPP {
    EAN2 {
        data: Vec<u8>,
    },
    EAN5 {
        data: Vec<u8>,
    },
}

impl EANSUPP {
    /// Creates a new barcode.
    /// Returns Result<EANSUPP, String> indicating parse success.
    /// Either a EAN2 or EAN5 variant will be returned depending on
    /// the length of `data`.
    pub fn new(data: String) -> Result<EANSUPP, String> {
        match EANSUPP::parse(data) {
            Ok(d) => {
                let digits: Vec<u8> = d.chars().map(|c| c.to_digit(10).expect("Unknown character") as u8).collect();

                match digits.len() {
                    2 => Ok(EANSUPP::EAN2{data: digits}),
                    5 => Ok(EANSUPP::EAN5{data: digits}),
                    n @ _ => Err(format!("Invalid supplemental length: {}", n)),
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[u8] {
        match *self {
            EANSUPP::EAN2{data: ref d} => &d[..],
            EANSUPP::EAN5{data: ref d} => &d[..],
        }
    }

    fn char_encoding(&self, side: usize, d: &u8) -> [u8; 7] {
        EAN_ENCODINGS[side][*d as usize]
    }
 
    /// Calculates the checksum digit using modified a modulo-10 weighting
    /// algorithm. This only makes sense for EAN5 barcodes.
    pub fn checksum_digit(&self) -> u8 {
        let mut odds = 0;
        let mut evens = 0;
        let data = self.raw_data();

        for (i, d) in data.iter().enumerate() {
            match i % 2 {
                1 => { evens += *d }
                _ => { odds += *d }
            }
        }

        match ((odds * 3) + (evens * 9)) % 10 {
            10    => 0,
            n @ _ => n,
        }
    }

    fn parity(&self) -> [usize; 5] {
        match *self {
            EANSUPP::EAN2{data: ref d} => {
                let modulo = ((d[0] * 10) + d[1]) % 4;
                EAN2_PARITY[modulo as usize]
            },
            EANSUPP::EAN5{data: ref _d} => {
                let check = self.checksum_digit() as usize;
                EAN5_PARITY[check]
            },
        }
    }

    fn payload(&self) -> Vec<u8> {
        let mut p = vec![];
        let slices: Vec<[u8; 7]> = self.raw_data()
            .iter()
            .zip(self.parity().iter())
            .map(|(d, s)| self.char_encoding(*s, &d))
            .collect();

        for (i, d) in slices.iter().enumerate() {
            if i > 0 {
                p.push(0);
                p.push(1);
            }

            p.extend(d.iter().cloned());
        }

        p
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> EncodedBarcode {
        helpers::join_vecs(&[
            EANSUPP_LEFT_GUARD.to_vec(), self.payload()][..])
    }
}

impl Parse for EANSUPP {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        2..5
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean_supp::*;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean2() {
        let ean2 = EANSUPP::new("12".to_string());

        assert!(ean2.is_ok());
    }

    #[test]
    fn new_ean5() {
        let ean5 = EANSUPP::new("12345".to_string());

        assert!(ean5.is_ok());
    }

    #[test]
    fn invalid_data_ean2() {
        let ean2 = EANSUPP::new("AT".to_string());

        assert!(ean2.is_err());
    }

    #[test]
    fn invalid_len_ean2() {
        let ean2 = EANSUPP::new("123".to_string());

        assert!(ean2.is_err());
    }

    #[test]
    fn ean2_raw_data() {
        let ean2 = EANSUPP::new("98".to_string()).unwrap();

        assert_eq!(ean2.raw_data(), &[9,8]);
    }

    #[test]
    fn ean5_raw_data() {
        let ean5 = EANSUPP::new("98567".to_string()).unwrap();

        assert_eq!(ean5.raw_data(), &[9,8,5,6,7]);
    }

    #[test]
    fn ean2_encode() {
        let ean21 = EANSUPP::new("34".to_string()).unwrap();

        assert_eq!(collapse_vec(ean21.encode()), "10110100001010100011".to_string());
    }

    #[test]
    fn ean5_encode() {
        let ean51 = EANSUPP::new("51234".to_string()).unwrap();

        assert_eq!(collapse_vec(ean51.encode()), "10110110001010011001010011011010111101010011101".to_string());
    }

}
