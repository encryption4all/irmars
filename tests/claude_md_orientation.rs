//! `CLAUDE.md` is orientation only. This test is what stops it from filling back
//! up with everything an agent ever noticed in this repo.

use std::{fs, path::PathBuf};

/// The host's threshold, not a cosmetic one: above 4,000 B a container working
/// this repo stops getting its cwd pointed at the checkout — the `choose_cwd`
/// gate in dobby-code's `docker/entrypoint.sh`
/// (encryption4all/dobby-code#679, decided in #482). Raising it should be a
/// decision.
const MAX_BYTES: u64 = 4_000;

#[test]
fn claude_md_stays_orientation_sized() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("CLAUDE.md");
    let bytes = fs::metadata(&path)
        .expect("CLAUDE.md is missing from the repository root")
        .len();

    assert!(
        bytes <= MAX_BYTES,
        "CLAUDE.md is {bytes} B, over the {MAX_BYTES} B cap. Documentation belongs at \
         docs.postguard.eu/repos/irmars, a durable check belongs in the agent rule bundle, and an \
         invariant belongs next to the code that enforces it."
    );
}
