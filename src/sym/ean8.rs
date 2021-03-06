//! This module provides types for EAN-8 barcodes, which are EAN style barcodes for smaller
//! packages on products like cigaretts, chewing gum, etc.

use ::sym::Parse;
use ::sym::EncodedBarcode;
use ::sym::helpers;
use ::sym::ean13::EAN_ENCODINGS;
use ::sym::ean13::EAN_LEFT_GUARD;
use ::sym::ean13::EAN_MIDDLE_GUARD;
use ::sym::ean13::EAN_RIGHT_GUARD;
use std::ops::Range;
use std::char;

/// The EAN-8 barcode type.
pub struct EAN8 {
    data: Vec<u8>,
}

impl EAN8 {
    /// Creates a new barcode.
    /// Returns Result<EAN8, String> indicating parse success.
    pub fn new(data: String) -> Result<EAN8, String> {
        match EAN8::parse(data) {
            Ok(d) => {
                let digits = d.chars().map(|c| c.to_digit(10).expect("Unknown character") as u8).collect();
                Ok(EAN8{data: digits})
            }
            Err(e) => Err(e),
        }
    }

    /// Returns the data as was passed into the constructor.
    pub fn raw_data(&self) -> &[u8] {
        &self.data[..]
    }

    /// Calculates the checksum digit using a weighting algorithm.
    pub fn checksum_digit(&self) -> u8 {
        let mut odds = 0;
        let mut evens = 0;

        for (i, d) in self.data.iter().enumerate() {
            match i % 2 {
                1 => { evens += *d }
                _ => { odds += *d }
            }
        }

        match 10 - (((odds * 3) + evens) % 10) {
            10    => 0,
            n @ _ => n,
        }
    }

    fn number_system_digits(&self) -> &[u8] {
        &self.data[0..2]
    }

    fn number_system_encoding(&self) -> Vec<u8> {
        let mut ns = vec![];

        for d in self.number_system_digits() {
            ns.extend(self.char_encoding(0, &d).iter().cloned());
        }

        ns
    }

    fn checksum_encoding(&self) -> Vec<u8> {
        self.char_encoding(2, &self.checksum_digit()).to_vec()
    }

    fn char_encoding(&self, side: usize, d: &u8) -> [u8; 7] {
        EAN_ENCODINGS[side][*d as usize]
    }

    fn left_digits(&self) -> &[u8] {
        &self.data[2..4]
    }

    fn right_digits(&self) -> &[u8] {
        &self.data[4..]
    }

    fn left_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self.left_digits()
            .iter()
            .map(|d| self.char_encoding(0, &d))
            .collect();

        slices.iter().flat_map(|e| e.iter()).cloned().collect()
    }

    fn right_payload(&self) -> Vec<u8> {
        let slices: Vec<[u8; 7]> = self.right_digits()
            .iter()
            .map(|d| self.char_encoding(2, &d))
            .collect();

        slices.iter().flat_map(|e| e.iter()).cloned().collect()
    }

    /// Encodes the barcode.
    /// Returns a Vec<u8> of binary digits.
    pub fn encode(&self) -> EncodedBarcode {
        helpers::join_vecs(&[
            EAN_LEFT_GUARD.to_vec(), self.number_system_encoding(),
            self.left_payload(), EAN_MIDDLE_GUARD.to_vec(), self.right_payload(),
            self.checksum_encoding(), EAN_RIGHT_GUARD.to_vec()][..])
    }
}

impl Parse for EAN8 {
    /// Returns the valid length of data acceptable in this type of barcode.
    fn valid_len() -> Range<u32> {
        7..8
    }

    /// Returns the set of valid characters allowed in this type of barcode.
    fn valid_chars() -> Vec<char> {
        (0..10).into_iter().map(|i| char::from_digit(i, 10).unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use ::sym::ean8::*;
    use std::char;

    fn collapse_vec(v: Vec<u8>) -> String {
        let chars = v.iter().map(|d| char::from_digit(*d as u32, 10).unwrap());
        chars.collect()
    }

    #[test]
    fn new_ean8() {
        let ean8 = EAN8::new("1234567".to_string());

        assert!(ean8.is_ok());
    }

    #[test]
    fn invalid_data_ean8() {
        let ean8 = EAN8::new("1234er123412".to_string());

        assert!(ean8.is_err());
    }

    #[test]
    fn invalid_len_ean8() {
        let ean8 = EAN8::new("1111112222222333333".to_string());

        assert!(ean8.is_err());
    }

    #[test]
    fn ean8_raw_data() {
        let ean8 = EAN8::new("1234567".to_string()).unwrap();

        assert_eq!(ean8.raw_data(), &[1,2,3,4,5,6,7]);
    }

    #[test]
    fn ean8_encode() {
        let ean81 = EAN8::new("5512345".to_string()).unwrap(); // Check digit: 7
        let ean82 = EAN8::new("9834651".to_string()).unwrap(); // Check digit: 3

        assert_eq!(collapse_vec(ean81.encode()), "1010110001011000100110010010011010101000010101110010011101000100101".to_string());
        assert_eq!(collapse_vec(ean82.encode()), "1010001011011011101111010100011010101010000100111011001101010000101".to_string());
    }

    #[test]
    fn ean8_checksum_calculation() {
        let ean81 = EAN8::new("4575678".to_string()).unwrap(); // Check digit: 8
        let ean82 = EAN8::new("9534763".to_string()).unwrap(); // Check digit: 9

        assert_eq!(ean81.checksum_digit(), 8);
        assert_eq!(ean82.checksum_digit(), 9);
    }
}
