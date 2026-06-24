**This library is still under development, and interfaces may or may not be stable. Use at your own peril**

This crate provides a rust wrapper around the irma server rest API. It can be used to interface with an irma server for disclosure, signatures and issuance. Although its concept is inspired by the original irma crate (https://github.com/Wassasin/irmars) it was constructed from the ground up to provide a consistent interface with support for all of the major session types.

For documentation on the IRMA ecosystem, see [the IRMA documentation](https://irma.app/docs)

## Chained sessions

`ExtendedIrmaRequest` supports [chained (next) sessions](https://irma.app/docs/chained-sessions): set a follow-up requestor URL with `.next_session(url)` (serialized as `nextSession`), and the server starts the linked session immediately after the current one succeeds.

When using chained sessions in production, run an irmago server of **at least v0.19.0**. That release ships [GHSA-pv8v-c99h-c5q4](https://github.com/privacybydesign/irmago/security/advisories/GHSA-pv8v-c99h-c5q4), which tightens permission handling for next-session requests.
