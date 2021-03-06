use anyhow::Result;
use bswp::pattern::{Pattern, Predicate};
use std::convert::TryFrom;

pub fn swap_from_str(s: &str) -> Result<(Pattern, Predicate)> {
    let separator = ',';
    let sp = s.splitn(4, |e| e == separator);
    let sp: Vec<&str> = sp.collect();
    match sp.as_slice() {
        [value, mask, periodicity, offset] => {
            let value: u8 = parse_hexa_byte(value)?;
            let mask: u8 = parse_hexa_byte(mask)?;
            let periodicity: usize = periodicity.parse()?;
            let offset: usize = offset.parse()?;
            Ok((
                Pattern::new(value).with_mask(mask),
                Predicate::new()
                    .with_periodicity(periodicity)
                    .with_offset(offset),
            ))
        }
        _ => Err(anyhow::anyhow!(
            "invalid format: '{}', expected <value>,<mask>,<periodicity>,<offset>",
            s
        )),
    }
}

fn parse_hexa_byte(s: &str) -> anyhow::Result<u8> {
    let radix = s.trim_start_matches("0x");
    u8::try_from(usize::from_str_radix(radix, 16)?)
        .or_else(|_| Err(anyhow::anyhow!("failed to cast {} to u8", s)))
}

#[cfg(test)]
mod tests {
    use crate::utils::{parse_hexa_byte, swap_from_str};

    #[test]
    fn test_swap_from_str_valid() {
        let valid = "0x42,0xFF,2,1";
        let swap = swap_from_str(valid);
        assert!(swap.is_ok());
        let (pattern, predicate) = swap.unwrap();
        assert_eq!(pattern.value, 0x42);
        assert_eq!(pattern.mask, 0xFF);
        assert_eq!(predicate.periodicity, 2);
        assert_eq!(predicate.offset, 1);
    }

    #[test]
    fn test_parse_hexa_byte_valid_base16() {
        let valid = "0x42";
        let parsed = parse_hexa_byte(valid);
        assert!(parsed.is_ok());
    }

    #[test]
    fn test_parse_hexa_byte_invalid_format() {
        let invalid = "0xG42";
        let parsed = parse_hexa_byte(invalid);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_hexa_byte_invalid_bound() {
        let invalid = "0xFFFF";
        let parsed = parse_hexa_byte(invalid);
        assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_hexa_byte_invalid_base10() {
        let invalid = "255";
        let parsed = parse_hexa_byte(invalid);
        assert!(parsed.is_err());
    }
}
