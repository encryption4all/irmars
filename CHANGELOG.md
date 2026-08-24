# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.3](https://github.com/encryption4all/irmars/compare/v0.2.2...v0.2.3) - 2026-08-24

### Added

- support chained sessions (nextSession) on ExtendedIrmaRequest
- add host option to override session QR host
- expose frontendRequest block on SessionData
- add IrmaClient::health() wrapper for the /health endpoint

### Fixed

- enforce disclosure proof in combined issuance+disclosure results
- enforce proof verification in IrmaClient::result()

### Other

- correct the upstream dormancy date and the size-gate citation
- state the size gate's rationale once
- cut CLAUDE.md to orientation and gate its size
- drop leading horizontal rule before first heading in CLAUDE.md
- add agent & contributor notes (migrated from dobby memory)
- Merge pull request #23 from encryption4all/fix/enforce-proof-status-in-result
- Merge remote-tracking branch 'origin/main' into fix/enforce-proof-status-in-result
- bump reqwest to 0.13

### Security

- validate the format of session tokens before using them in server requests

## [0.2.2](https://github.com/encryption4all/irmars/compare/v0.2.1...v0.2.2) - 2026-05-18

### Other

- bump irmago server to v0.19.2
- point scheme download at schemes.yivi.app
- modernize workflows and bump actions to latest
- enforce Conventional Commits in PR titles
- Bump dependencies to current major versions
