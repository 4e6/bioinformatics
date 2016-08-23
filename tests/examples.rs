extern crate bio;

use std::process::Command;

use bio::data::Dataset;

static FAILED_TO_RUN_EXAMPLE: &'static str = "Failed to run example";

#[test]
fn greedy_motif_search() {
    let reference = Dataset::open_text("data/greedy_motif_search/dataset_159_5.out");
    let command = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--example")
        .arg("greedy_motif_search")
        .arg("data/greedy_motif_search/dataset_159_5.dat")
        .output()
        .expect(FAILED_TO_RUN_EXAMPLE);

    assert!(command.status.success());
    assert_eq!(command.stdout, reference.bytes);
}

#[test]
fn greedy_motif_search_with_pseudocounts() {
    let reference = Dataset::open_text("data/greedy_motif_search_with_pseudocounts/dataset_160_9.out");
    let command = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--example")
        .arg("greedy_motif_search")
        .arg("data/greedy_motif_search_with_pseudocounts/dataset_160_9.dat")
        .arg("with_pseudocounts")
        .output()
        .expect(FAILED_TO_RUN_EXAMPLE);

    assert!(command.status.success());
    assert_eq!(command.stdout, reference.bytes);
}

#[test]
fn most_probable_kmer() {
    let reference = Dataset::open_text("data/most_probable_kmer/dataset_159_3.out");
    let command = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--example")
        .arg("most_probable_kmer")
        .arg("data/most_probable_kmer/dataset_159_3.dat")
        .output()
        .expect(FAILED_TO_RUN_EXAMPLE);

    assert!(command.status.success());
    assert_eq!(command.stdout, reference.bytes);
}

#[test]
fn motif_enumeration() {
    let reference = Dataset::open_text("data/motif_enumeration/sample.out");

    let command = Command::new("cargo")
        .arg("run")
        .arg("--release")
        .arg("--example")
        .arg("motif_enumeration")
        .arg("data/motif_enumeration/sample.in")
        .output()
        .expect(FAILED_TO_RUN_EXAMPLE);

    assert!(command.status.success());
    assert_eq!(command.stdout, reference.bytes);
}
