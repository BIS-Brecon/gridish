# gridish

A rust library for working with British and Irish national grids (OSGB, and OSI). Provides a simple interface for converting valid grid references into eastings / northings and vice versa, as well as functionality to recalculate a grid reference to a new precision. This crate intentionally does not provide functionality to convert between different coordinate systems, as there are already several libraries available to do this; it exists solely to fill the gap between numerical coordinates in eastings / northings and their textual representations.

## Examples

```
use gridish::{OSGB, Precision};
use geo_types::coord;

// Parse grid reference from a 6 figure (100m) string.
let gridref_100m: OSGB = "SO892437".parse().unwrap();

// Recalculate grid reference to 2 figures (10km)
let gridref_10k = gridref_100m.recalculate(Precision::_10Km);
assert_eq!("SO84".to_string(), gridref_10k.to_string());

// Get the eastings / northings at the gridref's south west corner
assert_eq!(gridref.sw(), coord! {x: 389_200, y: 243_700 }.into());
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