use crate::data::osgb_grids;
use gridish::{Error, ParseError, OSGB};

#[test]
fn parses_valid_strings() {
    let data = osgb_grids();

    for item in data {
        let grid: OSGB = item.input_string.parse().unwrap();

        assert_eq!(item.eastings, grid.sw().x() as u32);
        assert_eq!(item.northings, grid.sw().y() as u32);
        assert_eq!(item.precision, grid.precision());
    }
}

#[test]
fn rejects_invalid_strings() {
    assert_eq!(
        "TL123".parse::<OSGB>(),
        Err(Error::ParseError(ParseError::InvalidPrecision(3)))
    );

    assert_eq!(
        "123".parse::<OSGB>(),
        Err(Error::ParseError(ParseError::InvalidSquare('1')))
    );

    assert_eq!(
        "T45".parse::<OSGB>(),
        Err(Error::ParseError(ParseError::InvalidSquare('4')))
    );
}

#[test]
fn prints_correct_strings() {
    let data = osgb_grids();

    for item in data {
        let grid = OSGB::new(item.eastings, item.northings, item.precision).unwrap();

        assert_eq!(item.output_string, grid.to_string());
    }
}
