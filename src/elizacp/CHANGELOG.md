# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [11.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v11.0.0-alpha.1...elizacp-v11.0.0-alpha.2) - 2026-01-19

### Fixed

- *(sacp)* revert accidental JrMessageHandler and JrRequestCx renames

### Other

- upgrade to 11.0-alpha.1
- release
- go back from `connect_from` to `builder`
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename context types for clarity
- *(sacp)* rename Jr* traits to JsonRpc* for clarity

## [12.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v11.0.0...elizacp-v12.0.0) - 2026-01-19

### Fixed

- *(sacp)* revert accidental JrMessageHandler and JrRequestCx renames

### Other

- go back from `connect_from` to `builder`
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename context types for clarity
- *(sacp)* rename Jr* traits to JsonRpc* for clarity

## [11.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v10.0.0...elizacp-v11.0.0) - 2025-12-31

### Added

- *(elizacp)* implement Eliza algorithm based on the original style

## [10.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v10.0.0-alpha.3...elizacp-v10.0.0-alpha.4) - 2025-12-30

### Added

- *(deps)* [**breaking**] upgrade agent-client-protocol-schema to 0.10.5

## [10.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v10.0.0-alpha.2...elizacp-v10.0.0-alpha.3) - 2025-12-29

### Other

- updated the following local packages: sacp, sacp-tokio

## [10.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v10.0.0-alpha.1...elizacp-v10.0.0-alpha.2) - 2025-12-29

### Other

- updated the following local packages: sacp, sacp-tokio

## [10.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v9.0.0...elizacp-v10.0.0-alpha.1) - 2025-12-28

### Other

- [**breaking**] split peer.rs into separate peer and link modules
- [**breaking**] update module and documentation references from role to peer
- [**breaking**] give component a link
- update UntypedRole to UntypedRole in doc examples
- *(sacp)* rename with_client to run_until
- update references for renamed methods

## [9.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v8.0.0...elizacp-v9.0.0) - 2025-12-19

### Added

- *(sacp)* [**breaking**] require Send for JrMessageHandler with boxing witness macros
- *(sacp)* [**breaking**] add scoped lifetime support for MCP servers

### Other

- Merge pull request #88 from nikomatsakis/main

## [8.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v7.0.0...elizacp-v8.0.0) - 2025-12-17

### Other

- updated the following local packages: sacp, sacp-tokio

## [6.0.1](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v6.0.0...elizacp-v6.0.1) - 2025-12-17

### Other

- updated the following local packages: sacp, sacp-tokio

## [4.0.1](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v4.0.0...elizacp-v4.0.1) - 2025-12-15

### Other

- updated the following local packages: sacp, sacp-tokio

## [4.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v3.0.1...elizacp-v4.0.0) - 2025-12-12

### Added

- [**breaking**] introduce role-based connection API

## [3.0.1](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v3.0.0...elizacp-v3.0.1) - 2025-11-25

### Fixed

- *(elizacp)* spawn prompt processing to avoid blocking event loop

### Other

- Merge pull request #60 from nikomatsakis/main

## [3.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v2.0.1...elizacp-v3.0.0) - 2025-11-25

### Added

- *(elizacp)* add HTTP MCP server support and update tests to use HTTP bridge

## [2.0.1](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v2.0.0...elizacp-v2.0.1) - 2025-11-23

### Fixed

- *(elizacp)* support hyphens in MCP server and tool names

## [2.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.2.0...elizacp-v2.0.0) - 2025-11-23

### Other

- *(elizacp)* export ElizaAgent as public Component

## [1.2.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.1.0...elizacp-v1.2.0) - 2025-11-22

### Added

- *(elizacp)* add MCP tools/list support and refactor client handling

### Fixed

- update Stdio usage to Stdio::new() after API change

## [1.1.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0...elizacp-v1.1.0) - 2025-11-17

### Added

- *(sacp-test)* add mcp-echo-server binary for testing
- *(elizacp)* add MCP tool invocation support for testing

### Fixed

- *(elizacp)* update tool name regex to match MCP spec
- *(elizacp)* allow hyphens in MCP server and tool names

### Other

- Revert "fix(elizacp): allow hyphens in MCP server and tool names"
- *(test)* use structured SessionNotification instead of debug strings
- *(elizacp)* extract text content only to avoid UUID flakiness
- *(elizacp)* convert to expect_test for clearer output verification
- *(elizacp)* add integration test for MCP tool invocation

## [1.0.0](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.8...elizacp-v1.0.0) - 2025-11-13

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.8](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.7...elizacp-v1.0.0-alpha.8) - 2025-11-12

### Other

- Merge pull request #30 from nikomatsakis/main
- *(sacp)* remove Deref impl from JrRequestCx

## [1.0.0-alpha.7](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.6...elizacp-v1.0.0-alpha.7) - 2025-11-12

### Other

- Merge pull request #28 from nikomatsakis/main

## [1.0.0-alpha.6](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.5...elizacp-v1.0.0-alpha.6) - 2025-11-11

### Other

- updated the following local packages: sacp, sacp-tokio

## [1.0.0-alpha.5](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.4...elizacp-v1.0.0-alpha.5) - 2025-11-11

### Other

- convert Stdio to unit struct for easier reference

## [1.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.3...elizacp-v1.0.0-alpha.4) - 2025-11-11

### Other

- cleanup and simplify some of the logic to avoid "indirection" through

## [1.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.2...elizacp-v1.0.0-alpha.3) - 2025-11-09

### Other

- updated the following local packages: sacp

## [1.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/elizacp-v1.0.0-alpha.1...elizacp-v1.0.0-alpha.2) - 2025-11-08

### Other

- Merge pull request #16 from nikomatsakis/main
- wip wip wip
- [**breaking**] remove Unpin bounds and simplify transport API

## [1.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/releases/tag/elizacp-v1.0.0-alpha.1) - 2025-11-05

### Added

- *(elizacp)* add Eliza chatbot as ACP agent for testing

### Fixed

- fix github url
- *(elizacp)* exclude punctuation from pattern captures

### Other

- update all versions from 1.0.0-alpha to 1.0.0-alpha.1
- release v1.0.0-alpha
- *(conductor)* add mock component tests for nested conductors
- bump all packages to version 1.0.0-alpha
- *(sacp)* [**breaking**] reorganize modules with flat schema namespace
- release
- *(elizacp)* add module-level documentation to main.rs
- upgrade to latest version of depdencies
- *(elizacp)* release v0.1.0
- *(elizacp)* prepare for crates.io publication
- cleanup the source to make it prettier as an example
- *(elizacp)* use expect_test for exact output verification
- *(elizacp)* use seeded RNG for deterministic responses

## [1.0.0-alpha](https://github.com/symposium-dev/symposium-acp/releases/tag/elizacp-v1.0.0-alpha) - 2025-11-05

### Added

- *(elizacp)* add Eliza chatbot as ACP agent for testing

### Fixed

- fix github url
- *(elizacp)* exclude punctuation from pattern captures

### Other

- *(conductor)* add mock component tests for nested conductors
- bump all packages to version 1.0.0-alpha
- *(sacp)* [**breaking**] reorganize modules with flat schema namespace
- release
- *(elizacp)* add module-level documentation to main.rs
- upgrade to latest version of depdencies
- *(elizacp)* release v0.1.0
- *(elizacp)* prepare for crates.io publication
- cleanup the source to make it prettier as an example
- *(elizacp)* use expect_test for exact output verification
- *(elizacp)* use seeded RNG for deterministic responses

## [0.1.0](https://github.com/symposium-dev/symposium-acp/releases/tag/elizacp-v0.1.0) - 2025-10-31

### Added

- *(elizacp)* add Eliza chatbot as ACP agent for testing

### Fixed

- *(elizacp)* exclude punctuation from pattern captures

### Other

- *(elizacp)* prepare for crates.io publication
- cleanup the source to make it prettier as an example
- *(elizacp)* use expect_test for exact output verification
- *(elizacp)* use seeded RNG for deterministic responses
