use std::{fmt::Display, str::FromStr};

use geo_types::{LineString, Point, Polygon};

use crate::{
    ParseError,
    constants::*,
    error::OutOfBoundsError,
    grid::{GRID, coords_to_grid, grid_to_coords},
    grid_reference::GridReference,
    resolution::Resolution,
};

// The 500km grid's offset from the true origin.
const OFFSET_EAST: u32 = _500KM * 2;
const OFFSET_NORTH: u32 = _500KM;

// The bounds for eastings and northings
const BOUNDS_EAST: u32 = (_500KM * 5) - OFFSET_EAST;
const BOUNDS_NORTH: u32 = (_500KM * 5) - OFFSET_NORTH;

/// Type representing a valid British National Grid Reference.
/// Can be instantiated either by parsing from a string or through
/// a valid set of eastings and northings as coordinates.
///
/// Provides functionality to convert between strings and coordinates,
/// as well as re-mapping to a new precision.
// Is primarily a wrapper over Point, but with additional logic to
// handle 500Km squares and their false origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OSGB {
    point: crate::grid_reference::GridReference,
    square_500k_east: u32,
    square_500k_north: u32,
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
    /// use gridish::{OSGB, Resolution};
    ///
    /// let gridref = OSGB::new(
    ///     389_200,
    ///     243_700,
    ///     Resolution::_100m
    /// ).unwrap();
    ///
    /// assert_eq!(gridref.to_string(), "SO892437".to_string());
    /// ```
    pub fn new(
        eastings: u32,
        northings: u32,
        resolution: Resolution,
    ) -> Result<Self, OutOfBoundsError> {
        match (eastings >= BOUNDS_EAST, northings >= BOUNDS_NORTH) {
            (true, true) => Err(OutOfBoundsError::Both),
            (true, false) => Err(OutOfBoundsError::Eastings),
            (false, true) => Err(OutOfBoundsError::Northings),
            (false, false) => {
                let square_500k_east = (eastings + OFFSET_EAST) / _500KM;
                let square_500k_north = (northings + OFFSET_NORTH) / _500KM;
                let eastings = eastings % _500KM;
                let northings = northings % _500KM;

                Ok(Self {
                    point: GridReference::new(eastings, northings, resolution)?,
                    square_500k_east,
                    square_500k_north,
                })
            }
        }
    }

    pub fn eastings(&self) -> u32 {
        let east_500k = (self.square_500k_east * _500KM) - OFFSET_EAST;

        east_500k + self.point.eastings()
    }

    pub fn northings(&self) -> u32 {
        let north_500k = (self.square_500k_north * _500KM) - OFFSET_NORTH;

        north_500k + self.point.northings()
    }

    pub fn resolution(&self) -> Resolution {
        self.point.resolution()
    }

    /// Recalculates the grid reference to a new resolution.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSGB, Resolution};
    ///
    /// let gridref_100m: OSGB = "SO892437".parse().unwrap();
    /// let gridref_10k = gridref_100m.recalculate(Resolution::_10km);
    ///
    /// assert_eq!("SO84".to_string(), gridref_10k.to_string());
    /// ```
    pub fn recalculate(&self, resolution: Resolution) -> Self {
        if resolution.metres() <= self.point.resolution().metres() {
            self.clone()
        } else {
            Self {
                square_500k_east: self.square_500k_east,
                square_500k_north: self.square_500k_north,
                point: self.point.recalculate(resolution),
            }
        }
    }

    /// Returns the point at the Grid Reference's South West corner - its origin.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.south_west(), coord! {x: 389_200.0, y: 243_700.0 }.into());
    /// ```
    pub fn south_west(&self) -> Point {
        Point::new(self.eastings() as f64, self.northings() as f64)
    }

    /// Returns the point at the Grid Reference's North West corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.north_west(), coord! {x: 389_200.0, y: 243_800.0 }.into());
    /// ```
    pub fn north_west(&self) -> Point {
        Point::new(
            self.eastings() as f64,
            (self.northings() + self.point.resolution().metres()) as f64,
        )
    }

    /// Returns the point at the Grid Reference's North East corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.north_east(), coord! {x: 389_300.0, y: 243_800.0 }.into());
    /// ```
    pub fn north_east(&self) -> Point {
        Point::new(
            (self.eastings() + self.point.resolution().metres()) as f64,
            (self.northings() + self.point.resolution().metres()) as f64,
        )
    }

    /// Returns the point at the Grid Reference's South East corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.south_east(), coord! {x: 389_300.0, y: 243_700.0 }.into());
    /// ```
    pub fn south_east(&self) -> Point {
        Point::new(
            (self.eastings() + self.point.resolution().metres()) as f64,
            self.northings() as f64,
        )
    }

    /// Returns the point at the Grid Reference's centre.
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.centre(), coord! {x: 389_250.0, y: 243_750.0 }.into());
    /// ```
    pub fn centre(&self) -> Point {
        Point::new(
            self.eastings() as f64 + (self.point.resolution().metres() as f64 / 2.0),
            self.northings() as f64 + (self.point.resolution().metres() as f64 / 2.0),
        )
    }

    /// Returns the Grid Reference's perimeter.
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
    ///                 Point::new(389_200.0, 243_700.0),
    ///                 Point::new(389_200.0, 243_800.0),
    ///                 Point::new(389_300.0, 243_800.0),
    ///                 Point::new(389_300.0, 243_700.0)
    ///             ]
    ///         ),
    ///         vec![]
    ///     )
    /// );
    /// ```
    pub fn perimeter(&self) -> Polygon {
        Polygon::new(
            LineString::from(vec![
                self.south_west(),
                self.north_west(),
                self.north_east(),
                self.south_east(),
            ]),
            vec![],
        )
    }
}

impl FromStr for OSGB {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some(c) => {
                let (east, north) = grid_to_coords(&c, &GRID)?;
                match (
                    (east as u32 * _500KM) < OFFSET_EAST,
                    (north as u32 * _500KM) < OFFSET_NORTH,
                ) {
                    (true, true) => Err(ParseError::OutOfBounds(OutOfBoundsError::Both)),
                    (true, false) => Err(ParseError::OutOfBounds(OutOfBoundsError::Eastings)),
                    (false, true) => Err(ParseError::OutOfBounds(OutOfBoundsError::Northings)),
                    (false, false) => Ok(Self {
                        point: GridReference::from_string_rep(&s[1..])?,
                        square_500k_east: east as u32,
                        square_500k_north: north as u32,
                    }),
                }
            }
            None => Err(ParseError::InvalidString("Found empty string".to_string())),
        }
    }
}

impl Display for OSGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let square = coords_to_grid(
            self.square_500k_east as usize,
            self.square_500k_north as usize,
            &GRID,
        );

        write!(f, "{}{}", square, self.point.to_string_rep())
    }
}

#[cfg(feature = "serde")]
mod serde {
    use crate::OSGB;
    use serde::{de, ser};
    use std::fmt;

    impl ser::Serialize for OSGB {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct OSGBVisitor;

    impl<'de> de::Visitor<'de> for OSGBVisitor {
        type Value = OSGB;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a formatted grid ref string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            value.parse().map_err(E::custom)
        }
    }

    impl<'de> de::Deserialize<'de> for OSGB {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(OSGBVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_print() {
        let tests = vec![
            (Resolution::_100km, "TL"),
            (Resolution::_50km, "TLSW"),
            (Resolution::_10km, "TL03"),
            (Resolution::_5km, "TL03NW"),
            #[cfg(feature = "tetrads")]
            (Resolution::_2km, "TL03P"),
            (Resolution::_1km, "TL0438"),
            (Resolution::_500m, "TL0438SE"),
            (Resolution::_100m, "TL048380"),
            (Resolution::_50m, "TL048380SE"),
            (Resolution::_10m, "TL04863802"),
            (Resolution::_5m, "TL04863802SE"),
            (Resolution::_1m, "TL0486638023"),
        ];
        for test in tests {
            let point = OSGB::new(504866, 238023, test.0).unwrap();

            assert_eq!(&point.to_string(), test.1);
        }
    }

    #[test]
    fn strings_parse() {
        let tests = vec![
            ("TL", (Resolution::_100km, 500000, 200000)),
            ("TLSW", (Resolution::_50km, 500000, 200000)),
            ("TL03", (Resolution::_10km, 500000, 230000)),
            ("TL03NW", (Resolution::_5km, 500000, 235000)),
            #[cfg(feature = "tetrads")]
            ("TL03P", (Resolution::_2km, 504000, 238000)),
            ("TL0438", (Resolution::_1km, 504000, 238000)),
            ("TL0438SE", (Resolution::_500m, 504500, 238000)),
            ("TL048380", (Resolution::_100m, 504800, 238000)),
            ("TL048380SE", (Resolution::_50m, 504850, 238000)),
            ("TL04863802", (Resolution::_10m, 504860, 238020)),
            ("TL04863802SE", (Resolution::_5m, 504865, 238020)),
            ("TL0486638023", (Resolution::_1m, 504866, 238023)),
        ];

        for test in tests {
            println!("Testing string: {}", test.0);
            let point = OSGB::from_str(test.0).unwrap();

            assert_eq!(point.resolution(), test.1.0);
            assert_eq!(point.eastings(), test.1.1);
            assert_eq!(point.northings(), test.1.2);
        }
    }

    #[test]
    fn points_recalculate() {
        let point = OSGB::new(504866, 238023, Resolution::_1m).unwrap();
        let tests = vec![
            (Resolution::_100km, (500000, 200000)),
            (Resolution::_50km, (500000, 200000)),
            (Resolution::_10km, (500000, 230000)),
            (Resolution::_5km, (500000, 235000)),
            #[cfg(feature = "tetrads")]
            (Resolution::_2km, (504000, 238000)),
            (Resolution::_1km, (504000, 238000)),
            (Resolution::_500m, (504500, 238000)),
            (Resolution::_100m, (504800, 238000)),
            (Resolution::_50m, (504850, 238000)),
            (Resolution::_10m, (504860, 238020)),
            (Resolution::_5m, (504865, 238020)),
            (Resolution::_1m, (504866, 238023)),
        ];

        for test in tests {
            let new = point.recalculate(test.0);
            assert_eq!((new.eastings(), new.northings()), test.1);
        }
    }

    #[test]
    fn recalculate_does_not_increase_resolution() {
        let tests = vec![
            Resolution::_100km,
            Resolution::_50km,
            Resolution::_10km,
            Resolution::_5km,
            #[cfg(feature = "tetrads")]
            Resolution::_2km,
            Resolution::_1km,
            Resolution::_500m,
            Resolution::_100m,
            Resolution::_50m,
            Resolution::_10m,
            Resolution::_5m,
        ];

        for test in tests {
            let point = OSGB::new(504866, 238023, test).unwrap();

            assert_eq!(
                point.recalculate(Resolution::_1m).resolution(),
                point.resolution()
            );
        }
    }
}
