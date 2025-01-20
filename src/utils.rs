use crate::{error::ParseError, Error, Precision};

/// Converts the digits of a string into
/// eastings, northings and precision.
pub fn digits(s: &str) -> Result<(u32, u32, Precision), Error> {
    // Error if s length is over 10 or not even;
    let length = s.len();
    if length > 10 || (length % 2) != 0 {
        return Err(Error::ParseError(ParseError::InvalidPrecision(
            length as u32,
        )));
    }

    let (eastings, northings) = {
        if s.is_empty() {
            (0, 0)
        } else {
            let (e, n) = s.split_at(length / 2);

            (
                e.parse()
                    .map_err(|e| Error::ParseError(ParseError::ParseInt(e)))?,
                n.parse()
                    .map_err(|e| Error::ParseError(ParseError::ParseInt(e)))?,
            )
        }
    };

    let precision = match length {
        0 => Precision::_100Km,
        2 => Precision::_10Km,
        4 => Precision::_1Km,
        6 => Precision::_100M,
        8 => Precision::_10M,
        10 => Precision::_1M,
        _ => {
            return Err(Error::ParseError(ParseError::InvalidPrecision(
                length as u32,
            )))
        }
    };

    Ok((
        eastings * precision.metres(),
        northings * precision.metres(),
        precision,
    ))
}

#[cfg(test)]
mod test {
    use crate::{constants::*, error::ParseError, utils::digits, Error, Precision};

    #[test]
    fn parse_valid_digits() {
        assert_eq!(digits(""), Ok((0, 0, Precision::_100Km)));
        assert_eq!(digits("12"), Ok((_10KM, 20_000, Precision::_10Km)));
        assert_eq!(digits("1234"), Ok((12_000, 34_000, Precision::_1Km)));
        assert_eq!(digits("123456"), Ok((12_300, 45_600, Precision::_100M)));
        assert_eq!(digits("12345678"), Ok((12_340, 56_780, Precision::_10M)));
        assert_eq!(digits("0123456789"), Ok((01_234, 56_789, Precision::_1M)));
    }

    #[test]
    fn reject_invalid_digits() {
        // Reject wrong length
        assert_eq!(
            digits("123"),
            Err(Error::ParseError(ParseError::InvalidPrecision(3)))
        );

        // Reject non numbers
        assert_eq!(
            digits("ab")
                .expect_err("Letters should not be able to be parsed as integers")
                .to_string(),
            "invalid digit found in string".to_string()
        )
    }
}
