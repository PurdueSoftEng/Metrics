// parse test output produced by test script generated with `./run test`
// returns (total tests, tests passed)
pub fn test_cases(filename: &str) -> Option<(u32, u32)> {
    let file = std::fs::read_to_string(filename).ok()?;
    let lines = file.lines();

    let mut tests = 0;
    let mut passed = 0;
    for l in lines {
        tests += 1;
        if let Some("ok") = l.split(' ').next() {
            passed += 1;
        }
    }

    Some((tests, passed))
}

// parse line coverage output produced by test script generated with `./run test`
// returns percentage of lines tested
pub fn code_coverage(filename: &str) -> Option<f64> {
    let file = std::fs::read_to_string(filename).ok()?;
    let json: serde_json::Value = serde_json::from_str(&file).ok()?;
    json["data"][0]["totals"]["lines"]["percent"].as_f64()
}
