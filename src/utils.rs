use crate::{Error, Precision};

/// Converts the digits of a string into
/// eastings, northings and precision.
pub fn digits(s: &str) -> Result<(u32, u32, Precision), Error> {
    // Error is s length is over 10 or not even;
    if s.len() > 10 || (s.len() % 2) != 0 {
        return Err(Error::ParseError(format!(
            "{} is not a valid number of digits. Supported values: 0, 2, 4, 6, 8, 10.",
            s.len()
        )));
    }

    let (eastings, northings) = {
        if s.is_empty() {
            (0, 0)
        } else {
            let (e, n) = s.split_at(s.len() / 2);

            (
                e.parse()
                    .map_err(|e| Error::ParseError(format!("{:?}", e)))?,
                n.parse()
                    .map_err(|e| Error::ParseError(format!("{:?}", e)))?,
            )
        }
    };

    let precision = match s.len() {
        0 => Precision::_100Km,
        2 => Precision::_10Km,
        4 => Precision::_1Km,
        6 => Precision::_100M,
        8 => Precision::_10M,
        10 => Precision::_1M,
        _ => {
            return Err(Error::InvalidPrecision(format!(
                "{} is not a valid number of digits",
                s.len()
            )))
        }
    };

    Ok((
        eastings * precision.metres(),
        northings * precision.metres(),
        precision,
    ))
}

/// Removes all non-alphanumeric characters from string
/// and converts to uppercase for parsing.
pub fn trim_string(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{
        constants::*,
        utils::{digits, trim_string},
        Error, Precision,
    };

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
            Err(Error::ParseError(
                "3 is not a valid number of digits. Supported values: 0, 2, 4, 6, 8, 10."
                    .to_string()
            ))
        );

        // Reject non numbers
        assert_eq!(
            digits("ab"),
            Err(Error::ParseError(
                "ParseIntError { kind: InvalidDigit }".to_string()
            ))
        )
    }

    #[test]
    fn trim_strings() {
        assert_eq!(trim_string("so 14 5"), "SO145");
        assert_eq!(trim_string("So 222"), "SO222");
        assert_eq!(trim_string(" @ @ "), "@@");
    }
}
