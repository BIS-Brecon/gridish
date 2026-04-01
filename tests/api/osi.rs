use crate::data::osi_grids;
use gridish::{OSI, ParseError};

#[test]
fn parses_valid_strings() {
    let data = osi_grids();

    for item in data {
        let grid: OSI = item.input_string.parse().unwrap();

        assert_eq!(item.eastings, grid.south_west().x() as u32);
        assert_eq!(item.northings, grid.south_west().y() as u32);
        assert_eq!(item.resolution, grid.resolution());
    }
}

#[test]
fn rejects_invalid_strings() {
    assert_eq!("L123".parse::<OSI>(), Err(ParseError::InvalidResolution));

    assert_eq!("123".parse::<OSI>(), Err(ParseError::InvalidSquare('1')));
}

#[test]
fn prints_correct_strings() {
    let data = osi_grids();

    for item in data {
        let grid = OSI::new(item.eastings, item.northings, item.resolution).unwrap();

        assert_eq!(item.output_string, grid.to_string());
    }
}
