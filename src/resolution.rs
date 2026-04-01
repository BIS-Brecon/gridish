use crate::constants::*;

pub(crate) enum Suffix {
    Quadrant,
    #[cfg(feature = "tetrads")]
    Tetrad,
}

/// Supported resolutions for working with Grid References.
/// Represents the size of the square in metres that a point can fall within.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Resolution {
    /// 100km Square
    _100km,
    /// 50km Square (Quadrant of 100km Square)
    _50km,
    /// 10km Square
    _10km,
    /// 5km Square (Quadrant of 10km Square)
    _5km,
    /// 2km Square (Tetrad)
    #[cfg(feature = "tetrads")]
    _2km,
    /// 1km Square
    _1km,
    /// 500m Square (Quadrant of 1km Square)
    _500m,
    /// 100m Square
    _100m,
    /// 50m Square (Quadrant of 100m Square)
    _50m,
    /// 10m Square
    _10m,
    /// 5m Square (Quadrant of 10m Square)
    _5m,
    /// 1m Square
    _1m,
}

impl Resolution {
    /// Returns the Resolution in metres
    ///
    /// # Example
    /// ```
    /// # use gridish::Resolution;
    /// assert_eq!(Resolution::_1km.metres(), 1_000);
    /// ```
    pub fn metres(&self) -> u32 {
        match self {
            Resolution::_100km => _100KM,
            Resolution::_50km => _50KM,
            Resolution::_10km => _10KM,
            Resolution::_5km => _5KM,
            #[cfg(feature = "tetrads")]
            Resolution::_2km => _2KM,
            Resolution::_1km => _1KM,
            Resolution::_500m => _500M,
            Resolution::_100m => _100M,
            Resolution::_50m => _50M,
            Resolution::_10m => _10M,
            Resolution::_5m => _5M,
            Resolution::_1m => _1M,
        }
    }

    // Determines how the given resolution should be represented as a string
    pub(crate) fn representation(&self) -> (u32, u8, Option<Suffix>) {
        match self {
            Resolution::_100km => (_100KM, 0, None),
            Resolution::_50km => (_100KM, 0, Some(Suffix::Quadrant)),
            Resolution::_10km => (_10KM, 2, None),
            Resolution::_5km => (_10KM, 2, Some(Suffix::Quadrant)),
            #[cfg(feature = "tetrads")]
            Resolution::_2km => (_10KM, 2, Some(Suffix::Tetrad)),
            Resolution::_1km => (_1KM, 4, None),
            Resolution::_500m => (_1KM, 4, Some(Suffix::Quadrant)),
            Resolution::_100m => (_100M, 6, None),
            Resolution::_50m => (_100M, 6, Some(Suffix::Quadrant)),
            Resolution::_10m => (_10M, 8, None),
            Resolution::_5m => (_10M, 8, Some(Suffix::Quadrant)),
            Resolution::_1m => (_1M, 10, None),
        }
    }
}
