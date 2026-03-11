use crate::schema::{
    AuthenticateRequest, AuthenticateResponse, InitializeRequest, InitializeResponse,
    LoadSessionRequest, LoadSessionResponse, NewSessionRequest, NewSessionResponse, PromptRequest,
    PromptResponse, SetSessionConfigOptionRequest, SetSessionConfigOptionResponse,
    SetSessionModeRequest, SetSessionModeResponse,
};
use serde::Serialize;

use crate::jsonrpc::{JsonRpcMessage, JsonRpcRequest, JsonRpcResponse};
use crate::util::json_cast;

// Method constants
const METHOD_INITIALIZE: &str = "initialize";
const METHOD_AUTHENTICATE: &str = "authenticate";
const METHOD_SESSION_LOAD: &str = "session/load";
const METHOD_SESSION_NEW: &str = "session/new";
const METHOD_SESSION_PROMPT: &str = "session/prompt";
const METHOD_SESSION_SET_MODE: &str = "session/set_mode";
const METHOD_SESSION_SET_CONFIG_OPTION: &str = "session/set_config_option";

// ============================================================================
// InitializeRequest
// ============================================================================

impl JsonRpcMessage for InitializeRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_INITIALIZE
    }

    fn method(&self) -> &str {
        METHOD_INITIALIZE
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if !Self::matches_method(method) {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for InitializeRequest {
    type Response = InitializeResponse;
}

impl JsonRpcResponse for InitializeResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// AuthenticateRequest
// ============================================================================

impl JsonRpcMessage for AuthenticateRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_AUTHENTICATE
    }

    fn method(&self) -> &str {
        METHOD_AUTHENTICATE
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if !Self::matches_method(method) {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for AuthenticateRequest {
    type Response = AuthenticateResponse;
}

impl JsonRpcResponse for AuthenticateResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// LoadSessionRequest
// ============================================================================

impl JsonRpcMessage for LoadSessionRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_SESSION_LOAD
    }

    fn method(&self) -> &str {
        METHOD_SESSION_LOAD
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if !Self::matches_method(method) {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for LoadSessionRequest {
    type Response = LoadSessionResponse;
}

impl JsonRpcResponse for LoadSessionResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// NewSessionRequest
// ============================================================================

impl JsonRpcMessage for NewSessionRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_SESSION_NEW
    }

    fn method(&self) -> &str {
        METHOD_SESSION_NEW
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if !Self::matches_method(method) {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for NewSessionRequest {
    type Response = NewSessionResponse;
}

impl JsonRpcResponse for NewSessionResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// PromptRequest
// ============================================================================

impl JsonRpcMessage for PromptRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_SESSION_PROMPT
    }

    fn method(&self) -> &str {
        METHOD_SESSION_PROMPT
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if !Self::matches_method(method) {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for PromptRequest {
    type Response = PromptResponse;
}

impl JsonRpcResponse for PromptResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// SetSessionModeRequest
// ============================================================================

impl JsonRpcMessage for SetSessionModeRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_SESSION_SET_MODE
    }

    fn method(&self) -> &str {
        METHOD_SESSION_SET_MODE
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if !Self::matches_method(method) {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for SetSessionModeRequest {
    type Response = SetSessionModeResponse;
}

impl JsonRpcResponse for SetSessionModeResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// SetSessionConfigOptionRequest
// ============================================================================

impl JsonRpcMessage for SetSessionConfigOptionRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_SESSION_SET_CONFIG_OPTION
    }

    fn method(&self) -> &str {
        METHOD_SESSION_SET_CONFIG_OPTION
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if !Self::matches_method(method) {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for SetSessionConfigOptionRequest {
    type Response = SetSessionConfigOptionResponse;
}

impl JsonRpcResponse for SetSessionConfigOptionResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}
