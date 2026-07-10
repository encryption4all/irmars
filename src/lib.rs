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
//! when any disclosure proof carried by the session verified
//! (`proofStatus == VALID`); any other outcome is reported as
//! [`Error::ProofNotValid`]. This applies to disclosure and signing sessions,
//! and equally to combined issuance+disclosure sessions (an issuance built with
//! [`IssuanceRequestBuilder::add_discon`], which the server reports as
//! `type=issuing` alongside a disclosure proof). A bare `Ok(_)` from `result`
//! thus guarantees any disclosed attributes come from a verified proof. A plain
//! issuance session carries no disclosure proof and is gated on completion
//! only.
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
