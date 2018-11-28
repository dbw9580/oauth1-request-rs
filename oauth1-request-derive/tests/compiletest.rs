extern crate compiletest_rs as compiletest;

use std::process::Command;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();

    config.mode = mode.parse().expect("invalid mode");
    config.target_rustcflags = Some("-L test-deps/target/debug/deps".to_owned());
    config.src_base = format!("tests/{}", mode).into();

    let status = Command::new("cargo")
        .arg("build")
        .arg("--target-dir")
        .arg("test-deps/target")
        .arg("--manifest-path")
        .arg("test-deps/Cargo.toml")
        .status()
        .unwrap();
    if !status.success() {
        panic!("failed to build test dependencies");
    }

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
