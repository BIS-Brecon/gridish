use crate::data::osi_grids;
use gridish::{Error, ParseError, OSI};

#[test]
fn parses_valid_strings() {
    let data = osi_grids();

    for item in data {
        let grid: OSI = item.input_string.parse().unwrap();

        assert_eq!(item.eastings, grid.sw().x() as u32);
        assert_eq!(item.northings, grid.sw().y() as u32);
        assert_eq!(item.precision, grid.precision());
    }
}

#[test]
fn rejects_invalid_strings() {
    assert_eq!(
        "L123".parse::<OSI>(),
        Err(Error::ParseError(ParseError::InvalidPrecision(3)))
    );

    assert_eq!(
        "123".parse::<OSI>(),
        Err(Error::ParseError(ParseError::InvalidSquare('1')))
    );
}

#[test]
fn prints_correct_strings() {
    let data = osi_grids();

    for item in data {
        let grid = OSI::new(item.eastings, item.northings, item.precision).unwrap();

        assert_eq!(item.output_string, grid.to_string());
    }
}
