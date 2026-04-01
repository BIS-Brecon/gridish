use gridish::Resolution;

#[derive(Clone)]
pub struct TestGrid {
    pub eastings: u32,
    pub northings: u32,
    pub resolution: Resolution,
    pub input_string: String,
    pub output_string: String,
}

impl TestGrid {
    pub fn new(
        eastings: u32,
        northings: u32,
        resolution: Resolution,
        input_string: &str,
        output_string: &str,
    ) -> TestGrid {
        TestGrid {
            eastings,
            northings,
            resolution,
            input_string: input_string.to_string(),
            output_string: output_string.to_string(),
        }
    }
}

pub fn osgb_grids() -> Vec<TestGrid> {
    [
        TestGrid::new(300_000, 200_000, Resolution::_100km, "SO", "SO"),
        TestGrid::new(380_000, 240_000, Resolution::_10km, "SO84", "SO84"),
        TestGrid::new(389_000, 243_000, Resolution::_1km, "SO 8943", "SO8943"),
        TestGrid::new(389_200, 243_700, Resolution::_100m, "SO 892 437", "SO892437"),
        TestGrid::new(
            389_290,
            243_760,
            Resolution::_10m,
            "SO89294376",
            "SO89294376",
        ),
        TestGrid::new(
            389_291,
            243_762,
            Resolution::_1m,
            "SO8929143762",
            "SO8929143762",
        ),
        TestGrid::new(224_000, 668_000, Resolution::_1km, "ns 2468", "NS2468"),
        TestGrid::new(365_000, 620_000, Resolution::_1km, "NT6520", "NT6520"),
        TestGrid::new(
            512_300,
            245_600,
            Resolution::_100m,
            "TL 123 456",
            "TL123456",
        ),
        TestGrid::new(503_400, 443_400, Resolution::_100m, "Ta034434", "TA034434"),
    ]
    .to_vec()
}

pub fn osi_grids() -> Vec<TestGrid> {
    [
        TestGrid::new(300_000, 200_000, Resolution::_100km, "O", "O"),
        TestGrid::new(380_000, 240_000, Resolution::_10km, "O84", "O84"),
        TestGrid::new(389_000, 243_000, Resolution::_1km, "O8943", "O8943"),
        TestGrid::new(389_200, 243_700, Resolution::_100m, "O892437", "O892437"),
        TestGrid::new(389_290, 243_760, Resolution::_10m, "O89294376", "O89294376"),
        TestGrid::new(
            389_291,
            243_762,
            Resolution::_1m,
            "O8929143762",
            "O8929143762",
        ),
        TestGrid::new(224_000, 168_000, Resolution::_1km, "s2468 ", "S2468"),
        TestGrid::new(365_000, 120_000, Resolution::_1km, "T6520", "T6520"),
        TestGrid::new(12_300, 245_600, Resolution::_100m, "L 123456", "L123456"),
        TestGrid::new(3_400, 443_400, Resolution::_100m, "a 034 434", "A034434"),
        TestGrid::new(
            315_904,
            234_671,
            Resolution::_1m,
            "O1590434671",
            "O1590434671",
        ),
    ]
    .to_vec()
}
