## Agent notes (migrated from the dobby memory repo)

## Overview
`encryption4all/irmars` is a public Rust fork of `tweedegolf/irmars`, a client
library for IRMA/Yivi disclosure, signing, and issuance sessions (`IrmaClient` in
`src/irmaclient.rs`). This repo is ambiguous with an archived
`privacybydesign/irmars`; always pass `--repo encryption4all/irmars` explicitly to
`gh pr create` and other `gh` commands, or they can target the wrong repo. Default
branch is `main`.

## Security invariant: result() must enforce proof_status, not just session status
`IrmaClient::result()` must reject results unless the proof is actually valid. A
session with `status == Done` can still carry a `proof_status` of
`Invalid`/`Expired`/`InvalidTimestamp`/`UnmatchedRequest`/`MissingAttributes`, and
a consumer that only checks `Ok(result)` for `status == Done` will trust
attributes from an invalid or expired proof. `result()` must enforce
`proof_status == Some(ProofStatus::Valid)` both for Disclosing/Signing sessions
and for Issuing sessions that carry an embedded disclosure component (detect via
`proof_status.is_some() || !disclosed.is_empty()`); plain issuance with no
disclosure component is fine gated on session completion alone. Use the shared
`enforce_proof_valid` helper for this check rather than re-deriving it at each
call site.
