//! Test cases that are expected to fail compilation.

#[test]
fn compile_fail() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/compile-fail/*.rs");
}
