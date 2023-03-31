//! Test cases that are expected to fail compilation.


// To bless the new or modified compiler output for compile-fail test cases, run:
//
// $ cargo test
// $ mv wip/*.stderr tests/compile-fail
//
// from the project root.


#[test]
fn compile_fail() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/compile-fail/*.rs");
}
