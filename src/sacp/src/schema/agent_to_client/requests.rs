use serde::Serialize;

use crate::jsonrpc::{JsonRpcMessage, JsonRpcRequest, JsonRpcResponse};
use crate::schema::{
    CreateTerminalRequest, CreateTerminalResponse, KillTerminalRequest,
    KillTerminalResponse, ReadTextFileRequest, ReadTextFileResponse, ReleaseTerminalRequest,
    ReleaseTerminalResponse, RequestPermissionRequest, RequestPermissionResponse,
    TerminalOutputRequest, TerminalOutputResponse, WaitForTerminalExitRequest,
    WaitForTerminalExitResponse, WriteTextFileRequest, WriteTextFileResponse,
};
use crate::util::json_cast;

// Agent -> Client requests
// These are messages that agents send to clients/editors

// ============================================================================
// RequestPermissionRequest
// ============================================================================

const METHOD_REQUEST_PERMISSION: &str = "session/request_permission";

impl JsonRpcMessage for RequestPermissionRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_REQUEST_PERMISSION
    }

    fn method(&self) -> &str {
        METHOD_REQUEST_PERMISSION
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_REQUEST_PERMISSION {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for RequestPermissionRequest {
    type Response = RequestPermissionResponse;
}

impl JsonRpcResponse for RequestPermissionResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// WriteTextFileRequest
// ============================================================================

const METHOD_WRITE_TEXT_FILE: &str = "fs/write_text_file";

impl JsonRpcMessage for WriteTextFileRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_WRITE_TEXT_FILE
    }

    fn method(&self) -> &str {
        METHOD_WRITE_TEXT_FILE
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_WRITE_TEXT_FILE {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for WriteTextFileRequest {
    type Response = WriteTextFileResponse;
}

impl JsonRpcResponse for WriteTextFileResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// ReadTextFileRequest
// ============================================================================

const METHOD_READ_TEXT_FILE: &str = "fs/read_text_file";

impl JsonRpcMessage for ReadTextFileRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_READ_TEXT_FILE
    }

    fn method(&self) -> &str {
        METHOD_READ_TEXT_FILE
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_READ_TEXT_FILE {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for ReadTextFileRequest {
    type Response = ReadTextFileResponse;
}

impl JsonRpcResponse for ReadTextFileResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// CreateTerminalRequest
// ============================================================================

const METHOD_CREATE_TERMINAL: &str = "terminal/create";

impl JsonRpcMessage for CreateTerminalRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_CREATE_TERMINAL
    }

    fn method(&self) -> &str {
        METHOD_CREATE_TERMINAL
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_CREATE_TERMINAL {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for CreateTerminalRequest {
    type Response = CreateTerminalResponse;
}

impl JsonRpcResponse for CreateTerminalResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// TerminalOutputRequest
// ============================================================================

const METHOD_TERMINAL_OUTPUT: &str = "terminal/output";

impl JsonRpcMessage for TerminalOutputRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_TERMINAL_OUTPUT
    }

    fn method(&self) -> &str {
        METHOD_TERMINAL_OUTPUT
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_TERMINAL_OUTPUT {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for TerminalOutputRequest {
    type Response = TerminalOutputResponse;
}

impl JsonRpcResponse for TerminalOutputResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// ReleaseTerminalRequest
// ============================================================================

const METHOD_RELEASE_TERMINAL: &str = "terminal/release";

impl JsonRpcMessage for ReleaseTerminalRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_RELEASE_TERMINAL
    }

    fn method(&self) -> &str {
        METHOD_RELEASE_TERMINAL
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_RELEASE_TERMINAL {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for ReleaseTerminalRequest {
    type Response = ReleaseTerminalResponse;
}

impl JsonRpcResponse for ReleaseTerminalResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// WaitForTerminalExitRequest
// ============================================================================

const METHOD_WAIT_FOR_TERMINAL_EXIT: &str = "terminal/wait_for_exit";

impl JsonRpcMessage for WaitForTerminalExitRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_WAIT_FOR_TERMINAL_EXIT
    }

    fn method(&self) -> &str {
        METHOD_WAIT_FOR_TERMINAL_EXIT
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_WAIT_FOR_TERMINAL_EXIT {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for WaitForTerminalExitRequest {
    type Response = WaitForTerminalExitResponse;
}

impl JsonRpcResponse for WaitForTerminalExitResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}

// ============================================================================
// KillTerminalRequest
// ============================================================================

const METHOD_KILL_TERMINAL: &str = "terminal/kill";

impl JsonRpcMessage for KillTerminalRequest {
    fn matches_method(method: &str) -> bool {
        method == METHOD_KILL_TERMINAL
    }

    fn method(&self) -> &str {
        METHOD_KILL_TERMINAL
    }

    fn to_untyped_message(&self) -> Result<crate::UntypedMessage, crate::Error> {
        crate::UntypedMessage::new(self.method(), self)
    }

    fn parse_message(method: &str, params: &impl Serialize) -> Result<Self, crate::Error> {
        if method != METHOD_KILL_TERMINAL {
            return Err(crate::Error::method_not_found());
        }
        json_cast(params)
    }
}

impl JsonRpcRequest for KillTerminalRequest {
    type Response = KillTerminalResponse;
}

impl JsonRpcResponse for KillTerminalResponse {
    fn into_json(self, _method: &str) -> Result<serde_json::Value, crate::Error> {
        serde_json::to_value(self).map_err(crate::Error::into_internal_error)
    }

    fn from_value(_method: &str, value: serde_json::Value) -> Result<Self, crate::Error> {
        json_cast(&value)
    }
}
