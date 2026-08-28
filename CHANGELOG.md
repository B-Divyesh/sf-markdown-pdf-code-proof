# Changelog

## 0.1.0 — unreleased repair

- Sandbox renderer subprocesses with Linux Landlock filesystem allowlists and
  seccomp network denial; refuse renderer execution without those controls.
- Make service-worker precaching production-safe by excluding Static Web Apps'
  deployment-only configuration file and claiming updated clients promptly.

All notable changes follow Keep a Changelog. This project uses Semantic Versioning.

## [Unreleased]

### Fixed

- Verify every Markdown fragment against its matching PDF named destination and
  resolved page, rather than treating a count of link annotations as proof.

## [0.1.0] - 2026-08-27

### Added

- Initial CLI, PDF checks, proof sheet, and static documentation site.
