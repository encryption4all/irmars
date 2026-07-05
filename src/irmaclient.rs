use std::fmt::Debug;

use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

use crate::{
    sessionrequest::ExtendedIrmaRequest, Error, IrmaRequest, ProofStatus, SessionResult,
    SessionStatus, SessionType,
};

#[derive(Clone, Debug)]
enum AuthMethod {
    None,
    Token(TokenSecret),
}

#[derive(Clone)]
struct TokenSecret {
    token: String,
}

/// The information contained in the QR displayed to the end user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qr {
    #[doc(hidden)]
    pub u: String,
    #[doc(hidden)]
    pub irmaqr: SessionType,
}

/// Information received on session start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    /// The data for the QR to be shown to the end user
    #[serde(rename = "sessionPtr")]
    pub session_ptr: Qr,
    /// The token for further interaction with the session
    pub token: SessionToken,
    /// Information needed to drive the IRMA/Yivi frontend directly (e.g. for
    /// pairing). Present since irmago v0.14.0; `None` when the server does not
    /// return a `frontendRequest` block.
    #[serde(
        rename = "frontendRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frontend_request: Option<FrontendRequest>,
}

/// The `frontendRequest` block returned by irmago on session start, used to
/// communicate with the IRMA/Yivi frontend directly.
///
/// Since pairing is mandatory by default for IRMA clients (irmago v0.13.0),
/// the [`authorization`](Self::authorization) token is required to complete the
/// pairing handshake.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct FrontendRequest {
    /// Authorization token used to authenticate to the frontend endpoints.
    pub authorization: String,
    /// The lowest frontend protocol version the server supports, if reported.
    #[serde(
        rename = "minProtocolVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_protocol_version: Option<String>,
    /// The highest frontend protocol version the server supports, if reported.
    #[serde(
        rename = "maxProtocolVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_protocol_version: Option<String>,
}

/// Token used to identify individual sessions on the server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SessionToken(pub String);

// We manually implement debug to protect against accidentally leaking the secret through debug printing.
impl Debug for TokenSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenSecret").finish()
    }
}

/// Client for interacting with an irma server
#[derive(Debug, Clone)]
pub struct IrmaClient {
    url: Url,
    client: Client,
    authmethod: AuthMethod,
}

impl IrmaClient {
    /// Create a new client without any authentication or other special options
    pub fn new(url: &str) -> Result<IrmaClient, Error> {
        Ok(IrmaClient {
            url: Url::parse(url)?,
            client: Client::new(),
            authmethod: AuthMethod::None,
        })
    }

    /// Start an IRMA session with the given request
    pub async fn request(&self, request: &IrmaRequest) -> Result<SessionData, Error> {
        let req = self.client.post(self.url.join("session").unwrap());
        let req = match &self.authmethod {
            AuthMethod::None => req,
            AuthMethod::Token(TokenSecret { token }) => req.header("Authorization", token),
        };
        Ok(req
            .json(request)
            .send()
            .await?
            .error_for_status()?
            .json::<SessionData>()
            .await?)
    }

    /// Start an IRMA session with the given extended request (note: this interface is unstable, and might change significantly in the future)
    pub async fn request_extended(
        &self,
        request: &ExtendedIrmaRequest,
    ) -> Result<SessionData, Error> {
        let req = self.client.post(self.url.join("session").unwrap());
        let req = match &self.authmethod {
            AuthMethod::None => req,
            AuthMethod::Token(TokenSecret { token }) => req.header("Authorization", token),
        };
        Ok(req
            .json(request)
            .send()
            .await?
            .error_for_status()?
            .json::<SessionData>()
            .await?)
    }

    /// Get the status of a previously started irma session
    pub async fn status(&self, token: &SessionToken) -> Result<SessionStatus, Error> {
        Ok(self
            .client
            .get(self.url.join(&format!("session/{}/status", token.0))?)
            .send()
            .await?
            .error_for_status()?
            .json::<SessionStatus>()
            .await?)
    }

    /// Cancel a previously started session
    pub async fn cancel(&self, token: &SessionToken) -> Result<(), Error> {
        self.client
            .delete(self.url.join(&format!("session/{}", token.0))?)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// Get the result for a previously started irma session.
    ///
    /// For disclosure and signing sessions a successful (`Ok`) return
    /// guarantees the proof was cryptographically verified by the server
    /// (`proofStatus == VALID`); a completed session whose proof did not verify
    /// yields [`Error::ProofNotValid`]. Issuance sessions carry no proof to
    /// disclose and are therefore gated on completion only. This means callers
    /// can trust [`SessionResult::disclosed`] as soon as `result` returns `Ok`
    /// for a disclosure/signing session.
    pub async fn result(&self, token: &SessionToken) -> Result<SessionResult, Error> {
        let result = self
            .client
            .get(self.url.join(&format!("session/{}/result", token.0))?)
            .send()
            .await?
            .error_for_status()?
            .json::<SessionResult>()
            .await?;
        validate_result(result)
    }

    /// Check whether the irma server is healthy and ready to serve sessions.
    ///
    /// Issues `GET {base}/health` (added in irmago v0.15.0) and returns
    /// `Ok(())` on a 2xx response, or [`Error::NetworkError`] otherwise.
    /// Useful as a liveness/readiness check before starting a session.
    pub async fn health(&self) -> Result<(), Error> {
        self.client
            .get(self.url.join("health")?)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

/// Map a fetched [`SessionResult`] onto a `Result`, enforcing that disclosure
/// and signing sessions only succeed when their proof verified.
///
/// A `Done` disclosure/signing session is only accepted when
/// `proof_status == Some(ProofStatus::Valid)`; any other proof status (or a
/// missing one) is rejected with [`Error::ProofNotValid`]. Issuance sessions
/// produce no disclosure proof and are accepted on completion alone.
fn validate_result(result: SessionResult) -> Result<SessionResult, Error> {
    match result.status {
        SessionStatus::Done => match result.sessiontype {
            SessionType::Disclosing | SessionType::Signing => match result.proof_status {
                Some(ProofStatus::Valid) => Ok(result),
                other => Err(Error::ProofNotValid(other)),
            },
            SessionType::Issuing => Ok(result),
        },
        SessionStatus::Cancelled => Err(Error::SessionCancelled),
        SessionStatus::Timeout => Err(Error::SessionTimedOut),
        status => Err(Error::SessionNotFinished(status)),
    }
}

/// Builder for IRMA clients
pub struct IrmaClientBuilder {
    url: Url,
    authmethod: AuthMethod,
}

impl IrmaClientBuilder {
    /// Create a new Client builder, with the given URL for the irma server
    pub fn new(url: &str) -> Result<IrmaClientBuilder, Error> {
        Ok(IrmaClientBuilder {
            url: Url::parse(url)?,
            authmethod: AuthMethod::None,
        })
    }

    /// Enable token authentication for the client, using the provided token
    pub fn token_authentication(mut self, token: String) -> IrmaClientBuilder {
        self.authmethod = AuthMethod::Token(TokenSecret { token });
        self
    }

    /// Construct the actual IrmaClient
    pub fn build(self) -> IrmaClient {
        IrmaClient {
            url: self.url,
            client: Client::new(),
            authmethod: self.authmethod,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::validate_result;
    use crate::{
        Error, FrontendRequest, ProofStatus, SessionData, SessionResult, SessionStatus,
        SessionToken, SessionType,
    };

    fn result_with(
        sessiontype: SessionType,
        status: SessionStatus,
        proof_status: Option<ProofStatus>,
    ) -> SessionResult {
        SessionResult {
            token: SessionToken("token".into()),
            sessiontype,
            status,
            proof_status,
            disclosed: vec![],
            signature: None,
        }
    }

    #[test]
    fn valid_disclosure_proof_is_accepted() {
        for sessiontype in [SessionType::Disclosing, SessionType::Signing] {
            let result = result_with(sessiontype, SessionStatus::Done, Some(ProofStatus::Valid));
            assert_eq!(validate_result(result.clone()).unwrap(), result);
        }
    }

    #[test]
    fn non_valid_proof_on_finished_disclosure_is_rejected() {
        // A DONE session whose proof did not verify must not surface as Ok.
        for proof_status in [
            ProofStatus::Invalid,
            ProofStatus::InvalidTimestamp,
            ProofStatus::UnmatchedRequest,
            ProofStatus::MissingAttributes,
            ProofStatus::Expired,
        ] {
            for sessiontype in [SessionType::Disclosing, SessionType::Signing] {
                let result =
                    result_with(sessiontype, SessionStatus::Done, Some(proof_status.clone()));
                assert!(matches!(
                    validate_result(result),
                    Err(Error::ProofNotValid(Some(ref s))) if *s == proof_status
                ));
            }
        }
    }

    #[test]
    fn missing_proof_status_on_finished_disclosure_is_rejected() {
        let result = result_with(SessionType::Disclosing, SessionStatus::Done, None);
        assert!(matches!(
            validate_result(result),
            Err(Error::ProofNotValid(None))
        ));
    }

    #[test]
    fn issuance_is_gated_on_completion_only() {
        // Issuance carries no disclosure proof, so a DONE status is enough,
        // regardless of proof_status.
        for proof_status in [None, Some(ProofStatus::Valid)] {
            let result = result_with(SessionType::Issuing, SessionStatus::Done, proof_status);
            assert_eq!(validate_result(result.clone()).unwrap(), result);
        }
    }

    #[test]
    fn unfinished_and_aborted_sessions_map_to_their_errors() {
        assert!(matches!(
            validate_result(result_with(
                SessionType::Disclosing,
                SessionStatus::Cancelled,
                None
            )),
            Err(Error::SessionCancelled)
        ));
        assert!(matches!(
            validate_result(result_with(
                SessionType::Disclosing,
                SessionStatus::Timeout,
                None
            )),
            Err(Error::SessionTimedOut)
        ));
        assert!(matches!(
            validate_result(result_with(
                SessionType::Disclosing,
                SessionStatus::Connected,
                None
            )),
            Err(Error::SessionNotFinished(SessionStatus::Connected))
        ));
    }

    #[test]
    fn test_decode_session_data_with_frontend_request() {
        let data = serde_json::from_str::<SessionData>(
            r#"
            {
                "token": "KzxuWKwL5KGLKr4uerws",
                "sessionPtr": {
                    "u": "https://example.com/irma/session/abc",
                    "irmaqr": "disclosing"
                },
                "frontendRequest": {
                    "authorization": "O5Ld2vAr9pkz7ELzWqgM",
                    "minProtocolVersion": "1.0",
                    "maxProtocolVersion": "1.1"
                }
            }
            "#,
        )
        .unwrap();

        assert_eq!(
            data.frontend_request,
            Some(FrontendRequest {
                authorization: "O5Ld2vAr9pkz7ELzWqgM".into(),
                min_protocol_version: Some("1.0".into()),
                max_protocol_version: Some("1.1".into()),
            })
        );

        // Round-trips back to JSON without losing the frontend request.
        let reparsed =
            serde_json::from_str::<SessionData>(&serde_json::to_string(&data).unwrap()).unwrap();
        assert_eq!(reparsed.frontend_request, data.frontend_request);
    }

    #[test]
    fn test_decode_session_data_without_frontend_request() {
        // Servers older than irmago v0.14.0 omit the frontendRequest block.
        let data = serde_json::from_str::<SessionData>(
            r#"
            {
                "token": "KzxuWKwL5KGLKr4uerws",
                "sessionPtr": {
                    "u": "https://example.com/irma/session/abc",
                    "irmaqr": "disclosing"
                }
            }
            "#,
        )
        .unwrap();

        assert_eq!(data.frontend_request, None);

        // The field is skipped on serialization when absent.
        let json = serde_json::to_string(&data).unwrap();
        assert!(!json.contains("frontendRequest"));
    }

    #[test]
    fn test_decode_frontend_request_without_protocol_versions() {
        // Only authorization is guaranteed to be useful; versions are optional.
        let request = serde_json::from_str::<FrontendRequest>(
            r#"{ "authorization": "O5Ld2vAr9pkz7ELzWqgM" }"#,
        )
        .unwrap();

        assert_eq!(
            request,
            FrontendRequest {
                authorization: "O5Ld2vAr9pkz7ELzWqgM".into(),
                min_protocol_version: None,
                max_protocol_version: None,
            }
        );
    }
}
