# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [11.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/releases/tag/sacp-rmcp-v11.0.0-alpha.1) - 2026-01-19

### Other

- upgrade to 11.0-alpha.1
- release
- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename JrResponder ecosystem to Run

## [11.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v10.0.1...sacp-rmcp-v11.0.0) - 2026-01-19

### Other

- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename JrResponder ecosystem to Run

## [10.0.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v10.0.0...sacp-rmcp-v10.0.1) - 2025-12-31

### Other

- updated the following local packages: sacp

## [10.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v10.0.0-alpha.3...sacp-rmcp-v10.0.0-alpha.4) - 2025-12-30

### Other

- updated the following local packages: sacp

## [10.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v10.0.0-alpha.2...sacp-rmcp-v10.0.0-alpha.3) - 2025-12-29

### Other

- updated the following local packages: sacp

## [10.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v10.0.0-alpha.1...sacp-rmcp-v10.0.0-alpha.2) - 2025-12-29

### Other

- updated the following local packages: sacp

## [10.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v9.0.0...sacp-rmcp-v10.0.0-alpha.1) - 2025-12-28

### Other

- [**breaking**] rename FooRole types to FooPeer
- [**breaking**] rename link endpoint types from Foo to FooRole
- [**breaking**] give component a link
- [**breaking**] rename Endpoint to Role
- *(sacp)* rename JrRole to JrLink, Role type param to Link
- [**breaking**] rename JrRole to JrLink (and the Role associated type to Link)

## [9.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v8.0.0...sacp-rmcp-v9.0.0) - 2025-12-19

### Added

- *(sacp)* [**breaking**] add scoped lifetime support for MCP servers

### Other

- Merge pull request #88 from nikomatsakis/main
- *(sacp)* [**breaking**] remove to_future_hack from tool_fn API

## [8.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v7.0.0...sacp-rmcp-v8.0.0) - 2025-12-17

### Other

- updated the following local packages: sacp

## [7.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v6.0.0...sacp-rmcp-v7.0.0) - 2025-12-17

### Other

- update examples and tests to use new MCP server API
- update MCP server documentation for new API

## [0.10.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v0.9.0...sacp-rmcp-v0.10.0) - 2025-12-15

### Other

- new session redirection support

## [0.9.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v0.8.4...sacp-rmcp-v0.9.0) - 2025-12-12

### Added

- [**breaking**] introduce role-based connection API

## [0.8.4](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v0.8.3...sacp-rmcp-v0.8.4) - 2025-11-25

### Other

- updated the following local packages: sacp, sacp-proxy

## [0.8.3](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v0.8.2...sacp-rmcp-v0.8.3) - 2025-11-22

### Other

- updated the following local packages: sacp, sacp-proxy

## [0.8.2](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v0.8.1...sacp-rmcp-v0.8.2) - 2025-11-17

### Other

- updated the following local packages: sacp-proxy

## [0.8.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v0.8.0...sacp-rmcp-v0.8.1) - 2025-11-15

### Other

- updated the following local packages: sacp-proxy

## [0.8.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-rmcp-v0.1.0...sacp-rmcp-v0.8.0) - 2025-11-14

### Other

- release

## [0.1.0](https://github.com/symposium-dev/symposium-acp/releases/tag/sacp-rmcp-v0.1.0) - 2025-11-13

### Other

- *(sacp-proxy)* [**breaking**] move rmcp integration to separate sacp-rmcp crate
