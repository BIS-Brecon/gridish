use crate::ParseError;

/// A 5x5 grid made up of letters.
/// Used in grid references to break up
/// strings into 100km and 500km squares.
/// The origin is at the bottom left square: V.
const GRID_WIDTH: usize = 5;
pub(crate) const GRID: [char; 25] = [
    'V', 'W', 'X', 'Y', 'Z', 'Q', 'R', 'S', 'T', 'U', 'L', 'M', 'N', 'O', 'P', 'F', 'G', 'H', 'J',
    'K', 'A', 'B', 'C', 'D', 'E',
];

/// Returns the grid square of the given coordinates.
/// This is zero-based and scale agnostic, so (1, 1) => R;
pub(crate) fn coords_to_grid(column: usize, row: usize, grid: &[char]) -> char {
    let index = column + (GRID_WIDTH * row);

    grid[index]
}

/// Return the coordinates of the given grid square.
/// This is zero-based and scale agnostic, so H => (1, 3);
pub(crate) fn grid_to_coords(square: &char, grid: &[char]) -> Result<(usize, usize), ParseError> {
    let index = grid
        .iter()
        .position(|x| x.eq_ignore_ascii_case(square))
        .ok_or_else(|| ParseError::InvalidSquare(*square))?;

    let column = index % GRID_WIDTH;
    let row = index / GRID_WIDTH;

    Ok((column, row))
}

/// The grid used for tetrad coordinates.
#[cfg(feature = "tetrads")]
pub(crate) const TETRAD_GRID: [char; 25] = [
    'A', 'F', 'K', 'Q', 'V', 'B', 'G', 'L', 'R', 'W', 'C', 'H', 'M', 'S', 'X', 'D', 'I', 'N', 'T',
    'Y', 'E', 'J', 'P', 'U', 'Z',
];
