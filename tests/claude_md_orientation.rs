//! `CLAUDE.md` is orientation: what this repo is, the position it takes, and the
//! sibling repos to consider before changing it. Detail that is documentation
//! belongs at `docs.postguard.eu/repos/irmars`; detail that is a durable check
//! belongs in the agent rule bundle. This test is what stops the file from
//! filling back up with everything an agent ever noticed here.

use std::{fs, path::PathBuf};

/// The host's own threshold, not a cosmetic one: above 4,000 B a container
/// working this repo stops getting its cwd pointed at the checkout
/// (encryption4all/dobby-code#482). Raising it should be a decision.
const MAX_BYTES: u64 = 4_000;

#[test]
fn claude_md_stays_orientation_sized() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("CLAUDE.md");
    let bytes = fs::metadata(&path)
        .expect("CLAUDE.md is missing from the repository root")
        .len();

    assert!(
        bytes <= MAX_BYTES,
        "CLAUDE.md is {bytes} B, over the {MAX_BYTES} B cap. It is orientation only: what this \
         crate is, where it sits between Yivi and PostGuard, and which repos to consider before \
         changing it. Documentation goes to docs.postguard.eu/repos/irmars, a durable check goes \
         to the agent rule bundle, and an invariant goes next to the code that enforces it."
    );
}
