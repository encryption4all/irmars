# irmars

A Rust wrapper around the IRMA/Yivi server REST API. `IrmaClient`
(`src/irmaclient.rs`) starts and polls disclosure, signature and issuance
sessions; the request and result types mirror the server's JSON. Published to
crates.io as `irmars`, released by release-plz off Conventional Commits.

**Interfaces may change.** The README warns that the library is still under
development and its interfaces may not be stable. That warning still holds and
is the first thing to know before building on the crate.

## Position

One company, two GitHub orgs. `privacybydesign` is the Yivi/IRMA lineage;
`encryption4all` is the vehicle the PostGuard research project used to apply for
grants, kept as an org after Yivi bought PostGuard to commercialise it. The
split is historical, not organisational: same maintainers, same review
conventions, and we are maintainers here, not upstream contributors.

This crate is where the two sides meet. PostGuard authenticates an identity
through a Yivi disclosure before its PKG hands out a decryption key, and this is
the Rust client that runs that session.

## Other repos to consider before changing this one

- `privacybydesign/irmago` — the server this is a client of. Its session
  endpoints and its `SessionResult`/`proofStatus` JSON are the contract these
  types mirror, so a change there can invalidate them without touching this
  repo. CI pins the server binary at a release tag (`.github/workflows/rust.yml`).
- `encryption4all/postguard` — `pg-core`, `pg-pkg` and `pg-cli` depend on this
  crate under a rename: `irma = { package = "irmars", version = "0.2.2" }`. The
  coupling is a crates.io version pin, not the repo, so nothing landed here
  reaches PostGuard until a release is published and that pin moves, security
  fixes included.
- `tweedegolf/irmars` — the upstream this repo forks from; its `main` has not
  moved since 2021, though a side branch saw one commit in 2023. Not synced; do
  not expect changes to flow either way.
- `privacybydesign/irmars` — an archived namesake, not this repo. Pass
  `--repo encryption4all/irmars` to `gh` so a command cannot land on it.

## What is not in this file

Documentation belongs at `docs.postguard.eu/repos/irmars` (postguard-docs,
`docs/repos/irmars.md`). A durable check belongs in the rule bundle the host
lands in each container at `~/dobby-rules.md`, one rule per check. The crate's
own invariants are documented where they are enforced: `src/lib.rs`'s module
docs explain why a completed session is not a verified one, and unit tests in
`src/irmaclient.rs` pin it.

The agent notes this file used to be are in git history: 1,334 bytes at
`e6fd5dd`, the last revision carrying them (`git show e6fd5dd:CLAUDE.md`).
`tests/claude_md_orientation.rs` holds this file at orientation size.
