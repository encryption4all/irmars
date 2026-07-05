//! A Rust client for [IRMA/Yivi](https://irma.app) servers.
//!
//! # Trusting a session result
//!
//! Completion is not the same as verification: an IRMA/Yivi server reports a
//! disclosure or signing session as `DONE` together with the disclosed
//! attributes even when the proof did not verify (for example an expired,
//! invalid, or unmatched proof). Callers must therefore never trust
//! [`SessionResult::disclosed`] on the basis of session completion alone.
//!
//! To make the safe path the default, [`IrmaClient::result`] only returns `Ok`
//! for a disclosure or signing session when the proof verified
//! (`proofStatus == VALID`); any other outcome is reported as
//! [`Error::ProofNotValid`]. A bare `Ok(_)` from `result` on such a session
//! thus guarantees a verified proof, and its `disclosed` attributes can be
//! trusted. Issuance sessions carry no disclosure proof and are gated on
//! completion only.
//!
//! When inspecting a [`SessionResult`] obtained by other means, check the proof
//! explicitly before trusting `disclosed`:
//!
//! ```no_run
//! # use irmars::{ProofStatus, SessionResult};
//! # fn handle(result: SessionResult) {
//! if matches!(result.proof_status, Some(ProofStatus::Valid)) {
//!     // safe to use result.disclosed
//! }
//! # }
//! ```

mod error;
mod irmaclient;
mod sessionrequest;
mod sessionresult;
mod util;

pub use error::Error;
pub use irmaclient::{
    FrontendRequest, IrmaClient, IrmaClientBuilder, Qr, SessionData, SessionToken,
};
pub use sessionrequest::{
    AttributeRequest, ConDisCon, Credential, CredentialBuilder, DisclosureRequestBuilder,
    ExtendedIrmaRequest, IrmaRequest, IssuanceRequestBuilder, NextSessionData,
    SignatureRequestBuilder,
};
pub use sessionresult::{
    AttributeStatus, DisclosedAttribute, ProofStatus, SessionResult, SessionStatus, SessionType,
};
pub use util::TranslatedString;
