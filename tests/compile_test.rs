//! Test cases for a particular compiler output when using the proc-macro crate.


// To add the (expected) compiler output of new compile test cases, run:
//
// $ cargo test
// $ mv wip/*.stderr tests/compile-fail

// To bless the modified output, run:
//
// $ TRYBUILD=overwrite cargo test


/// Test compiler failures.
#[test]
fn compile_fail() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/compile-fail/*.rs");
}

/// Test compiler warnings.
///
/// This is still a compile_fail() tests because we are turning expected warnings into errors
/// in the actual test cases.
#[test]
fn compile_warn() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/compile-warn/*.rs");
}
