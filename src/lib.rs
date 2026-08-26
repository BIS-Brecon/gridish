//! # gridish
//! A rust library for working with British and Irish national grids (OSGB, and OSI). Provides a simple interface for converting valid grid references into eastings / northings and vice versa, as well as functionality to recalculate a grid reference to a new precision. This crate intentionally does not provide functionality to convert between different coordinate systems, as there are already several libraries available to do this; it exists solely to fill the gap between numerical coordinates in eastings / northings and their textual representations.
//!
//! ## Examples
//! ```rust
//! use gridish::{OSGB, Resolution};
//! use geo_types::coord;
//!
//! // Parse grid reference from a 6 figure (100m) string.
//! let gridref_100m: OSGB = "SO892437".parse().unwrap();
//!
//! // Recalculate grid reference to 2 figures (10km)
//! let gridref_10k = gridref_100m.recalculate(Resolution::_10km);
//! assert_eq!("SO84".to_string(), gridref_10k.to_string());
//!
//! // Get the eastings / northings at the gridref's south west corner
//! assert_eq!(gridref_100m.south_west(), coord! {x: 389_200.0, y: 243_700.0 }.into());
//! assert_eq!(gridref_10k.south_west(), coord! {x: 380_000.0, y: 240_000.0 }.into());
//! ```
//!
//! ## Features
//! - `serde`: Provides support for (de)serialization using serde.
//! - `tetrads`: Provides support for tetrad grid references in the [DINTY](https://web.archive.org/web/20110527152140/http://www.kmbrc.org.uk/recording/help/gridrefhelp.php?page=6)
//! format, as commonly used in biological surveys.
//! - `quadrants`: Provides support for quadrants at the end of grid references.
//!
//! ### Tetrads
//! ```rust
//! # #[cfg(feature = "tetrads")]
//! # {
//! use gridish::OSGB;
//! use geo_types::coord;
//!
//! // Parse grid reference from a 2 figure with tetrad (2km) string.
//! let gridref_2k: OSGB = "SN24R".parse().unwrap();
//!
//! // Get the eastings / northings at the gridref's south west corner
//! assert_eq!(gridref_2k.south_west(), coord! {x: 226_000.0, y: 242_000.0 }.into());
//! # }
//! ```
//!
//! //! ### Quadrants
//! ```rust
//! # #[cfg(feature = "quadrants")]
//! # {
//! use gridish::OSGB;
//! use geo_types::coord;
//!
//! // Parse grid reference from a 2 figure with quadrant (5km) string.
//! let gridref_5k: OSGB = "SN24NW".parse().unwrap();
//!
//! // Get the eastings / northings at the gridref's south west corner
//! assert_eq!(gridref_5k.south_west(), coord! {x: 220_000.0, y: 245_000.0 }.into());
//! # }
//! ```

mod constants;
mod error;
mod grid;
mod grid_reference;
mod osbg;
mod osi;
mod resolution;
mod utils;

pub use error::{OutOfBoundsError, ParseError};
pub use osbg::OSGB;
pub use osi::OSI;
pub use resolution::Resolution;
