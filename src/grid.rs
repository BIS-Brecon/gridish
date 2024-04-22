use crate::Error;

/// A 5x5 grid made up of letters.
/// Used in grid references to break up
/// strings into 100km and 500km squares.
/// The origin is at the bottom left square: V.
const GRID_WIDTH: usize = 5;
const GRID: [char; 25] = [
    'V', 'W', 'X', 'Y', 'Z', 'Q', 'R', 'S', 'T', 'U', 'L', 'M', 'N', 'O', 'P', 'F', 'G', 'H', 'J',
    'K', 'A', 'B', 'C', 'D', 'E',
];

pub fn square_to_coords(square: &char) -> Result<(usize, usize), Error> {
    Ok(grid_to_coords(square, &GRID)?)
}

pub fn coords_to_square(column: usize, row: usize) -> Result<char, Error> {
    Ok(coords_to_grid(column, row, &GRID)?)
}

/// Return the coordinates of the given grid square.
/// This is zero-based and scale agnostic, so H => (1, 3);
fn grid_to_coords(square: &char, grid: &[char]) -> Result<(usize, usize), Error> {
    let index = grid
        .iter()
        .position(|x| x == square)
        .ok_or_else(|| Error::ParseError(format!("{square} is not a valid grid square.")))?;

    let column = index % GRID_WIDTH;
    let row = index / GRID_WIDTH;

    Ok((column, row))
}

/// Returns the grid square of the given coordinates.
/// This is zero-based and scale agnostic, so (1, 1) => R;
fn coords_to_grid(column: usize, row: usize, grid: &[char]) -> Result<char, Error> {
    if column >= GRID_WIDTH || row >= GRID_WIDTH {
        Err(Error::OutOfBounds)
    } else {
        let index = column + (GRID_WIDTH * row);

        Ok(*grid.get(index).ok_or_else(|| Error::OutOfBounds)?)
    }
}

/// The grid used for tetrad coordinates.
#[cfg(feature = "tetrads")]
const TETRAD_GRID: [char; 25] = [
    'A', 'F', 'K', 'Q', 'V', 'B', 'G', 'L', 'R', 'W', 'C', 'H', 'M', 'S', 'X', 'D', 'I', 'N', 'T',
    'Y', 'E', 'J', 'P', 'U', 'Z',
];

#[cfg(feature = "tetrads")]
pub fn tetrad_to_coords(square: &char) -> Result<(usize, usize), Error> {
    Ok(grid_to_coords(square, &TETRAD_GRID)?)
}

#[cfg(feature = "tetrads")]
pub fn coords_to_tetrad(column: usize, row: usize) -> Result<char, Error> {
    Ok(coords_to_grid(column, row, &TETRAD_GRID)?)
}

#[cfg(test)]
mod test {
    use crate::{
        grid::{coords_to_square, square_to_coords, GRID, GRID_WIDTH},
        Error,
    };

    const VALID_SQUARES: [(char, (usize, usize)); 25] = [
        ('A', (0, 4)),
        ('B', (1, 4)),
        ('C', (2, 4)),
        ('D', (3, 4)),
        ('E', (4, 4)),
        ('F', (0, 3)),
        ('G', (1, 3)),
        ('H', (2, 3)),
        ('J', (3, 3)),
        ('K', (4, 3)),
        ('L', (0, 2)),
        ('M', (1, 2)),
        ('N', (2, 2)),
        ('O', (3, 2)),
        ('P', (4, 2)),
        ('Q', (0, 1)),
        ('R', (1, 1)),
        ('S', (2, 1)),
        ('T', (3, 1)),
        ('U', (4, 1)),
        ('V', (0, 0)),
        ('W', (1, 0)),
        ('X', (2, 0)),
        ('Y', (3, 0)),
        ('Z', (4, 0)),
    ];

    #[test]
    fn grid_is_correct_size() {
        assert_eq!(GRID_WIDTH * GRID_WIDTH, GRID.len())
    }

    #[test]
    fn valid_letters_return_coords() {
        for square in VALID_SQUARES {
            assert_eq!(square_to_coords(&square.0), Ok(square.1));
        }
    }

    #[test]
    fn invalid_letters_are_rejected() {
        let squares = ['a', 'I', '0', '@'];

        for square in squares {
            assert_eq!(
                square_to_coords(&square),
                Err(Error::ParseError(format!(
                    "{square} is not a valid grid square."
                )))
            );
        }
    }

    #[test]
    fn valid_coords_return_letter() {
        for square in VALID_SQUARES {
            assert_eq!(coords_to_square(square.1 .0, square.1 .1), Ok(square.0));
        }
    }

    #[test]
    fn invalid_coords_are_rejected() {
        let coords = [(0, 5), (5, 0)];

        for coord in coords {
            assert_eq!(coords_to_square(coord.0, coord.1), Err(Error::OutOfBounds));
        }
    }
}

#[cfg(feature = "tetrads")]
#[cfg(test)]
mod test_tetrad {
    use crate::grid::{coords_to_tetrad, tetrad_to_coords};
    use crate::Error;

    const VALID_TETRADS: [(char, (usize, usize)); 25] = [
        ('A', (0, 0)),
        ('B', (0, 1)),
        ('C', (0, 2)),
        ('D', (0, 3)),
        ('E', (0, 4)),
        ('F', (1, 0)),
        ('G', (1, 1)),
        ('H', (1, 2)),
        ('I', (1, 3)),
        ('J', (1, 4)),
        ('K', (2, 0)),
        ('L', (2, 1)),
        ('M', (2, 2)),
        ('N', (2, 3)),
        ('P', (2, 4)),
        ('Q', (3, 0)),
        ('R', (3, 1)),
        ('S', (3, 2)),
        ('T', (3, 3)),
        ('U', (3, 4)),
        ('V', (4, 0)),
        ('W', (4, 1)),
        ('X', (4, 2)),
        ('Y', (4, 3)),
        ('Z', (4, 4)),
    ];

    #[test]
    fn valid_letters_return_coords_tetrad() {
        for square in VALID_TETRADS {
            assert_eq!(tetrad_to_coords(&square.0), Ok(square.1));
        }
    }

    #[test]
    fn invalid_letters_are_rejected_tetrad() {
        let squares = ['a', 'O', '0', '@'];

        for square in squares {
            assert_eq!(
                tetrad_to_coords(&square),
                Err(Error::ParseError(format!(
                    "{square} is not a valid grid square."
                )))
            );
        }
    }

    #[test]
    fn valid_coords_return_letter_tetrad() {
        for square in VALID_TETRADS {
            assert_eq!(coords_to_tetrad(square.1 .0, square.1 .1), Ok(square.0));
        }
    }

    #[test]
    fn invalid_coords_are_rejected_tetrad() {
        let coords = [(0, 5), (5, 0)];

        for coord in coords {
            assert_eq!(coords_to_tetrad(coord.0, coord.1), Err(Error::OutOfBounds));
        }
    }
}
