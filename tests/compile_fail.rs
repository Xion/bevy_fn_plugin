//! Test cases that are expected to fail compilation.

// spell-checker:ignore compiletest, filecheck, rmeta, rustcflags, tempdir

use std::env;
use std::path::PathBuf;

use compiletest_rs::Config;


fn main() {
    run_mode("compile-fail");
}

fn run_mode(mode: &str) {
    let mut config = Config::default().tempdir();

    config.mode = mode.parse().expect("invalid mode");
    config.src_base = PathBuf::from("tests").join(mode);
    config.target_rustcflags = Some("-L target/debug -L target/debug/deps".into());
    config.llvm_filecheck = Some(env::var("FILECHECK").unwrap_or("FileCheck".to_string()).into());
    config.clean_rmeta();

    config.filters = env::args().nth(1).into_iter().collect();

    compiletest_rs::run_tests(&config);
}

