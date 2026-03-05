# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [11.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/releases/tag/sacp-v11.0.0-alpha.1) - 2026-01-19

### Added

- *(sacp)* introduce Role trait system
- *(sacp)* expose outgoing request id on JrResponse
- *(sacp)* parse responses in into_typed_message_cx
- *(sacp)* add matches_method to JrMessage, change parse_message to return Result
- *(sacp)* add if_response_to and if_ok_response_to to MatchMessageFrom
- *(sacp)* add if_response_to and if_ok_response_to to MatchMessage
- *(sacp)* route responses through handler chain
- *(sacp)* store method name with pending reply subscriptions
- *(sacp)* add Response variant to MessageCx

### Fixed

- *(sacp)* revert accidental JrMessageHandler and JrRequestCx renames
- *(conductor)* handle Response variants in message forwarding

### Other

- upgrade to 11.0-alpha.1
- release
- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
- *(sacp)* [**breaking**] rename HandleMessageFrom to HandleDispatchFrom
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename MessageCx to Dispatch for clearer semantics
- *(sacp)* update doctests for new Role-based API
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename JrMessageHandler to HandleMessageFrom
- *(sacp)* rename JrConnectionBuilder to ConnectFrom
- *(sacp)* simplify spawn_connection API
- *(sacp)* rename context types for clarity
- *(sacp)* rename JrResponder ecosystem to Run
- *(sacp)* rename Jr* traits to JsonRpc* for clarity
- wip
- wip
- *(sacp)* use handle_incoming_message for response peer filtering
- *(sacp)* introduce JrResponseCx for incoming response handling
- *(sacp)* unify JrRequestCx send logic into send_fn

## [11.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v10.1.0...sacp-v11.0.0) - 2026-01-19

### Added

- *(sacp)* introduce Role trait system
- *(sacp)* expose outgoing request id on JrResponse
- *(sacp)* parse responses in into_typed_message_cx
- *(sacp)* add matches_method to JrMessage, change parse_message to return Result
- *(sacp)* add if_response_to and if_ok_response_to to MatchMessageFrom
- *(sacp)* add if_response_to and if_ok_response_to to MatchMessage
- *(sacp)* route responses through handler chain
- *(sacp)* store method name with pending reply subscriptions
- *(sacp)* add Response variant to MessageCx

### Fixed

- *(sacp)* revert accidental JrMessageHandler and JrRequestCx renames
- *(conductor)* handle Response variants in message forwarding

### Other

- go back from `connect_from` to `builder`
- fix unresolved rustdoc link warnings for v11 API
- *(sacp)* [**breaking**] rename HandleMessageFrom to HandleDispatchFrom
- *(sacp)* [**breaking**] rename *_cx variables to descriptive names
- *(sacp)* [**breaking**] rename MessageCx to Dispatch for clearer semantics
- *(sacp)* update doctests for new Role-based API
- *(sacp)* [**breaking**] rename Serve to ConnectTo for clearer semantics
- *(sacp)* [**breaking**] replace JrLink/JrPeer with unified Role-based API
- *(sacp)* rename JrMessageHandler to HandleMessageFrom
- *(sacp)* rename JrConnectionBuilder to ConnectFrom
- *(sacp)* simplify spawn_connection API
- *(sacp)* rename context types for clarity
- *(sacp)* rename JrResponder ecosystem to Run
- *(sacp)* rename Jr* traits to JsonRpc* for clarity
- wip
- wip
- *(sacp)* use handle_incoming_message for response peer filtering
- *(sacp)* introduce JrResponseCx for incoming response handling
- *(sacp)* unify JrRequestCx send logic into send_fn

## [10.1.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v10.0.0...sacp-v10.1.0) - 2025-12-31

### Added

- *(elizacp)* implement Eliza algorithm based on the original style

## [10.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/sacp-v10.0.0-alpha.3...sacp-v10.0.0-alpha.4) - 2025-12-30

### Added

- *(deps)* [**breaking**] upgrade agent-client-protocol-schema to 0.10.5

## [10.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/sacp-v10.0.0-alpha.2...sacp-v10.0.0-alpha.3) - 2025-12-29

### Added

- *(sacp)* implement Component for JrConnectionBuilder
- *(sacp)* add tool enable/disable filtering for MCP servers
- *(sacp-cookbook)* create dedicated cookbook crate with comprehensive examples

### Other

- [**breaking**] make `McpServer` implement component

## [10.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/sacp-v10.0.0-alpha.1...sacp-v10.0.0-alpha.2) - 2025-12-29

### Added

- *(sacp)* add ack mechanism for response dispatch ordering

### Other

- *(sacp)* un-ignore doc examples and make them compile
- *(sacp)* fix all rustdoc link warnings
- *(sacp)* reorganize lib.rs and create concepts module
- *(sacp)* document ordering guarantees for on_* methods
- *(sacp)* merge reply actor into incoming actor
- *(sacp)* add ordering module and improve proxy session docs
- Merge pull request #93 from symposium-dev/release-plz-2025-12-21T18-15-53Z

## [10.0.0-alpha.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-v9.0.0...sacp-v10.0.0-alpha.1) - 2025-12-28

### Added

- *(sacp)* add LocalRole associated type to JrLink
- *(sacp)* add on_proxy_session_start and return SessionId from start_session_proxy
- *(sacp)* add on_session_start for spawned sessions
- *(sacp)* add proxy_session for per-session MCP server injection
- *(sacp)* add build_session_from for proxying session requests
- *(sacp)* make tool_fn run concurrently
- *(sacp)* add tool_fn for concurrent stateless tools

### Fixed

- *(sacp)* use unstructured output for non-object MCP tool results
- *(sacp)* resolve race condition in proxy_remaining_messages

### Other

- [**breaking**] add ProxyPeer and improve link documentation
- [**breaking**] split peer.rs into separate peer and link modules
- [**breaking**] update module and documentation references from role to peer
- [**breaking**] rename FooRole types to FooPeer
- [**breaking**] replace RemotePeer with HasDefaultPeer::DefaultPeer
- [**breaking**] remove LocalRole from JrLink trait
- [**breaking**] rename link endpoint types from Foo to FooRole
- [**breaking**] give component a link
- [**breaking**] split Conductor into agent vs proxy mode with ConductorLink trait
- [**breaking**] rename End type param to Peer, endpoint vars to peer
- [**breaking**] more endpoint -> peer renames
- update UntypedRole to UntypedRole in doc examples
- [**breaking**] rename Endpoint to Role
- *(sacp)* rename JrRole to JrLink, Role type param to Link
- [**breaking**] rename JrRole to JrLink (and the Role associated type to Link)
- *(deps)* upgrade rmcp to 0.12.0
- *(sacp-conductor)* use explicit endpoints for ConductorToClient
- better debug logging
- cleanup the proxying code
- *(sacp)* fix cookbook doctest for start_session_proxy
- *(sacp)* add block_task() builder pattern for SessionBuilder
- *(sacp)* rename spawn_session to start_session
- *(sacp)* rename with_client to run_until
- *(sacp)* add lifetime-safe session proxying API
- *(sacp)* make all cookbook doctests compile and run
- *(sacp)* make cookbook examples compile
- *(sacp)* add cookbook module with common patterns
- update references for renamed methods
- *(sacp)* simplify McpServer handler architecture
- rename to McpNewSessionHandler for clarity
- *(sacp)* reduce public API surface for handler types
- *(sacp)* extract process_stream_concurrently utility

## [9.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v8.0.0...sacp-v9.0.0) - 2025-12-19

### Added

- *(sacp)* [**breaking**] require Send for JrMessageHandler with boxing witness macros
- *(sacp)* [**breaking**] use AsyncFnMut for tool closures with macro workaround
- *(sacp)* [**breaking**] merge pending_tasks into JrResponder and remove 'scope lifetime
- *(sacp)* [**breaking**] add scoped lifetime support for MCP servers
- *(sacp)* [**breaking**] add scoped lifetime to JrConnectionBuilder and JrConnection

### Other

- *(sacp)* update doc examples to use new macro witness API
- *(sacp)* simplify return types with impl Trait
- *(sacp)* simplify handler return types with impl Trait
- Merge pull request #88 from nikomatsakis/main
- *(sacp)* [**breaking**] remove to_future_hack from tool_fn API

## [8.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v7.0.0...sacp-v8.0.0) - 2025-12-17

### Fixed

- *(sacp)* add missing HasDefaultEndpoint bounds to handler methods
- *(sacp)* ensure NewSessionRequest flows through handler chain

### Other

- *(sacp)* add tool_fn!() macro for MCP tool registration

## [7.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v6.0.0...sacp-v7.0.0) - 2025-12-17

### Fixed

- match session messages from *agent*, not *client*

### Other

- update examples and tests to use new MCP server API
- update MCP server documentation for new API
- rename handler chain terminology to connection builder
- improved docs

## [4.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v3.0.0...sacp-v4.0.0) - 2025-12-15

### Fixed

- tests now pass
- *(sacp)* use correct camelCase field name for sessionId lookup
- *(sacp)* use blocking next() instead of try_next() in session read_update

### Other

- Merge pull request #77 from nikomatsakis/main
- *(sacp)* add guidance on when to use MatchMessage vs MatchMessageFrom
- *(sacp)* extract role-agnostic MatchMessage from MatchMessageFrom
- *(sacp)* rename MatchMessage to MatchMessageFrom
- process user-handlers first, dynamic later
- new session redirection support

## [3.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v2.0.0...sacp-v3.0.0) - 2025-12-12

### Added

- *(sacp-derive)* add derive macros for JrRequest, JrNotification, JrResponsePayload
- *(deps)* [**breaking**] upgrade rmcp from 0.8 to 0.9
- [**breaking**] introduce role-based connection API

### Other

- release
- add derive macro examples to JrRequest, JrNotification, JrResponsePayload traits
- use derive macros for proxy_protocol types
- fix broken GitHub org references in src/**/*.rs files
- fix broken GitHub org references in src/**/*.rs files

## [2.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.1.1...sacp-v2.0.0) - 2025-12-12

### Added

- *(sacp-derive)* add derive macros for JrRequest, JrNotification, JrResponsePayload
- *(deps)* [**breaking**] upgrade rmcp from 0.8 to 0.9
- [**breaking**] introduce role-based connection API

### Other

- add derive macro examples to JrRequest, JrNotification, JrResponsePayload traits
- use derive macros for proxy_protocol types
- fix broken GitHub org references in src/**/*.rs files
- fix broken GitHub org references in src/**/*.rs files

## [1.1.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.1.0...sacp-v1.1.1) - 2025-11-25

### Other

- move HTTP and implement shared code
- implementing HTTP MCP bridge

## [1.1.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0...sacp-v1.1.0) - 2025-11-22

### Added

- *(sacp)* add IntoHandled support to on_receive_message

### Fixed

- *(sacp-tee)* ensure tracing outputs to stderr

### Other

- *(sacp-tokio)* rewrite AcpAgent to use Lines instead of ByteStreams
- cleanup the unpin usage
- *(sacp)* create Lines component and have ByteStreams delegate to it
- report more info on parse failures

## [1.0.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0-alpha.7...sacp-v1.0.0) - 2025-11-13

### Fixed

- fix docs to not mention `Deref` impl

### Other

- Revert to state before 1.0.0 release
- release version 1.0.0 for all crates
- Merge pull request #32 from nikomatsakis/main

## [1.0.0-alpha.7](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0-alpha.6...sacp-v1.0.0-alpha.7) - 2025-11-12

### Other

- Merge pull request #30 from nikomatsakis/main
- *(sacp)* remove Deref impl from JrRequestCx
- *(sacp)* add common patterns section to crate-level documentation
- *(sacp)* add Component::serve() and simplify channel API

## [1.0.0-alpha.6](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0-alpha.5...sacp-v1.0.0-alpha.6) - 2025-11-12

### Added

- *(sacp)* extend IntoHandled support to notification handlers
- *(sacp)* add IntoHandled trait for flexible handler return types

### Other

- Merge pull request #28 from nikomatsakis/main
- *(sacp)* add tests for IntoHandled message transformation

## [1.0.0-alpha.5](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0-alpha.4...sacp-v1.0.0-alpha.5) - 2025-11-11

### Other

- [**breaking**] make Component trait ergonomic with async fn and introduce DynComponent
- clarify that Component should be implemented instead of Transport
- [**breaking**] make Component the primary trait with Transport as blanket impl

## [1.0.0-alpha.4](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0-alpha.3...sacp-v1.0.0-alpha.4) - 2025-11-11

### Other

- unify Transport and Component traits with BoxFuture-returning signatures
- create selective jsonrpcmsg re-export module
- move Component trait to sacp-proxy crate
- *(sacp)* improve IntoJrTransport and Component trait impls

## [1.0.0-alpha.3](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0-alpha.2...sacp-v1.0.0-alpha.3) - 2025-11-09

### Other

- Merge pull request #18 from nikomatsakis/main
- *(sacp-conductor)* route all message forwarding through central queue

## [1.0.0-alpha.2](https://github.com/symposium-dev/symposium-acp/compare/sacp-v1.0.0-alpha.1...sacp-v1.0.0-alpha.2) - 2025-11-08

### Added

- *(sacp)* add convenience methods for common connection patterns
- *(sacp)* add IntoJrConnectionTransport trait and ByteStreamTransport

### Other

- fix doctests for API refactoring
- wip wip wip
- wipwipwip
- introduce a `IntoJrHandler` trait
- [**breaking**] remove Unpin bounds and simplify transport API
- *(sacp)* clarify id: None semantics and remove phase references
- *(sacp)* split actors into protocol and transport layers

## [0.2.0](https://github.com/symposium-dev/symposium-acp/compare/sacp-v0.1.1...sacp-v0.2.0) - 2025-11-04

### Added

- *(sacp-tokio)* implement JrConnectionExt trait for to_agent
- create sacp-tokio crate and improve AcpAgent API
- *(sacp)* add AcpAgent utility and yolo-one-shot-client example

### Fixed

- fix github url

### Other

- add GitHub links to example files in sacp and sacp-proxy
- *(sacp)* [**breaking**] rename json_rpc_cx to connection_cx
- add deny(missing_docs) and document all public APIs
- *(sacp)* improve cx cloning pattern in doc examples
- *(sacp)* update crate-level documentation to match README
- factor "doc-test-only" code into its own crate
- make doctests build
- *(sacp)* fix remaining doc test compilation errors
- *(sacp)* enhance JrResponse method documentation
- *(sacp)* document event loop and concurrency model
- rename JsonRpcRequest to JrRequest
- use util.rs
- *(sacp)* move typed utilities to util module and add docs
- *(sacp)* remove mention of non-existent derive macro
- *(sacp)* fix doctests to compile instead of being ignored
- *(sacp)* add comprehensive rustdoc for JrConnection
- *(sacp)* rewrite README with streamlined quick start
- *(sacp)* use stderr for yolo-one-shot-client meta output

## [0.1.1](https://github.com/symposium-dev/symposium-acp/compare/sacp-v0.1.0...sacp-v0.1.1) - 2025-10-30

### Added

- *(sacp)* re-export all agent-client-protocol-schema types

### Fixed

- replace crate::Error with sacp::Error in dependent crates

### Other

- remove more uses of `agent_client_protocl_schema`
- replace acp:: with crate::/sacp:: throughout codebase
- rename JsonRpc* types to Jr* across all crates
- *(deps)* switch from agent-client-protocol to agent-client-protocol-schema
- *(sacp)* add simple_agent example demonstrating JsonRpcConnection usage
