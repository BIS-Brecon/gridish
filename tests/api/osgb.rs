use crate::data::osgb_grids;
use gridish::{Error, OSGB};

#[test]
fn parses_valid_strings() {
    let data = osgb_grids();

    for item in data {
        let grid: OSGB = item.input_string.parse().unwrap();

        assert_eq!(item.eastings, grid.sw().x());
        assert_eq!(item.northings, grid.sw().y());
        assert_eq!(item.precision, grid.precision());
    }
}

#[test]
fn rejects_invalid_strings() {
    assert_eq!(
        "TL123".parse::<OSGB>(),
        Err(Error::ParseError(
            "3 is not a valid number of digits. Supported values: 0, 2, 4, 6, 8, 10.".to_string()
        ))
    );

    assert_eq!(
        "123".parse::<OSGB>(),
        Err(Error::ParseError(
            "1 is not a valid grid square.".to_string()
        ))
    );

    assert_eq!(
        "T45".parse::<OSGB>(),
        Err(Error::ParseError(
            "4 is not a valid grid square.".to_string()
        ))
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
