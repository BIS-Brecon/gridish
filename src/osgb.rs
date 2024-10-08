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
    ///
    /// # Example
    /// ```
    /// use gridish::OSGB;
    /// use geo_types::coord;
    ///
    /// let gridref: OSGB = "SO892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.sw(), coord! {x: 389_200.0, y: 243_700.0 }.into());
    /// ```
    pub fn sw(&self) -> Point {
        Point::new(self.eastings() as f64, self.northings() as f64)
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
    /// assert_eq!(gridref.nw(), coord! {x: 389_200.0, y: 243_800.0 }.into());
    /// ```
    pub fn nw(&self) -> Point {
        Point::new(
            self.eastings() as f64,
            (self.northings() + self.point.precision().metres()) as f64,
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
    /// assert_eq!(gridref.ne(), coord! {x: 389_300.0, y: 243_800.0 }.into());
    /// ```
    pub fn ne(&self) -> Point {
        Point::new(
            (self.eastings() + self.point.precision().metres()) as f64,
            (self.northings() + self.point.precision().metres()) as f64,
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
    /// assert_eq!(gridref.se(), coord! {x: 389_300.0, y: 243_700.0 }.into());
    /// ```
    pub fn se(&self) -> Point {
        Point::new(
            (self.eastings() + self.point.precision().metres()) as f64,
            self.northings() as f64,
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
    /// assert_eq!(gridref.centre(), coord! {x: 389_250.0, y: 243_750.0 }.into());
    /// ```
    pub fn centre(&self) -> Point {
        Point::new(
            self.eastings() as f64 + (self.point.precision().metres() as f64 / 2.0),
            self.northings() as f64 + (self.point.precision().metres() as f64 / 2.0),
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

        assert_eq!(sw, Point::new(0.0, 0.0));
        assert_eq!(nw, Point::new(0.0, 100.0));
        assert_eq!(ne, Point::new(100.0, 100.0));
        assert_eq!(se, Point::new(100.0, 0.0));
        assert_eq!(osgb.centre(), Point::new(50.0, 50.0));
        assert_eq!(
            osgb.perimeter(),
            Polygon::new(LineString::from(vec![sw, nw, ne, se]), vec![])
        )
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

    #[cfg(test)]
    mod tests {
        use crate::{grid, Precision, OSGB};

        #[derive(Clone)]
        pub struct TestGrid {
            pub eastings: u32,
            pub northings: u32,
            pub precision: Precision,
            pub input_string: String,
            pub output_string: String,
        }

        impl TestGrid {
            pub fn new(
                eastings: u32,
                northings: u32,
                precision: Precision,
                input_string: &str,
                output_string: &str,
            ) -> TestGrid {
                TestGrid {
                    eastings,
                    northings,
                    precision,
                    input_string: input_string.to_string(),
                    output_string: output_string.to_string(),
                }
            }
        }

        fn grids() -> [TestGrid; 10] {
            [
                TestGrid::new(300_000, 200_000, Precision::_100Km, "SO", "SO"),
                TestGrid::new(380_000, 240_000, Precision::_10Km, "SO84", "SO84"),
                TestGrid::new(389_000, 243_000, Precision::_1Km, "SO8943", "SO8943"),
                TestGrid::new(389_200, 243_700, Precision::_100M, "SO892437", "SO892437"),
                TestGrid::new(
                    389_290,
                    243_760,
                    Precision::_10M,
                    "SO89294376",
                    "SO89294376",
                ),
                TestGrid::new(
                    389_291,
                    243_762,
                    Precision::_1M,
                    "SO8929143762",
                    "SO8929143762",
                ),
                TestGrid::new(224_000, 668_000, Precision::_1Km, "ns 24 68", "NS2468"),
                TestGrid::new(365_000, 620_000, Precision::_1Km, "NT6520", "NT6520"),
                TestGrid::new(512_300, 245_600, Precision::_100M, " TL123456 ", "TL123456"),
                TestGrid::new(503_400, 443_400, Precision::_100M, "Ta 0344 34", "TA034434"),
            ]
        }

        #[test]
        fn test_serde_serialize() {
            for grid in grids() {
                assert_eq!(
                    serde_json::to_string(
                        &OSGB::new(grid.eastings, grid.northings, grid.precision).unwrap()
                    )
                    .unwrap(),
                    format!("\"{}\"", grid.output_string)
                );
            }
        }

        #[test]
        fn test_serde_deserialize() {
            let from_str = serde_json::from_str::<OSGB>;

            for grid in grids() {
                let osgb: OSGB =
                    serde_json::from_str(&format!("\"{}\"", grid.input_string)).unwrap();

                assert_eq!(
                    osgb,
                    OSGB::new(grid.eastings, grid.northings, grid.precision).unwrap()
                );
            }
        }
    }
}
