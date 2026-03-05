# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [11.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/releases/tag/yopo-v11.0.0-alpha.1) - 2026-01-19

### Other

- upgrade to 11.0-alpha.1
- release
- go back from `connect_from` to `builder`
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename MessageCx to Dispatch for clearer semantics
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename context types for clarity

## [11.0.0](https://github.com/symposium-dev/symposium-acp/compare/yopo-v10.0.1...yopo-v11.0.0) - 2026-01-19

### Other

- go back from `connect_from` to `builder`
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename MessageCx to Dispatch for clearer semantics
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename context types for clarity

## [10.0.1](https://github.com/symposium-dev/symposium-acp/compare/yopo-v10.0.0...yopo-v10.0.1) - 2025-12-31

### Other

- updated the following local packages: sacp, sacp-tokio

## [10.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/yopo-v10.0.0-alpha.3...yopo-v10.0.0-alpha.4) - 2025-12-30

### Added

- *(deps)* [**breaking**] upgrade agent-client-protocol-schema to 0.10.5

## [10.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/yopo-v10.0.0-alpha.2...yopo-v10.0.0-alpha.3) - 2025-12-29

### Other

- updated the following local packages: sacp, sacp-tokio

## [10.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/yopo-v10.0.0-alpha.1...yopo-v10.0.0-alpha.2) - 2025-12-29

### Other

- updated the following local packages: sacp, sacp-tokio

## [10.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/compare/yopo-v9.0.0...yopo-v10.0.0-alpha.1) - 2025-12-28

### Other

- [**breaking**] split peer.rs into separate peer and link modules
- [**breaking**] update module and documentation references from role to peer
- [**breaking**] give component a link
- *(sacp)* add block_task() builder pattern for SessionBuilder
- *(sacp)* rename spawn_session to start_session
- *(sacp)* rename with_client to run_until
- *(sacp)* add lifetime-safe session proxying API

## [9.0.0](https://github.com/symposium-dev/symposium-acp/compare/yopo-v8.0.0...yopo-v9.0.0) - 2025-12-19

### Added

- *(sacp)* [**breaking**] require Send for JrMessageHandler with boxing witness macros
- *(sacp)* [**breaking**] use AsyncFnMut for tool closures with macro workaround

## [8.0.0](https://github.com/symposium-dev/symposium-acp/compare/yopo-v7.0.0...yopo-v8.0.0) - 2025-12-17

### Other

- updated the following local packages: sacp, sacp-tokio

## [6.0.1](https://github.com/symposium-dev/symposium-acp/compare/yopo-v6.0.0...yopo-v6.0.1) - 2025-12-17

### Other

- updated the following local packages: sacp, sacp-tokio

## [2.0.1](https://github.com/symposium-dev/symposium-acp/compare/yopo-v2.0.0...yopo-v2.0.1) - 2025-12-15

### Fixed

- *(yopo)* remove unused MatchMessageFrom import

### Other

- *(sacp)* add guidance on when to use MatchMessage vs MatchMessageFrom
- *(sacp)* rename MatchMessage to MatchMessageFrom
- new session redirection support

## [2.0.0](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.2.1...yopo-v2.0.0) - 2025-12-12

### Added

- [**breaking**] introduce role-based connection API

## [1.2.1](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.2.0...yopo-v1.2.1) - 2025-11-25

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.2.0](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.1.0...yopo-v1.2.0) - 2025-11-22

### Added

- *(yopo)* add tracing support with configurable log levels
- *(sacp)* add IntoHandled support to on_receive_message

## [1.1.0](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0...yopo-v1.1.0) - 2025-11-18

### Added

- *(yopo)* add library API with callback support for testing SACP agents

### Fixed

- *(yopo)* correct doctest examples to use valid AcpAgent API

### Other

- *(yopo)* simplify callback implementation using AsyncFnMut
- *(yopo)* accept impl ToString for prompt parameter
- *(yopo)* return sacp::Error instead of Box<dyn Error>

## [1.0.0](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.8...yopo-v1.0.0) - 2025-11-13

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.8](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.7...yopo-v1.0.0-alpha.8) - 2025-11-12

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.7](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.6...yopo-v1.0.0-alpha.7) - 2025-11-12

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.6](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.5...yopo-v1.0.0-alpha.6) - 2025-11-11

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.5](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.4...yopo-v1.0.0-alpha.5) - 2025-11-11

### Other

- updated the following local packages: sacp-tokio

## [1.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.3...yopo-v1.0.0-alpha.4) - 2025-11-11

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.2...yopo-v1.0.0-alpha.3) - 2025-11-09

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/yopo-v1.0.0-alpha.1...yopo-v1.0.0-alpha.2) - 2025-11-08

### Other

- Merge pull request #16 from nikomatsakis/main
- wip wip wip
- [**breaking**] remove Unpin bounds and simplify transport API

## [1.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/releases/tag/yopo-v1.0.0-alpha.1) - 2025-11-05

### Added

- *(yopo)* add yopo binary crate and simplify example

### Other

- Merge pull request #14 from nikomatsakis/main
- update all versions from 1.0.0-alpha to 1.0.0-alpha.1

## [1.0.0-alpha](https://github.com/symposium-dev/symposium-acp/releases/tag/yopo-v1.0.0-alpha) - 2025-11-05

### Added

- *(yopo)* add yopo binary crate and simplify example
