use gridish::Precision;

#[derive(Clone)]
pub struct TestGrid {
    pub eastings: u32,
    pub northings: u32,
    pub precision: Precision,
    pub input_string: String,
    pub output_string: String,
}

impl TestGrid {
    pub fn new(
        eastings: u32,
        northings: u32,
        precision: Precision,
        input_string: &str,
        output_string: &str,
    ) -> TestGrid {
        TestGrid {
            eastings,
            northings,
            precision,
            input_string: input_string.to_string(),
            output_string: output_string.to_string(),
        }
    }
}

pub fn osgb_grids() -> Vec<TestGrid> {
    [
        TestGrid::new(300_000, 200_000, Precision::_100Km, "SO", "SO"),
        TestGrid::new(380_000, 240_000, Precision::_10Km, "SO84", "SO84"),
        TestGrid::new(389_000, 243_000, Precision::_1Km, "SO8943", "SO8943"),
        TestGrid::new(389_200, 243_700, Precision::_100M, "SO892437", "SO892437"),
        TestGrid::new(
            389_290,
            243_760,
            Precision::_10M,
            "SO89294376",
            "SO89294376",
        ),
        TestGrid::new(
            389_291,
            243_762,
            Precision::_1M,
            "SO8929143762",
            "SO8929143762",
        ),
        TestGrid::new(224_000, 668_000, Precision::_1Km, "ns 24 68", "NS2468"),
        TestGrid::new(365_000, 620_000, Precision::_1Km, "NT6520", "NT6520"),
        TestGrid::new(512_300, 245_600, Precision::_100M, " TL123456 ", "TL123456"),
        TestGrid::new(503_400, 443_400, Precision::_100M, "Ta 0344 34", "TA034434"),
    ]
    .to_vec()
}

pub fn osi_grids() -> Vec<TestGrid> {
    [
        TestGrid::new(300_000, 200_000, Precision::_100Km, "O", "O"),
        TestGrid::new(380_000, 240_000, Precision::_10Km, "O84", "O84"),
        TestGrid::new(389_000, 243_000, Precision::_1Km, "O8943", "O8943"),
        TestGrid::new(389_200, 243_700, Precision::_100M, "O892437", "O892437"),
        TestGrid::new(389_290, 243_760, Precision::_10M, "O89294376", "O89294376"),
        TestGrid::new(
            389_291,
            243_762,
            Precision::_1M,
            "O8929143762",
            "O8929143762",
        ),
        TestGrid::new(224_000, 168_000, Precision::_1Km, "s 24 68", "S2468"),
        TestGrid::new(365_000, 120_000, Precision::_1Km, "T6520", "T6520"),
        TestGrid::new(12_300, 245_600, Precision::_100M, " L123456 ", "L123456"),
        TestGrid::new(3_400, 443_400, Precision::_100M, "a 0344 34", "A034434"),
        TestGrid::new(
            315_904,
            234_671,
            Precision::_1M,
            "O1590434671",
            "O1590434671",
        ),
    ]
    .to_vec()
}
