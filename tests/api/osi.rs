use crate::data::osi_grids;
use gridish::{Error, OSI};

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
        Err(Error::ParseError(
            "3 is not a valid number of digits. Supported values: 0, 2, 4, 6, 8, 10.".to_string()
        ))
    );

    assert_eq!(
        "123".parse::<OSI>(),
        Err(Error::ParseError(
            "1 is not a valid grid square.".to_string()
        ))
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
