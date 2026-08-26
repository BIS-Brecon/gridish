use crate::{
    ParseError,
    constants::*,
    error::OutOfBoundsError,
    grid::{GRID, coords_to_grid, grid_to_coords},
    resolution::Resolution,
    utils::{pad, round_down},
};

#[cfg(any(feature = "tetrads", feature = "quadrants"))]
use crate::resolution::Suffix;

/// The core of the British and Irish national grids.
/// A coordinate point that can represent any location
/// on a 500km grid at up to 1m precision.
/// Made up of eastings, northings, and the resolution.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) struct GridReference {
    eastings: u32,
    northings: u32,
    resolution: Resolution,
}

impl GridReference {
    /// Create a new Grid Reference.
    /// Returns an out of bounds error if eastings or northings fall outside the 500km grid.
    pub(crate) fn new(
        eastings: u32,
        northings: u32,
        resolution: Resolution,
    ) -> Result<Self, OutOfBoundsError> {
        match (eastings >= _500KM, northings >= _500KM) {
            (true, true) => Err(OutOfBoundsError::Both),
            (true, false) => Err(OutOfBoundsError::Eastings),
            (false, true) => Err(OutOfBoundsError::Northings),
            (false, false) => Ok(Self {
                eastings: round_down(eastings, resolution.metres()),
                northings: round_down(northings, resolution.metres()),
                resolution,
            }),
        }
    }

    /// Return the eastings of the grid reference.
    pub(crate) fn eastings(&self) -> u32 {
        self.eastings
    }

    /// Return the northings of the grid reference.
    pub(crate) fn northings(&self) -> u32 {
        self.northings
    }

    /// Return the resolution of the grid reference.
    pub(crate) fn resolution(&self) -> Resolution {
        self.resolution
    }

    /// Recalculate the grid reference to the given resolution.
    /// Truncates eastings and northings if the resolution is lower, but keeps existing
    /// properties intact if the resolution is greater to avoid giving a false impression
    /// of accuracy.
    pub(crate) fn recalculate(&self, resolution: Resolution) -> Self {
        if resolution.metres() <= self.resolution().metres() {
            return self.clone();
        }

        let eastings = round_down(self.eastings(), resolution.metres());
        let northings = round_down(self.northings(), resolution.metres());

        Self::new(eastings, northings, resolution)
            .expect("Existing grid reference should already be in bounds.")
    }

    /// Converts a Grid Reference to its String Representation.
    pub(crate) fn to_string_rep(&self) -> String {
        // Determine string representation
        cfg_select! {
            any(feature = "tetrads", feature = "quadrants") => {
                let (res, digits, suffix) = self.resolution().representation();
            }
            _ => {
                let (res, digits) = self.resolution().representation();
            }
        }

        // Setup working eastings and northings
        let (mut east, mut north) = (self.eastings, self.northings);

        // Square
        let square = coords_to_grid((east / _100KM) as usize, (north / _100KM) as usize, &GRID);
        east = east % _100KM;
        north = north % _100KM;

        // Digits
        let east_digits = pad(east / res, digits as usize / 2);
        let north_digits = pad(north / res, digits as usize / 2);

        cfg_select! {
            any(feature = "tetrads", feature = "quadrants") => {
                // Suffix
                let suffix = if let Some(s) = suffix {
                    east = east % res;
                    north = north % res;

                    match s {
                        #[cfg(feature = "quadrants")]
                        Suffix::Quadrant => {
                            let half = res / 2;

                            match (east >= half, north >= half) {
                                (true, true) => "NE",
                                (true, false) => "SE",
                                (false, true) => "NW",
                                (false, false) => "SW",
                            }
                        }
                        #[cfg(feature = "tetrads")]
                        Suffix::Tetrad => &coords_to_grid(
                            (east / self.resolution().metres()) as usize,
                            (north / self.resolution().metres()) as usize,
                            &crate::grid::TETRAD_GRID,
                        )
                        .to_string(),
                    }
                } else {
                    ""
                };

                format!("{square}{east_digits}{north_digits}{suffix}")
            }
            _ => {
                format!("{square}{east_digits}{north_digits}")
            }
        }
    }

    /// Create a Grid Reference from its String representation.,
    pub(crate) fn from_string_rep(s: &str) -> Result<Self, ParseError> {
        let mut res = _100KM;
        let mut chars = s.chars();
        let mut east = 0;
        let mut north = 0;

        // Parse square
        let square = chars
            .next()
            .ok_or(ParseError::InvalidString("Empty string".to_string()))?;
        let (e, n) = grid_to_coords(&square, &GRID)?;
        east += e as u32 * res;
        north += n as u32 * res;

        // Start tracking position in string
        let mut pos = 1;

        // Parse digits if found
        if let Some(end) = s.rfind(|c: char| c.is_numeric()) {
            let digits = &s[pos..=end].trim_start();
            let (eastings, northings) = if let Some(s) = digits.split_once(' ') {
                (s.0, s.1)
            } else {
                digits.split_at(digits.char_indices().count() / 2)
            };

            // Determine resolution from digits length
            res = match (eastings.len(), northings.len()) {
                (1, 1) => _10KM,
                (2, 2) => _1KM,
                (3, 3) => _100M,
                (4, 4) => _10M,
                (5, 5) => _1M,
                _ => {
                    return Err(ParseError::InvalidString(format!(
                        "{} is not a supported number of digits.",
                        eastings.len() + northings.len()
                    )));
                }
            };

            east += eastings.parse::<u32>()? * res;
            north += northings.parse::<u32>()? * res;

            // Move pos to end of digits for finding suffix
            pos = end;
        }

        // Parse suffix if found
        #[allow(unused_variables)]
        if let Some(start) = s[pos..].find(|c: char| c.is_alphabetic()) {
            cfg_select! {
                any(feature = "tetrads", feature = "quadrants") => {
                    // Correct start position
                    let start = pos + start;
                    let suffix = &s[start..s.len()].trim();
                    #[allow(unused_mut)]
                    let mut matched_suffix = false;

                    // Tetrad
                    #[cfg(feature = "tetrads")]
                    if suffix.len() == 1 {
                        if res == _10KM {
                            res = res / 5;
                            let (e, n) = grid_to_coords(
                                &suffix.chars().next().expect("String should not be empty"),
                                &crate::grid::TETRAD_GRID,
                            )?;
                            east += e as u32 * res;
                            north += n as u32 * res;
                            matched_suffix = true;
                        } else {
                            return Err(ParseError::InvalidString(
                                "Tetrads are only supported for 2 figure grid references.".to_string(),
                            ));
                        }
                    }

                    if !matched_suffix {
                        cfg_select! {
                            feature = "quadrants" => {
                                res = res / 2;
                                let (e, n) = match *suffix {
                                    "NW" => (0, res),
                                    "NE" => (res, res),
                                    "SE" => (res, 0),
                                    "SW" => (0, 0),
                                    _ => {
                                        return Err(ParseError::InvalidString(format!("{suffix} is not a valid quadrant.")));
                                    }
                                };

                                east += e;
                                north += n;
                            }
                            feature = "tetrads" => {
                                return Err(ParseError::InvalidString(format!("{suffix} is not a valid tetrad.")));
                            }
                        }
                    }
                }
                _ => {
                    return Err(ParseError::InvalidString("Extra characters found after digits.".to_string()));
                }
            }
        }

        let resolution = match res {
            _100KM => Resolution::_100km,
            #[cfg(feature = "quadrants")]
            _50KM => Resolution::_50km,
            _10KM => Resolution::_10km,
            #[cfg(feature = "quadrants")]
            _5KM => Resolution::_5km,
            #[cfg(feature = "tetrads")]
            _2KM => Resolution::_2km,
            _1KM => Resolution::_1km,
            #[cfg(feature = "quadrants")]
            _500M => Resolution::_500m,
            _100M => Resolution::_100m,
            #[cfg(feature = "quadrants")]
            _50M => Resolution::_50m,
            _10M => Resolution::_10m,
            #[cfg(feature = "quadrants")]
            _5M => Resolution::_5m,
            _1M => Resolution::_1m,
            _ => {
                return Err(ParseError::InvalidString(format!(
                    "{res} is not a supported resolution."
                )));
            }
        };

        Ok(Self::new(east, north, resolution)?)
    }
}
