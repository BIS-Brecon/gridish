use crate::constants::*;
use crate::Error;
use crate::Precision;

/// A type wrapping u32 to allow bounds checking
/// and remapping to different precisions.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Metres(u32);

impl Metres {
    pub fn precision(&self, precision: Precision) -> Self {
        let remainder = self.0 % precision.metres();

        Self(self.0 - remainder)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }

    /// Returns the number of metres within the current 100Km square.
    /// Padded out to a valid grid reference format.
    pub fn padded(&self, precision: Precision) -> String {
        if precision.digits() == 0 {
            return "".to_string();
        } else {
            let metres = self.0 % _100KM;
            format!(
                "{:0width$}",
                metres / precision.metres(),
                width = precision.digits() / 2
            )
        }
    }
}

impl TryFrom<u32> for Metres {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value >= _500KM {
            Err(Error::OutOfBounds)
        } else {
            Ok(Self(value))
        }
    }
}

impl Into<u32> for Metres {
    fn into(self) -> u32 {
        self.0
    }
}

impl Into<f64> for Metres {
    fn into(self) -> f64 {
        f64::from(self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::constants::_500KM;
    use crate::coordinates::metres::Metres;
    use crate::Error;
    use crate::Precision;

    #[test]
    fn rejects_out_of_bounds() {
        // Test zero
        let metres: Result<Metres, Error> = 0.try_into();
        assert_eq!(Into::<u32>::into(metres.unwrap()), 0);

        // Test final allowed value
        let metres: Result<Metres, Error> = (_500KM - 1).try_into();
        assert_eq!(Into::<u32>::into(metres.unwrap()), (_500KM - 1));

        // Test out of bounds
        let metres: Result<Metres, Error> = _500KM.try_into();
        assert_eq!(metres, Err(Error::OutOfBounds));
    }

    #[test]
    fn recalculates_precision() {
        let metres: Metres = 23_480.try_into().unwrap();
        let values = [
            (metres.precision(Precision::_1M), 23_480),
            (metres.precision(Precision::_10M), 23_480),
            (metres.precision(Precision::_100M), 23_400),
            (metres.precision(Precision::_1Km), 23_000),
            (metres.precision(Precision::_10Km), 20_000),
            (metres.precision(Precision::_100Km), 0),
        ];

        for value in values {
            assert_eq!(Into::<u32>::into(value.0), value.1);
        }
    }

    #[test]
    fn adds_correct_padding() {
        // Test zero metres
        let metres: Metres = 0.try_into().unwrap();
        let values = [
            ("", Precision::_100Km),
            ("0", Precision::_10Km),
            ("00", Precision::_1Km),
            ("000", Precision::_100M),
            ("0000", Precision::_10M),
            ("00000", Precision::_1M),
        ];

        for value in values {
            assert_eq!(value.0, metres.padded(value.1));
        }

        // Test 250 metres
        let metres: Metres = 200_250.try_into().unwrap();
        let values = [
            ("", Precision::_100Km),
            ("0", Precision::_10Km),
            ("00", Precision::_1Km),
            ("002", Precision::_100M),
            ("0025", Precision::_10M),
            ("00250", Precision::_1M),
        ];

        for value in values {
            assert_eq!(value.0, metres.padded(value.1));
        }
    }
}
