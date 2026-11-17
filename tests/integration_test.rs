// Integration test for SigmaOS-standard-notes
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_standard_notes as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
