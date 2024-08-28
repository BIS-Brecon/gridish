use crate::utils::trim_string;
use crate::{coordinates::point::Point as GridPoint, Error, Precision};
use geo_types::{LineString, Point, Polygon};
use std::fmt::Display;
use std::str::FromStr;

/// Type representing a valid Irish National Grid Reference.
/// Can be instantiated either by parsing from a string or through
/// a valid set of eastings and northings as coordinates.
///
/// Provides functionality to convert between strings and coordinates,
/// as well as re-mapping to a new precision.
// Works as a simple wrapper around Point, with some additional methods.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OSI {
    point: GridPoint,
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
    /// use gridish::{OSI, Precision};
    ///
    /// let gridref = OSI::new(
    ///     389_200,
    ///     243_700,
    ///     Precision::_100M
    /// ).unwrap();
    ///
    /// assert_eq!(gridref.to_string(), "O892437".to_string());
    /// ```
    pub fn new(eastings: u32, northings: u32, precision: Precision) -> Result<Self, Error> {
        let eastings = eastings.try_into()?;
        let northings = northings.try_into()?;

        Ok(Self {
            point: GridPoint::new(eastings, northings, precision),
        })
    }

    /// Recalculates the grid reference to a new precision.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSI, Precision};
    ///
    /// let gridref_100m: OSI = "O892437".parse().unwrap();
    /// let gridref_10k = gridref_100m.recalculate(Precision::_10Km);
    ///
    /// assert_eq!("O84".to_string(), gridref_10k.to_string());
    /// ```
    pub fn recalculate(&self, precision: Precision) -> Self {
        if precision > self.point.precision() {
            //
            self.clone()
        } else {
            let point = GridPoint::new(self.point.eastings(), self.point.northings(), precision);

            Self { point }
        }
    }

    /// Returns the point at the OSI's
    /// 'South West' corner - its origin.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.sw(), coord! {x: 389_200.0, y: 243_700.0 }.into());
    /// ```
    pub fn sw(&self) -> Point {
        Point::new(self.point.eastings().into(), self.point.northings().into())
    }

    /// Returns the point at the OSI's
    /// 'North West' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.nw(), coord! {x: 389_200.0, y: 243_800.0 }.into());
    /// ```
    pub fn nw(&self) -> Point {
        Point::new(
            self.point.eastings().inner() as f64,
            (self.point.northings().inner() + self.point.precision().metres()) as f64,
        )
    }

    /// Returns the point at the OSI's
    /// 'North East' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.ne(), coord! {x: 389_300.0, y: 243_800.0 }.into());
    /// ```
    pub fn ne(&self) -> Point {
        Point::new(
            (self.point.eastings().inner() + self.point.precision().metres()) as f64,
            (self.point.northings().inner() + self.point.precision().metres()) as f64,
        )
    }

    /// Returns the point at the OSI's
    /// 'South East' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.se(), coord! {x: 389_300.0, y: 243_700.0 }.into());
    /// ```
    pub fn se(&self) -> Point {
        Point::new(
            (self.point.eastings().inner() + self.point.precision().metres()) as f64,
            self.point.northings().inner() as f64,
        )
    }

    /// Returns the point at the OSI's
    /// centre.
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
            self.point.eastings().inner() as f64 + (self.point.precision().metres() as f64 / 2.0),
            self.point.northings().inner() as f64 + (self.point.precision().metres() as f64 / 2.0),
        )
    }

    /// Returns the OSI's perimeter.
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
            LineString::from(vec![self.sw(), self.nw(), self.ne(), self.se()]),
            vec![],
        )
    }

    /// Returns the OSI's precision.
    ///
    /// # Example
    /// ```
    /// use gridish::{OSI, Precision};
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.precision(), Precision::_100M);
    /// ```
    pub fn precision(&self) -> Precision {
        self.point.precision()
    }
}

impl FromStr for OSI {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: String = trim_string(s);
        let point: GridPoint = string.parse()?;

        Ok(Self { point })
    }
}

impl Display for OSI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.point)
    }
}

#[cfg(test)]
mod test {
    use crate::OSI;
    use geo_types::{LineString, Point, Polygon};

    #[test]
    fn coordinates_are_correct() {
        let osi = OSI::new(0, 0, crate::Precision::_100M).unwrap();
        let sw = osi.sw();
        let nw = osi.nw();
        let ne = osi.ne();
        let se = osi.se();

        assert_eq!(sw, Point::new(0.0, 0.0));
        assert_eq!(nw, Point::new(0.0, 100.0));
        assert_eq!(ne, Point::new(100.0, 100.0));
        assert_eq!(se, Point::new(100.0, 0.0));
        assert_eq!(osi.centre(), Point::new(50.0, 50.0));
        assert_eq!(
            osi.perimeter(),
            Polygon::new(LineString::from(vec![sw, nw, ne, se]), vec![])
        )
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

    #[cfg(test)]
    mod tests {
        use crate::{grid, Precision, OSI};

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

        fn grids() -> [TestGrid; 11] {
            [
                TestGrid::new(300_000, 200_000, Precision::_100Km, "O", "O"),
                TestGrid::new(380_000, 240_000, Precision::_10Km, "O84", "O84"),
                TestGrid::new(389_000, 243_000, Precision::_1Km, "O8943", "O8943"),
                TestGrid::new(389_200, 243_700, Precision::_100M, "O892437", "O892437"),
                TestGrid::new(389_290, 243_760, Precision::_10M, "O89294376", "O89294376"),
                TestGrid::new(
                    389_291,
                    243_762,
                    Precision::_1M,
                    "O8929143762",
                    "O8929143762",
                ),
                TestGrid::new(224_000, 168_000, Precision::_1Km, "s 24 68", "S2468"),
                TestGrid::new(365_000, 120_000, Precision::_1Km, "T6520", "T6520"),
                TestGrid::new(12_300, 245_600, Precision::_100M, " L123456 ", "L123456"),
                TestGrid::new(3_400, 443_400, Precision::_100M, "a 0344 34", "A034434"),
                TestGrid::new(
                    315_904,
                    234_671,
                    Precision::_1M,
                    "O1590434671",
                    "O1590434671",
                ),
            ]
        }

        #[test]
        fn test_serde_serialize() {
            for grid in grids() {
                assert_eq!(
                    serde_json::to_string(
                        &OSI::new(grid.eastings, grid.northings, grid.precision).unwrap()
                    )
                    .unwrap(),
                    format!("\"{}\"", grid.output_string)
                );
            }
        }

        #[test]
        fn test_serde_deserialize() {
            let from_str = serde_json::from_str::<OSI>;

            for grid in grids() {
                let OSI: OSI = serde_json::from_str(&format!("\"{}\"", grid.input_string)).unwrap();

                assert_eq!(
                    OSI,
                    OSI::new(grid.eastings, grid.northings, grid.precision).unwrap()
                );
            }
        }
    }
}
