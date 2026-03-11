//! JsonRpcRequest and JsonRpcNotification implementations for ACP enum types.
//!
//! This module implements the JSON-RPC traits for the enum types from
//! agent-client-protocol-schema that represent all possible messages:
//! - ClientRequest/AgentResponse (messages agents receive/send)
//! - ClientNotification (notifications agents receive)
//! - AgentRequest/ClientResponse (messages clients receive/send)
//! - AgentNotification (notifications clients receive)

use crate::schema::{AgentNotification, AgentRequest, ClientNotification, ClientRequest};
use serde::Serialize;

use crate::jsonrpc::{JsonRpcMessage, JsonRpcNotification, JsonRpcRequest};
use crate::util::json_cast;

// ============================================================================
// Agent side (messages that agents receive)
// ============================================================================

impl JsonRpcMessage for ClientRequest {
    fn matches_method(_method: &str) -> bool {
        // Enum types match any method - the parsing will determine if it's valid
        true
    }

    fn method(&self) -> &str {
        match self {
            ClientRequest::InitializeRequest(_) => "initialize",
            ClientRequest::AuthenticateRequest(_) => "authenticate",
            ClientRequest::NewSessionRequest(_) => "session/new",
            ClientRequest::LoadSessionRequest(_) => "session/load",
            ClientRequest::SetSessionModeRequest(_) => "session/set_mode",
            ClientRequest::SetSessionConfigOptionRequest(_) => "session/set_config_option",
            ClientRequest::PromptRequest(_) => "session/prompt",
            ClientRequest::ExtMethodRequest(ext) => &ext.method,
            _ => "_unknown",
        }
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        match method {
            "initialize" => json_cast(params).map(ClientRequest::InitializeRequest),
            "authenticate" => json_cast(params).map(ClientRequest::AuthenticateRequest),
            "session/new" => json_cast(params).map(ClientRequest::NewSessionRequest),
            "session/load" => json_cast(params).map(ClientRequest::LoadSessionRequest),
            "session/set_mode" => json_cast(params).map(ClientRequest::SetSessionModeRequest),
            "session/set_config_option" => json_cast(params).map(ClientRequest::SetSessionConfigOptionRequest),
            "session/prompt" => json_cast(params).map(ClientRequest::PromptRequest),
            _ => {
                // Check for extension methods (prefixed with underscore)
                if let Some(custom_method) = method.strip_prefix('_') {
                    json_cast(params).map(|ext_req: crate::schema::ExtRequest| {
                        ClientRequest::ExtMethodRequest(crate::schema::ExtRequest::new(
                            custom_method.to_string(),
                            ext_req.params,
                        ))
                    })
                } else {
                    Err(crate::Error::method_not_found())
                }
            }
        }
    }
}

impl JsonRpcRequest for ClientRequest {
    type Response = serde_json::Value;
}

impl JsonRpcMessage for ClientNotification {
    fn matches_method(_method: &str) -> bool {
        // Enum types match any method - the parsing will determine if it's valid
        true
    }

    fn method(&self) -> &str {
        match self {
            ClientNotification::CancelNotification(_) => "session/cancel",
            ClientNotification::ExtNotification(ext) => &ext.method,
            _ => "_unknown",
        }
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        match method {
            "session/cancel" => json_cast(params).map(ClientNotification::CancelNotification),
            _ => {
                // Check for extension notifications (prefixed with underscore)
                if let Some(custom_method) = method.strip_prefix('_') {
                    json_cast(params).map(|ext_notif: crate::schema::ExtNotification| {
                        ClientNotification::ExtNotification(crate::schema::ExtNotification::new(
                            custom_method.to_string(),
                            ext_notif.params,
                        ))
                    })
                } else {
                    Err(crate::Error::method_not_found())
                }
            }
        }
    }
}

impl JsonRpcNotification for ClientNotification {}

// ============================================================================
// Client side (messages that clients/editors receive)
// ============================================================================

impl JsonRpcMessage for AgentRequest {
    fn matches_method(_method: &str) -> bool {
        // Enum types match any method - the parsing will determine if it's valid
        true
    }

    fn method(&self) -> &str {
        match self {
            AgentRequest::WriteTextFileRequest(_) => "fs/write_text_file",
            AgentRequest::ReadTextFileRequest(_) => "fs/read_text_file",
            AgentRequest::RequestPermissionRequest(_) => "session/request_permission",
            AgentRequest::CreateTerminalRequest(_) => "terminal/create",
            AgentRequest::TerminalOutputRequest(_) => "terminal/output",
            AgentRequest::ReleaseTerminalRequest(_) => "terminal/release",
            AgentRequest::WaitForTerminalExitRequest(_) => "terminal/wait_for_exit",
            AgentRequest::KillTerminalRequest(_) => "terminal/kill",
            AgentRequest::ExtMethodRequest(ext) => &ext.method,
            _ => "_unknown",
        }
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        match method {
            "fs/write_text_file" => json_cast(params).map(AgentRequest::WriteTextFileRequest),
            "fs/read_text_file" => json_cast(params).map(AgentRequest::ReadTextFileRequest),
            "session/request_permission" => {
                json_cast(params).map(AgentRequest::RequestPermissionRequest)
            }
            "terminal/create" => json_cast(params).map(AgentRequest::CreateTerminalRequest),
            "terminal/output" => json_cast(params).map(AgentRequest::TerminalOutputRequest),
            "terminal/release" => json_cast(params).map(AgentRequest::ReleaseTerminalRequest),
            "terminal/wait_for_exit" => {
                json_cast(params).map(AgentRequest::WaitForTerminalExitRequest)
            }
            "terminal/kill" => json_cast(params).map(AgentRequest::KillTerminalRequest),
            _ => {
                // Check for extension methods (prefixed with underscore)
                if let Some(custom_method) = method.strip_prefix('_') {
                    json_cast(params).map(|ext_req: crate::schema::ExtRequest| {
                        AgentRequest::ExtMethodRequest(crate::schema::ExtRequest::new(
                            custom_method.to_string(),
                            ext_req.params,
                        ))
                    })
                } else {
                    Err(crate::Error::method_not_found())
                }
            }
        }
    }
}

impl JsonRpcRequest for AgentRequest {
    type Response = serde_json::Value;
}

impl JsonRpcMessage for AgentNotification {
    fn matches_method(_method: &str) -> bool {
        // Enum types match any method - the parsing will determine if it's valid
        true
    }

    fn method(&self) -> &str {
        match self {
            AgentNotification::SessionNotification(_) => "session/update",
            AgentNotification::ExtNotification(ext) => &ext.method,
            _ => "_unknown",
        }
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        match method {
            "session/update" => json_cast(params).map(AgentNotification::SessionNotification),
            _ => {
                // Check for extension notifications (prefixed with underscore)
                if let Some(custom_method) = method.strip_prefix('_') {
                    json_cast(params).map(|ext_notif: crate::schema::ExtNotification| {
                        AgentNotification::ExtNotification(crate::schema::ExtNotification::new(
                            custom_method.to_string(),
                            ext_notif.params,
                        ))
                    })
                } else {
                    Err(crate::Error::method_not_found())
                }
            }
        }
    }
}

impl JsonRpcNotification for AgentNotification {}
