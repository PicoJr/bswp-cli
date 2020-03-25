use bswp::pattern::{BytePattern, Locality};
use bswp::Swap;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
enum ParsingHexaError {
    InvalidByteFormat,
    InvalidBound,
}

#[derive(Debug, PartialEq)]
enum ParsingError {
    InvalidSwapFormat,
    InvalidValue(ParsingHexaError),
    InvalidMask(ParsingHexaError),
    InvalidPeriodicity,
    InvalidOffset,
}

fn swap_from_str(s: &str) -> Result<Swap, ParsingError> {
    let separator = ',';
    let sp = s.splitn(4, |e| e == separator);
    let sp: Vec<&str> = sp.collect();
    match sp.as_slice() {
        [value, mask, periodicity, offset] => {
            let value: u8 = parse_hexa_byte(value).or_else(|e| Err(ParsingError::InvalidValue(e)))?;
            let mask: u8 = parse_hexa_byte(mask).or_else(|e| Err(ParsingError::InvalidMask(e)))?;
            let periodicity: usize = periodicity.parse().or_else(|_| Err(ParsingError::InvalidPeriodicity))?;
            let offset: usize = offset.parse().or_else(|_| Err(ParsingError::InvalidOffset))?;
            Ok((
                BytePattern::new(value, mask),
                Locality::new(periodicity, offset),
            ))
        }
        _ => Err(ParsingError::InvalidSwapFormat),
    }
}

fn parse_hexa_byte(s: &str) -> Result<u8, ParsingHexaError> {
    let radix = s.trim_start_matches("0x");
    u8::try_from(usize::from_str_radix(radix, 16).or_else(|_| Err(ParsingHexaError::InvalidByteFormat))?)
        .or_else(|_| Err(ParsingHexaError::InvalidBound))
}

#[cfg(test)]
mod tests {
    use crate::utils::{parse_hexa_byte, ParsingError, ParsingHexaError, swap_from_str};

    #[test]
    fn test_swap_from_str_valid() {
        let valid = "0x42,0xFF,2,1";
        let swap = swap_from_str(valid);
        assert!(swap.is_ok());
        let (pattern, locality) = swap.unwrap();
    }

    #[test]
    fn test_parse_hexa_byte_valid_base16() {
        let valid = "0x42";
        let parsed = parse_hexa_byte(valid);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_parse_hexa_byte_invalid_format() {
        let valid = "0xG42";
        let parsed = parse_hexa_byte(valid);
        assert!(parsed.is_err());
        assert_eq!(parsed.unwrap_err(), ParsingHexaError::InvalidByteFormat)
    }

    #[test]
    fn test_parse_hexa_byte_invalid_bound() {
        let valid = "0xFFFF";
        let parsed = parse_hexa_byte(valid);
        assert!(parsed.is_err());
        assert_eq!(parsed.unwrap_err(), ParsingHexaError::InvalidBound)
    }

    #[test]
    fn test_parse_hexa_byte_invalid_base10() {
        let valid = "255";
        let parsed = parse_hexa_byte(valid);
        assert!(parsed.is_err());
        assert_eq!(parsed.unwrap_err(), ParsingHexaError::InvalidBound)
    }
}
