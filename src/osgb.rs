use crate::constants::_500KM;
use crate::grid::{coords_to_square, square_to_coords};
use crate::utils::trim_string;
use crate::{coordinates::point::Point as GridPoint, Error, Precision};
use geo_types::{LineString, Point, Polygon};
use std::fmt::Display;
use std::str::FromStr;

/// The 500km grid's offset from the true origin.
const OFFSET_EAST: u32 = _500KM * 2;
const OFFSET_NORTH: u32 = _500KM;

/// Type representing a valid British National Grid Reference.
/// Can be instantiated either by parsing from a string or through
/// a valid set of eastings and northings as coordinates.
///
/// Provides functionality to convert between strings and coordinates,
/// as well as re-mapping to a new precision.
// Is primarily a wrapper over Point, but with additional logic to
// handle 500Km squares and their offset origin.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OSGB {
    square_500k_east: u32,
    square_500k_north: u32,
    point: GridPoint,
}

impl OSGB {
    /// Creates a new grid reference from the given coordinates
    /// and precision.
    ///
    /// # Errors
    /// Returns an error if the given coordinates are out of bounds.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSGB, Precision};
    ///
    /// let gridref = OSGB::new(
    ///     389_200,
    ///     243_700,
    ///     Precision::_100M
    /// ).unwrap();
    ///
    /// assert_eq!(gridref.to_string(), "SO892437".to_string());
    /// ```
    pub fn new(eastings: u32, northings: u32, precision: Precision) -> Result<Self, Error> {
        // The grid row and column determined from the true origin.
        let square_500k_east = (eastings + OFFSET_EAST) / _500KM;
        let square_500k_north = (northings + OFFSET_NORTH) / _500KM;

        // Determine the 500k grid square.
        let square = coords_to_square(square_500k_east as usize, square_500k_north as usize)?;

        // If 500km square is out of range return error
        if !['S', 'T', 'N', 'O', 'H'].contains(&square) {
            Err(Error::ParseError(format!(
                "{square} is not a supported 500km square."
            )))
        } else {
            let eastings = (eastings % _500KM).try_into()?;
            let northings = (northings % _500KM).try_into()?;

            Ok(Self {
                square_500k_east,
                square_500k_north,
                point: GridPoint::new(eastings, northings, precision),
            })
        }
    }

    /// Recalculates the grid reference to a new precision.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSGB, Precision};
    ///
    /// let gridref_100m: OSGB = "SO892437".parse().unwrap();
    /// let gridref_10k = gridref_100m.recalculate(Precision::_10Km);
    ///
    /// assert_eq!("SO84".to_string(), gridref_10k.to_string());
    /// ```
    pub fn recalculate(&self, precision: Precision) -> Self {
        if precision > self.point.precision() {
            //
            self.clone()
        } else {
            let point = GridPoint::new(self.point.eastings(), self.point.northings(), precision);

            Self {
                square_500k_east: self.square_500k_east,
                square_500k_north: self.square_500k_north,
                point,
            }
        }
    }

    /// Returns the point at the osgb's
    /// 'South West' corner - its origin.
    /// Recalculates the grid reference to a new precision.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.sw(), coord! {x: 389_200, y: 243_700 }.into());
    /// ```
    pub fn sw(&self) -> Point<u32> {
        Point::new(self.eastings(), self.northings())
    }

    /// Returns the point at the osgb's
    /// 'North West' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.nw(), coord! {x: 389_200, y: 243_800 }.into());
    /// ```
    pub fn nw(&self) -> Point<u32> {
        Point::new(
            self.eastings(),
            self.northings() + self.point.precision().metres(),
        )
    }

    /// Returns the point at the osgb's
    /// 'North East' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.ne(), coord! {x: 389_300, y: 243_800 }.into());
    /// ```
    pub fn ne(&self) -> Point<u32> {
        Point::new(
            self.eastings() + self.point.precision().metres(),
            self.northings() + self.point.precision().metres(),
        )
    }

    /// Returns the point at the osgb's
    /// 'South East' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.se(), coord! {x: 389_300, y: 243_700 }.into());
    /// ```
    pub fn se(&self) -> Point<u32> {
        Point::new(
            self.eastings() + self.point.precision().metres(),
            self.northings(),
        )
    }

    /// Returns the point at the osgb's
    /// centre.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.centre(), coord! {x: 389_250, y: 243_750 }.into());
    /// ```
    pub fn centre(&self) -> Point<u32> {
        Point::new(
            self.eastings() + (self.point.precision().metres() / 2),
            self.northings() + (self.point.precision().metres() / 2),
        )
    }

    /// Returns the osgb's perimeter.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::{LineString, Point, Polygon};
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(
    ///     gridref.perimeter(),
    ///     Polygon::new(
    ///         LineString::from(
    ///             vec![
    ///                 Point::new(389_200, 243_700),
    ///                 Point::new(389_200, 243_800),
    ///                 Point::new(389_300, 243_800),
    ///                 Point::new(389_300, 243_700)
    ///             ]
    ///         ),
    ///         vec![]
    ///     )
    /// );
    /// ```
    pub fn perimeter(&self) -> Polygon<u32> {
        Polygon::new(
            LineString::from(vec![self.sw(), self.nw(), self.ne(), self.se()]),
            vec![],
        )
    }

    /// Returns the osgb's precision.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSGB, Precision};
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.precision(), Precision::_100M);
    /// ```
    pub fn precision(&self) -> Precision {
        self.point.precision()
    }

    // Returns the eastings calculated from the offset origin.
    fn eastings(&self) -> u32 {
        let east_500k = (self.square_500k_east * _500KM) - OFFSET_EAST;

        east_500k + self.point.eastings().inner()
    }

    // Returns the northings calculated from the offset origin.
    fn northings(&self) -> u32 {
        let north_500k = (self.square_500k_north * _500KM) - OFFSET_NORTH;

        north_500k + self.point.northings().inner()
    }
}

impl FromStr for OSGB {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: String = trim_string(s);

        match string.chars().next() {
            Some(c) => {
                let (east, north) = square_to_coords(&c)?;
                let point: GridPoint = string[1..string.len()].parse()?;

                Ok(Self {
                    square_500k_east: east as u32,
                    square_500k_north: north as u32,
                    point,
                })
            }
            None => Err(Error::ParseError("String can not be empty.".to_string())),
        }
    }
}

impl Display for OSGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Unwrapping here as squares have already been validated.
        let square = coords_to_square(
            self.square_500k_east as usize,
            self.square_500k_north as usize,
        )
        .unwrap();

        write!(f, "{}{}", square, self.point)
    }
}

#[cfg(test)]
mod test {
    use crate::OSGB;
    use geo_types::{LineString, Point, Polygon};

    #[test]
    fn coordinates_are_correct() {
        let osgb = OSGB::new(0, 0, crate::Precision::_100M).unwrap();
        let sw = osgb.sw();
        let nw = osgb.nw();
        let ne = osgb.ne();
        let se = osgb.se();

        assert_eq!(sw, Point::new(0, 0));
        assert_eq!(nw, Point::new(0, 100));
        assert_eq!(ne, Point::new(100, 100));
        assert_eq!(se, Point::new(100, 0));
        assert_eq!(osgb.centre(), Point::new(50, 50));
        assert_eq!(
            osgb.perimeter(),
            Polygon::new(LineString::from(vec![sw, nw, ne, se]), vec![])
        )
    }
}
