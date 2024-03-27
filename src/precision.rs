use crate::constants::*;

/// Supported 'resolutions' for grid references.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum Precision {
    _100Km,
    _10Km,
    _1Km,
    _100M,
    _10M,
    _1M,
}

impl Precision {
    /// Returns the Precision in metres
    pub fn metres(&self) -> u32 {
        match self {
            Precision::_100Km => _100KM,
            Precision::_10Km => _10KM,
            Precision::_1Km => _1KM,
            Precision::_100M => _100M,
            Precision::_10M => _10M,
            Precision::_1M => _1M,
        }
    }

    /// Returns the number of digits needed to represent
    /// a grid reference with this precision
    pub fn digits(&self) -> usize {
        match self {
            Precision::_100Km => 0,
            Precision::_10Km => 2,
            Precision::_1Km => 4,
            Precision::_100M => 6,
            Precision::_10M => 8,
            Precision::_1M => 10,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::constants::*;
    use crate::Precision;

    #[test]
    fn converts_to_metres() {
        assert_eq!(Precision::_100Km.metres(), _100KM);
        assert_eq!(Precision::_10Km.metres(), _10KM);
        assert_eq!(Precision::_1Km.metres(), 1_000);
        assert_eq!(Precision::_100M.metres(), 100);
        assert_eq!(Precision::_10M.metres(), 10);
        assert_eq!(Precision::_1M.metres(), 1);
    }
}
