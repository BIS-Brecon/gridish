//! # gridish
//! A rust library for working with British and Irish national grids (OSGB, and OSI). Provides a simple interface for converting valid grid references into eastings / northings and vice versa, as well as functionality to recalculate a grid reference to a new precision. This crate intentionally does not provide functionality to convert between different coordinate systems, as there are already several libraries available to do this; it exists solely to fill the gap between numerical coordinates in eastings / northings and their textual representations.
//!
//! ## Examples
//! ```
//! use gridish::{OSGB, Precision};
//! use geo_types::coord;
//!
//! // Parse grid reference from a 6 figure (100m) string.
//! let gridref_100m: OSGB = "SO892437".parse().unwrap();
//!
//! // Recalculate grid reference to 2 figures (10km)
//! let gridref_10k = gridref_100m.recalculate(Precision::_10Km);
//! assert_eq!("SO84".to_string(), gridref_10k.to_string());
//!
//! // Get the eastings / northings at the gridref's south west corner
//! assert_eq!(gridref_100m.sw(), coord! {x: 389_200, y: 243_700 }.into());
//! assert_eq!(gridref_10k.sw(), coord! {x: 380_000, y: 240_000 }.into());
//! ```
//!
//! ## Features
//! Gridish can support tetrad grid references in the [DINTY](https://web.archive.org/web/20110527152140/http://www.kmbrc.org.uk/recording/help/gridrefhelp.php?page=6)
//! format as commonly used in biological surveys by using the feature flag `tetrads`.
//!
//! ```
//! # #[cfg(feature = "tetrads")]
//! # {
//! use gridish::{OSGB, Precision};
//! use geo_types::coord;
//!
//! // Parse grid reference from a 6 figure (100m) string.
//! let gridref_2k: OSGB = "SN24R".parse().unwrap();
//!
//! // Get the eastings / northings at the gridref's south west corner
//! assert_eq!(gridref_2k.sw(), coord! {x: 226_000, y: 242_000 }.into());
//! # }
//! ```

mod constants;
mod coordinates;
mod error;
mod grid;
mod osgb;
mod osi;
mod precision;
mod utils;

pub use error::Error;
pub use osgb::OSGB;
pub use osi::OSI;
pub use precision::Precision;
