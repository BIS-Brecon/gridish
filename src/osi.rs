use std::{fmt::Display, str::FromStr};

use geo_types::{LineString, Point, Polygon};

use crate::{
    ParseError, constants::*, error::OutOfBoundsError, grid_reference::GridReference,
    resolution::Resolution,
};

// The bounds for eastings and northings
const BOUNDS_EAST: u32 = _500KM;
const BOUNDS_NORTH: u32 = _500KM;

/// Type representing a valid Irish National Grid Reference.
/// Can be instantiated either by parsing from a string or through
/// a valid set of eastings and northings as coordinates.
///
/// Provides functionality to convert between strings and coordinates,
/// as well as re-mapping to a new precision.
// Works as a simple wrapper around Point, with some additional methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OSI {
    point: crate::grid_reference::GridReference,
}

impl OSI {
    /// Creates a new grid reference from the given coordinates
    /// and precision.
    ///
    /// # Errors
    /// Returns an error if the given coordinates are out of bounds.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSI, Resolution};
    ///
    /// let gridref = OSI::new(
    ///     389_200,
    ///     243_700,
    ///     Resolution::_100m
    /// ).unwrap();
    ///
    /// assert_eq!(gridref.to_string(), "O892437".to_string());
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
            (false, false) => Ok(Self {
                point: GridReference::new(eastings, northings, resolution)?,
            }),
        }
    }

    pub fn eastings(&self) -> u32 {
        self.point.eastings()
    }

    pub fn northings(&self) -> u32 {
        self.point.northings()
    }

    pub fn resolution(&self) -> Resolution {
        self.point.resolution()
    }

    /// Recalculates the grid reference to a new resolution.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSI, Resolution};
    ///
    /// let gridref_100m: OSI = "O892437".parse().unwrap();
    /// let gridref_10k = gridref_100m.recalculate(Resolution::_10km);
    ///
    /// assert_eq!("O84".to_string(), gridref_10k.to_string());
    /// ```
    pub fn recalculate(&self, resolution: Resolution) -> Self {
        if resolution.metres() <= self.point.resolution().metres() {
            self.clone()
        } else {
            Self {
                point: self.point.recalculate(resolution),
            }
        }
    }

    /// Returns the point at the Grid Reference's South West corner - its origin.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
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
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
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
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
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
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
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
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
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
    /// use gridish::OSI;
    /// use geo_types::{LineString, Point, Polygon};
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
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

impl FromStr for OSI {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let point = GridReference::from_string_rep(s)?;

        Ok(Self { point })
    }
}

impl Display for OSI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.point.to_string_rep())
    }
}

#[cfg(feature = "serde")]
mod serde {
    use crate::OSI;
    use serde::{de, ser};
    use std::fmt;

    impl ser::Serialize for OSI {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    struct OSIVisitor;

    impl<'de> de::Visitor<'de> for OSIVisitor {
        type Value = OSI;

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

    impl<'de> de::Deserialize<'de> for OSI {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            deserializer.deserialize_str(OSIVisitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_print() {
        let tests = vec![
            (Resolution::_100km, "O"),
            #[cfg(feature = "quadrants")]
            (Resolution::_50km, "OSW"),
            (Resolution::_10km, "O13"),
            #[cfg(feature = "quadrants")]
            (Resolution::_5km, "O13SE"),
            #[cfg(feature = "tetrads")]
            (Resolution::_2km, "O13M"),
            (Resolution::_1km, "O1534"),
            #[cfg(feature = "quadrants")]
            (Resolution::_500m, "O1534NE"),
            (Resolution::_100m, "O159346"),
            #[cfg(feature = "quadrants")]
            (Resolution::_50m, "O159346NW"),
            (Resolution::_10m, "O15903467"),
            #[cfg(feature = "quadrants")]
            (Resolution::_5m, "O15903467SW"),
            (Resolution::_1m, "O1590434671"),
        ];
        for test in tests {
            let point = OSI::new(315904, 234671, test.0).unwrap();

            assert_eq!(&point.to_string(), test.1);
        }
    }

    #[test]
    fn strings_parse() {
        let tests = vec![
            ("O", (Resolution::_100km, 300000, 200000)),
            #[cfg(feature = "quadrants")]
            ("OSW", (Resolution::_50km, 300000, 200000)),
            ("O13", (Resolution::_10km, 310000, 230000)),
            #[cfg(feature = "quadrants")]
            ("O13SE", (Resolution::_5km, 315000, 230000)),
            #[cfg(feature = "tetrads")]
            ("O13M", (Resolution::_2km, 314000, 234000)),
            ("O1534", (Resolution::_1km, 315000, 234000)),
            #[cfg(feature = "quadrants")]
            ("O1534NE", (Resolution::_500m, 315500, 234500)),
            ("O159346", (Resolution::_100m, 315900, 234600)),
            #[cfg(feature = "quadrants")]
            ("O159346NW", (Resolution::_50m, 315900, 234650)),
            ("O15903467", (Resolution::_10m, 315900, 234670)),
            #[cfg(feature = "quadrants")]
            ("O15903467SW", (Resolution::_5m, 315900, 234670)),
            ("O1590434671", (Resolution::_1m, 315904, 234671)),
        ];

        for test in tests {
            println!("Testing string: {}", test.0);
            let point = OSI::from_str(test.0).unwrap();

            assert_eq!(point.resolution(), test.1.0);
            assert_eq!(point.eastings(), test.1.1);
            assert_eq!(point.northings(), test.1.2);
        }
    }

    #[test]
    fn points_recalculate() {
        let point = OSI::new(315904, 234671, Resolution::_1m).unwrap();
        let tests = vec![
            (Resolution::_100km, (300000, 200000)),
            #[cfg(feature = "quadrants")]
            (Resolution::_50km, (300000, 200000)),
            (Resolution::_10km, (310000, 230000)),
            #[cfg(feature = "quadrants")]
            (Resolution::_5km, (315000, 230000)),
            #[cfg(feature = "tetrads")]
            (Resolution::_2km, (314000, 234000)),
            (Resolution::_1km, (315000, 234000)),
            #[cfg(feature = "quadrants")]
            (Resolution::_500m, (315500, 234500)),
            (Resolution::_100m, (315900, 234600)),
            #[cfg(feature = "quadrants")]
            (Resolution::_50m, (315900, 234650)),
            (Resolution::_10m, (315900, 234670)),
            #[cfg(feature = "quadrants")]
            (Resolution::_5m, (315900, 234670)),
            (Resolution::_1m, (315904, 234671)),
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
            #[cfg(feature = "quadrants")]
            Resolution::_50km,
            Resolution::_10km,
            #[cfg(feature = "quadrants")]
            Resolution::_5km,
            #[cfg(feature = "tetrads")]
            Resolution::_2km,
            Resolution::_1km,
            #[cfg(feature = "quadrants")]
            Resolution::_500m,
            Resolution::_100m,
            #[cfg(feature = "quadrants")]
            Resolution::_50m,
            Resolution::_10m,
            #[cfg(feature = "quadrants")]
            Resolution::_5m,
        ];

        for test in tests {
            let point = OSI::new(315904, 234671, test).unwrap();

            assert_eq!(
                point.recalculate(Resolution::_1m).resolution(),
                point.resolution()
            );
        }
    }
}

#[test]
#[cfg(not(feature = "tetrads"))]
fn tetrads_are_rejected_when_not_enabled() {
    cfg_select! {
        feature = "quadrants" => {
            assert_eq!(
                OSI::from_str("L03P"),
                Err(ParseError::InvalidString("P is not a valid quadrant.".to_string()))
            );
        }
        _ => {
            assert_eq!(
                OSI::from_str("L03P"),
                Err(ParseError::InvalidString("Extra characters found after digits.".to_string()))
            );
        }
    }
}

#[test]
#[cfg(not(feature = "quadrants"))]
fn quadrants_are_rejected_when_not_enabled() {
    cfg_select! {
        feature = "tetrads" => {
            assert_eq!(
                OSI::from_str("L03SW"),
                Err(ParseError::InvalidString("SW is not a valid tetrad.".to_string()))
            );
        }
        _ => {
            assert_eq!(
                OSI::from_str("L03SW"),
                Err(ParseError::InvalidString("Extra characters found after digits.".to_string()))
            );
        }
    }
}
