use crate::constants::_100KM;
use crate::coordinates::metres::Metres;
use crate::grid::{coords_to_square, square_to_coords};
use crate::{utils, Error, Precision};
use std::fmt::Display;
use std::str::FromStr;

/// The core of the British and Irish national grids.
/// A coordinate point that can represent any location
/// on a 500km grid at up to 1m precision.
/// Made up of eastings, northings, and the precision.
///
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    eastings: Metres,
    northings: Metres,
    precision: Precision,
}

impl Point {
    pub fn new(eastings: Metres, northings: Metres, precision: Precision) -> Self {
        let eastings = eastings.precision(precision);
        let northings = northings.precision(precision);

        Self {
            eastings,
            northings,
            precision,
        }
    }

    pub fn eastings(&self) -> Metres {
        self.eastings
    }

    pub fn northings(&self) -> Metres {
        self.northings
    }

    pub fn precision(&self) -> Precision {
        self.precision
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some(c) => {
                // Determine grid square
                let (column, row) = square_to_coords(&c)?;

                // Parse digits and precision
                let (eastings, northings, precision) = utils::digits(&s[1..s.len()])?;

                // Finally, add grid square to eastings and northings.
                let eastings = (column as u32 * _100KM) + eastings;
                let northings = (row as u32 * _100KM) + northings;

                Ok(Self {
                    eastings: eastings.try_into()?,
                    northings: northings.try_into()?,
                    precision,
                })
            }
            None => Err(Error::ParseError("String can not be empty.".to_string())),
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let eastings = self.eastings.inner();
        let northings = self.northings.inner();

        // Determine letter.
        let column = (eastings / _100KM) as usize;
        let row = (northings / _100KM) as usize;
        // Unwrapping here as metres are type checked to fit into bounds.
        let letter = coords_to_square(column, row).unwrap();

        write!(
            f,
            "{}{}{}",
            letter,
            self.eastings.padded(self.precision),
            self.northings.padded(self.precision)
        )
    }
}

#[cfg(test)]
mod test {
    use crate::coordinates::point::Point;
    use crate::precision::Precision;

    struct TestPoint {
        eastings: u32,
        northings: u32,
        precision: Precision,
    }

    const VALID_POINTS: [(&'static str, TestPoint); 2] = [
        (
            "N",
            TestPoint {
                eastings: 200_000,
                northings: 200_000,
                precision: Precision::_100Km,
            },
        ),
        (
            "N24",
            TestPoint {
                eastings: 220_000,
                northings: 240_000,
                precision: Precision::_10Km,
            },
        ),
    ];

    #[test]
    fn recalculates_precision_on_initialisation() {
        let eastings = 123.try_into().unwrap();
        let northings = 2000.try_into().unwrap();
        let precision = Precision::_10M;

        let point = Point::new(eastings, northings, precision);

        assert_eq!(point.eastings(), 120.try_into().unwrap());
        assert_eq!(point.northings(), 2000.try_into().unwrap());
    }

    #[test]
    fn parses_valid_strings() {
        for point in VALID_POINTS {
            let grid_point: Point = point.0.parse().unwrap();

            assert_eq!(grid_point.eastings.inner(), point.1.eastings);
            assert_eq!(grid_point.northings.inner(), point.1.northings);
            assert_eq!(grid_point.precision, point.1.precision);
        }
    }

    #[test]
    fn prints_valid_strings() {
        for point in VALID_POINTS {
            let eastings = point.1.eastings.try_into().unwrap();
            let northings = point.1.northings.try_into().unwrap();
            let grid_point = Point::new(eastings, northings, point.1.precision);

            assert_eq!(grid_point.to_string(), point.0);
        }
    }
}
