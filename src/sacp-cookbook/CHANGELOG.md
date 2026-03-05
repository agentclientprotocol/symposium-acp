# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [11.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/releases/tag/sacp-cookbook-v11.0.0-alpha.1) - 2026-01-19

### Other

- upgrade to 11.0-alpha.1
- release
- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
- *(sacp-cookbook)* update cookbook examples for v11 Role-based API
- *(sacp)* [**breaking**] rename HandleMessageFrom to HandleDispatchFrom
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename MessageCx to Dispatch for clearer semantics
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename JrMessageHandler to HandleMessageFrom
- *(sacp)* rename JrConnectionBuilder to ConnectFrom
- *(sacp)* simplify spawn_connection API
- *(sacp)* rename context types for clarity
- *(sacp)* rename JrResponder ecosystem to Run

## [11.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-cookbook-v10.0.1...sacp-cookbook-v11.0.0) - 2026-01-19

### Other

- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
- *(sacp-cookbook)* update cookbook examples for v11 Role-based API
- *(sacp)* [**breaking**] rename HandleMessageFrom to HandleDispatchFrom
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename MessageCx to Dispatch for clearer semantics
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename JrMessageHandler to HandleMessageFrom
- *(sacp)* rename JrConnectionBuilder to ConnectFrom
- *(sacp)* simplify spawn_connection API
- *(sacp)* rename context types for clarity
- *(sacp)* rename JrResponder ecosystem to Run

## [10.0.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-cookbook-v10.0.0...sacp-cookbook-v10.0.1) - 2025-12-31

### Other

- updated the following local packages: sacp, sacp-tokio, sacp-conductor, sacp-rmcp

## [10.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/sacp-cookbook-v10.0.0-alpha.3...sacp-cookbook-v10.0.0-alpha.4) - 2025-12-30

### Added

- *(deps)* [**breaking**] upgrade agent-client-protocol-schema to 0.10.5

## [10.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/sacp-cookbook-v10.0.0-alpha.2...sacp-cookbook-v10.0.0-alpha.3) - 2025-12-29

### Added

- *(sacp)* add tool enable/disable filtering for MCP servers
