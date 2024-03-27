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
    /// and precision. Returns an error if it is out of bounds.
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

    /// Returns the point at the osgb's
    /// 'South West' corner - its origin.
    /// Recalculates the grid reference to a new precision.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.sw(), coord! {x: 389_200, y: 243_700 }.into());
    /// ```
    pub fn sw(&self) -> Point<u32> {
        Point::new(self.point.eastings().into(), self.point.northings().into())
    }

    /// Returns the point at the osgb's
    /// 'North West' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.nw(), coord! {x: 389_200, y: 243_800 }.into());
    /// ```
    pub fn nw(&self) -> Point<u32> {
        Point::new(
            self.point.eastings().inner(),
            self.point.northings().inner() + self.point.precision().metres(),
        )
    }

    /// Returns the point at the osgb's
    /// 'North East' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.ne(), coord! {x: 389_300, y: 243_800 }.into());
    /// ```
    pub fn ne(&self) -> Point<u32> {
        Point::new(
            self.point.eastings().inner() + self.point.precision().metres(),
            self.point.northings().inner() + self.point.precision().metres(),
        )
    }

    /// Returns the point at the osgb's
    /// 'South East' corner.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.se(), coord! {x: 389_300, y: 243_700 }.into());
    /// ```
    pub fn se(&self) -> Point<u32> {
        Point::new(
            self.point.eastings().inner() + self.point.precision().metres(),
            self.point.northings().inner(),
        )
    }

    /// Returns the point at the osgb's
    /// centre.
    ///
    /// # Example
    /// ```
    /// use gridish::OSI;
    /// use geo_types::coord;
    ///
    /// let gridref: OSI = "O892437".parse().unwrap();
    ///
    /// assert_eq!(gridref.centre(), coord! {x: 389_250, y: 243_750 }.into());
    /// ```
    pub fn centre(&self) -> Point<u32> {
        Point::new(
            self.point.eastings().inner() + (self.point.precision().metres() / 2),
            self.point.northings().inner() + (self.point.precision().metres() / 2),
        )
    }

    /// Returns the osgb's perimeter.
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
        let string: String = s
            .to_uppercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
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

        assert_eq!(sw, Point::new(0, 0));
        assert_eq!(nw, Point::new(0, 100));
        assert_eq!(ne, Point::new(100, 100));
        assert_eq!(se, Point::new(100, 0));
        assert_eq!(osi.centre(), Point::new(50, 50));
        assert_eq!(
            osi.perimeter(),
            Polygon::new(LineString::from(vec![sw, nw, ne, se]), vec![])
        )
    }
}
