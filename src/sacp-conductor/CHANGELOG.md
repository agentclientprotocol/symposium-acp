# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [11.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/releases/tag/sacp-conductor-v11.0.0-alpha.1) - 2026-01-19

### Added

- *(conductor)* infer response tracing direction from context
- *(sacp)* add Response variant to MessageCx

### Fixed

- *(sacp)* revert accidental JrMessageHandler and JrRequestCx renames
- *(conductor)* normalize acp_url field in trace snapshot tests
- *(conductor)* handle Response variants in message forwarding
- *(conductor)* panic on Response in forwarding functions

### Other

- upgrade to 11.0-alpha.1
- release
- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
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
- *(sacp)* rename Jr* traits to JsonRpc* for clarity
- get tracing working
- refactor tracing
- *(conductor)* simplify trace bridge API
- *(conductor)* move tracing to transport layer with bridge wrappers
- remove into_connection_builder
- add a test that shows a request going all the way back to the client
- *(sacp-conductor)* add trace snapshot for client-hosted MCP server
- *(sacp-conductor)* add trace snapshot test for agent-initiated MCP tool calls
- wip
- *(sacp)* introduce JrResponseCx for incoming response handling

## [11.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v10.0.1...sacp-conductor-v11.0.0) - 2026-01-19

### Added

- *(conductor)* infer response tracing direction from context
- *(sacp)* add Response variant to MessageCx

### Fixed

- *(sacp)* revert accidental JrMessageHandler and JrRequestCx renames
- *(conductor)* normalize acp_url field in trace snapshot tests
- *(conductor)* handle Response variants in message forwarding
- *(conductor)* panic on Response in forwarding functions

### Other

- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
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
- *(sacp)* rename Jr* traits to JsonRpc* for clarity
- get tracing working
- refactor tracing
- *(conductor)* simplify trace bridge API
- *(conductor)* move tracing to transport layer with bridge wrappers
- remove into_connection_builder
- add a test that shows a request going all the way back to the client
- *(sacp-conductor)* add trace snapshot for client-hosted MCP server
- *(sacp-conductor)* add trace snapshot test for agent-initiated MCP tool calls
- wip
- *(sacp)* introduce JrResponseCx for incoming response handling

## [10.0.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v10.0.0...sacp-conductor-v10.0.1) - 2025-12-31

### Other

- Merge pull request #109 from nikomatsakis/ci-binaries
- add binary releases for CLI tools

## [10.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v10.0.0-alpha.3...sacp-conductor-v10.0.0-alpha.4) - 2025-12-30

### Added

- *(deps)* [**breaking**] upgrade agent-client-protocol-schema to 0.10.5

## [10.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v10.0.0-alpha.2...sacp-conductor-v10.0.0-alpha.3) - 2025-12-29

### Added

- *(sacp)* add tool enable/disable filtering for MCP servers

### Other

- *(sacp-conductor)* add standalone MCP server tests
- [**breaking**] make `McpServer` implement component

## [10.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v10.0.0-alpha.1...sacp-conductor-v10.0.0-alpha.2) - 2025-12-29

### Other

- updated the following local packages: sacp, sacp-tokio

## [10.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v9.0.0...sacp-conductor-v10.0.0-alpha.1) - 2025-12-28

### Added

- *(sacp)* add tool_fn for concurrent stateless tools

### Fixed

- *(sacp)* use unstructured output for non-object MCP tool results
- *(sacp-conductor)* route proxied responses through conductor message loop
- *(sacp-conductor)* use Agent endpoint for proxy mode successor forwarding

### Other

- [**breaking**] split peer.rs into separate peer and link modules
- [**breaking**] update module and documentation references from role to peer
- [**breaking**] rename FooRole types to FooPeer
- [**breaking**] rename link endpoint types from Foo to FooRole
- [**breaking**] give component a link
- refactor!(conductor): replace ComponentList with typed instantiator traits
- [**breaking**] split Conductor into agent vs proxy mode with ConductorLink trait
- use self.conductor_tx
- [**breaking**] rename End type param to Peer, endpoint vars to peer
- update UntypedRole to UntypedRole in doc examples
- [**breaking**] rename Endpoint to Role
- *(sacp)* rename JrRole to JrLink, Role type param to Link
- [**breaking**] rename JrRole to JrLink (and the Role associated type to Link)
- *(sacp-conductor)* use explicit endpoints for ConductorToClient
- *(sacp)* add block_task() builder pattern for SessionBuilder
- *(sacp)* rename with_client to run_until
- update references for renamed methods

## [9.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v8.0.0...sacp-conductor-v9.0.0) - 2025-12-19

### Added

- *(sacp)* [**breaking**] require Send for JrMessageHandler with boxing witness macros
- *(sacp)* [**breaking**] use AsyncFnMut for tool closures with macro workaround
- *(sacp)* [**breaking**] merge pending_tasks into JrResponder and remove 'scope lifetime
- *(sacp)* [**breaking**] add scoped lifetime support for MCP servers
- *(sacp)* [**breaking**] add scoped lifetime to JrConnectionBuilder and JrConnection

### Fixed

- *(sacp-conductor)* use pre-built binaries in all integration tests
- *(sacp-conductor)* use pre-built binaries in trace_generation test

### Other

- *(sacp)* simplify return types with impl Trait
- *(sacp-conductor)* formatting and accept impl ToString for name
- Merge pull request #88 from nikomatsakis/main
- *(sacp)* [**breaking**] remove to_future_hack from tool_fn API

## [8.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v7.0.0...sacp-conductor-v8.0.0) - 2025-12-17

### Fixed

- *(sacp)* ensure NewSessionRequest flows through handler chain

### Other

- *(sacp)* add tool_fn!() macro for MCP tool registration

## [7.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v6.0.0...sacp-conductor-v7.0.0) - 2025-12-17

### Other

- update examples and tests to use new MCP server API
- update MCP server documentation for new API
- rename handler chain terminology to connection builder

## [5.0.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v5.0.0...sacp-conductor-v5.0.1) - 2025-12-15

### Other

- *(sacp)* rename MatchMessage to MatchMessageFrom
- new session redirection support

## [5.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v4.0.0...sacp-conductor-v5.0.0) - 2025-12-12

### Added

- [**breaking**] introduce role-based connection API

### Other

- release

## [4.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v3.0.0...sacp-conductor-v4.0.0) - 2025-12-12

### Added

- [**breaking**] introduce role-based connection API

## [3.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v2.1.1...sacp-conductor-v3.0.0) - 2025-11-25

### Added

- *(elizacp)* add HTTP MCP server support and update tests to use HTTP bridge
- *(conductor)* change default MCP bridge mode to HTTP

### Fixed

- *(conductor)* return 202 Accepted for MCP notifications per HTTP spec
- *(conductor)* correct typo in actor parameter name
- *(conductor)* update tests to use McpBridgeMode instead of Option<Vec<String>>

### Other

- *(conductor)* simplify HTTP MCP bridge with typed HttpMessage enum
- *(conductor)* update outdated comments in MCP bridge modules
- move HTTP and implement shared code
- move stdio into the stdio arm
- *(conductor)* introduce McpBridgeMode enum to support multiple bridge types
- adopt newer convention
- *(conductor)* extract MCP bridge actor into trait with stdio submodule
- implementing HTTP MCP bridge
- impenetrable error

## [2.1.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v2.1.0...sacp-conductor-v2.1.1) - 2025-11-23

### Other

- *(elizacp)* export ElizaAgent as public Component

## [2.1.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v2.0.1...sacp-conductor-v2.1.0) - 2025-11-22

### Added

- *(sacp-conductor)* write tracing logs to debug file when --debug is enabled
- *(sacp-conductor)* add --log option for setting log level
- *(sacp-conductor)* strip ANSI escape codes from stderr in debug logs
- *(sacp-conductor)* add debug logging with --debug and --debug-dir flags
- *(elizacp)* add MCP tools/list support and refactor client handling

### Fixed

- *(sacp-conductor)* empty conductor now responds with proxy capability
- *(sacp-conductor)* handle empty components in proxy mode

### Other

- *(sacp-conductor)* avoid creating debug logger twice
- *(sacp-conductor)* simplify logging to only use command-line args
- *(sacp-conductor)* move main logic into library

## [2.0.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v2.0.0...sacp-conductor-v2.0.1) - 2025-11-18

### Other

- *(yopo)* accept impl ToString for prompt parameter
- replace yolo_prompt with direct yopo::prompt calls
- upgrade to sacp-proxy 2.0.0 and migrate tests to McpServer API

## [2.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.1...sacp-conductor-v2.0.0) - 2025-11-17

### Added

- *(sacp-proxy)* add meta fields to all request/notification types
- *(conductor)* add session-aware MCP bridge with connection correlation

### Fixed

- *(elizacp)* update tool name regex to match MCP spec
- *(test)* suppress dead code warnings from mcp_integration module

### Other

- Merge pull request #47 from nikomatsakis/main
- *(test)* remove broken test_mcp_bridge_with_session_id
- *(test)* use block_task() instead of custom recv helper
- *(sacp-conductor)* verify MCP tools receive correct session_id
- add test helpers and elizacp component wrapper

## [1.0.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0...sacp-conductor-v1.0.1) - 2025-11-15

### Other

- updated the following local packages: sacp-proxy

## [1.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.9...sacp-conductor-v1.0.0) - 2025-11-13

### Other

- *(sacp-proxy)* [**breaking**] move rmcp integration to separate sacp-rmcp crate

## [1.0.0-alpha.9](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.8...sacp-conductor-v1.0.0-alpha.9) - 2025-11-12

### Other

- Merge pull request #30 from nikomatsakis/main
- *(sacp)* remove Deref impl from JrRequestCx
- *(sacp)* add Component::serve() and simplify channel API

## [1.0.0-alpha.8](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.7...sacp-conductor-v1.0.0-alpha.8) - 2025-11-12

### Other

- Merge pull request #28 from nikomatsakis/main

## [1.0.0-alpha.7](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.6...sacp-conductor-v1.0.0-alpha.7) - 2025-11-11

### Other

- Merge pull request #26 from nikomatsakis/main
- [**breaking**] make Component trait ergonomic with async fn and introduce DynComponent
- [**breaking**] make Component the primary trait with Transport as blanket impl

## [1.0.0-alpha.6](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.5...sacp-conductor-v1.0.0-alpha.6) - 2025-11-11

### Added

- *(sacp-conductor)* implement Component trait for Conductor
- *(sacp-conductor)* add SYMPOSIUM_LOG environment variable for file logging

### Other

- Merge pull request #24 from nikomatsakis/main

## [1.0.0-alpha.5](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.4...sacp-conductor-v1.0.0-alpha.5) - 2025-11-11

### Other

- convert Stdio to unit struct for easier reference

## [1.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.3...sacp-conductor-v1.0.0-alpha.4) - 2025-11-11

### Other

- cleanup and simplify some of the logic to avoid "indirection" through
- remove ComponentProvider trait
- unify Transport and Component traits with BoxFuture-returning signatures
- create selective jsonrpcmsg re-export module
- replace jsonrpcmsg::Message with sacp::JsonRpcMessage throughout codebase
- move Component trait to sacp-proxy crate
- *(sacp)* improve IntoJrTransport and Component trait impls
- *(sacp-conductor)* make Component trait mirror IntoJrTransport interface
- *(sacp-conductor)* introduce Component trait for better semantics

## [1.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.2...sacp-conductor-v1.0.0-alpha.3) - 2025-11-09

### Added

- *(sacp-conductor)* implement lazy component initialization
- *(sacp-conductor)* add ComponentList trait for lazy component instantiation

### Fixed

- *(sacp-conductor)* prevent response messages from overtaking notifications

### Other

- Merge pull request #18 from nikomatsakis/main
- *(sacp-conductor)* document lazy initialization and ComponentList trait
- *(sacp-conductor)* use if-let and assert in lazy initialization
- *(sacp-conductor)* route all message forwarding through central queue
- *(sacp-conductor)* decouple message handler from component count

## [1.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v1.0.0-alpha.1...sacp-conductor-v1.0.0-alpha.2) - 2025-11-08

### Added

- *(sacp)* add convenience methods for common connection patterns

### Other

- fix doctests for API refactoring
- wip wip wip
- wipwipwip
- [**breaking**] remove Unpin bounds and simplify transport API

## [0.2.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v0.1.1...sacp-conductor-v0.2.0) - 2025-11-04

### Fixed

- fix github url

### Other

- *(sacp-conductor)* add comprehensive crate-level documentation
- *(sacp)* [**breaking**] rename json_rpc_cx to connection_cx
- rename JsonRpcRequest to JrRequest
- add READMEs for sacp-tokio, sacp-proxy, and sacp-conductor

## [0.1.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-conductor-v0.1.0...sacp-conductor-v0.1.1) - 2025-10-30

### Fixed

- replace crate::Error with sacp::Error in dependent crates

### Other

- remove more uses of `agent_client_protocl_schema`
- replace acp:: with crate::/sacp:: throughout codebase
- rename JsonRpc* types to Jr* across all crates
- *(deps)* switch from agent-client-protocol to agent-client-protocol-schema
- prepare release-plz
