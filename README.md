# gridish
A rust library for working with British and Irish national grids (OSGB, and OSI). Provides a simple interface for converting valid grid references into eastings / northings and vice versa, as well as functionality to recalculate a grid reference to a new precision. This crate intentionally does not provide functionality to convert between different coordinate systems, as there are already several libraries available to do this; it exists solely to fill the gap between numerical coordinates in eastings / northings and their textual representations.

## Examples
```rust
use gridish::{OSGB, Resolution};
use geo_types::coord;

// Parse grid reference from a 6 figure (100m) string.
let gridref_100m: OSGB = "SO892437".parse().unwrap();

// Recalculate grid reference to 2 figures (10km)
let gridref_10k = gridref_100m.recalculate(Resolution::_10km);
assert_eq!("SO84".to_string(), gridref_10k.to_string());

// Get the eastings / northings at the gridref's south west corner
assert_eq!(gridref_100m.south_west(), coord! {x: 389_200.0, y: 243_700.0 }.into());
assert_eq!(gridref_10k.south_west(), coord! {x: 380_000.0, y: 240_000.0 }.into());
```

## Features
- `serde`: Provides support for (de)serialization using serde.
- `tetrads`: Provides support for tetrad grid references in the [DINTY](https://web.archive.org/web/20110527152140/http://www.kmbrc.org.uk/recording/help/gridrefhelp.php?page=6)
format, as commonly used in biological surveys.
- `quadrants`: Provides support for quadrants at the end of grid references.

### Tetrads
```rust
use gridish::OSGB;
use geo_types::coord;

// Parse grid reference from a 2 figure with tetrad (2km) string.
let gridref_2k: OSGB = "SN24R".parse().unwrap();

// Get the eastings / northings at the gridref's south west corner
assert_eq!(gridref_2k.south_west(), coord! {x: 226_000.0, y: 242_000.0 }.into());
```

### Quadrants
```rust
use gridish::OSGB;
use geo_types::coord;

// Parse grid reference from a 2 figure with quadrant (5km) string.
let gridref_5k: OSGB = "SN24NW".parse().unwrap();

// Get the eastings / northings at the gridref's south west corner
assert_eq!(gridref_5k.south_west(), coord! {x: 220_000.0, y: 245_000.0 }.into());
```

## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

All contributions from humans and other sentient organisms are welcome. Any contributions made with the aid of Generative AI, or where you as an individual are not the author, are not.

Please open an [issue](https://github.com/BIS-Brecon/gridish/issues) for discussion before adding any new features.
