extern crate bio;

use std::process::Command;

use bio::data::Dataset;

static FAILED_TO_RUN_EXAMPLE: &'static str = "Failed to run example";

#[test]
fn motif_enumeration() {
    let dataset = Dataset::open_text("data/motif_enumeration/sample.out");

    let command = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--example")
        .arg("motif_enumeration")
        .arg("data/motif_enumeration/sample.in")
        .output()
        .expect(FAILED_TO_RUN_EXAMPLE);

    assert!(command.status.success());
    assert_eq!(command.stdout, dataset.bytes);
}
