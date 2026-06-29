// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use super::transport::McpTransportError;
use serde_json::Value;
use std::net::SocketAddr;

pub const DEFAULT_MCP_TOKEN_ENV: &str = "STEVENARELLA_MCP_TOKEN";
pub const REASON_EMPTY_TOKEN_ENV_NAME: &str = "empty_token_env_name";
pub const REASON_EMPTY_TOKEN_VALUE: &str = "empty_token_value";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpAuth {
    NotRequiredForLoopback,
    TokenEnv { name: String, token: String },
}

impl TcpAuth {
    pub(crate) fn required_token(&self) -> Option<&str> {
        match self {
            TcpAuth::NotRequiredForLoopback => None,
            TcpAuth::TokenEnv { token, .. } => Some(token.as_str()),
        }
    }
}

pub(crate) fn validate_tcp_auth<F>(
    bind_addr: SocketAddr,
    token_env: Option<&str>,
    token_lookup: &F,
) -> Result<TcpAuth, McpTransportError>
where
    F: Fn(&str) -> Option<String>,
{
    if bind_addr.ip().is_loopback() {
        return Ok(TcpAuth::NotRequiredForLoopback);
    }

    let token_env = normalized_token_env(token_env, bind_addr)?;
    let token = validate_token_value(&token_env, token_lookup)?;
    Ok(TcpAuth::TokenEnv {
        name: token_env,
        token,
    })
}

pub(crate) fn request_has_token(params: Option<&Value>, required_token: &str) -> bool {
    params
        .and_then(Value::as_object)
        .and_then(|params| params.get("token"))
        .and_then(Value::as_str)
        .map_or(false, |token| token == required_token)
}

fn normalized_token_env(
    token_env: Option<&str>,
    bind_addr: SocketAddr,
) -> Result<String, McpTransportError> {
    let Some(token_env) = token_env else {
        return Err(McpTransportError::MissingTokenEnvForNonLoopback { bind_addr });
    };
    let token_env = token_env.trim();
    if token_env.is_empty() {
        return Err(McpTransportError::InvalidTokenEnvName {
            reason: REASON_EMPTY_TOKEN_ENV_NAME,
        });
    }

    Ok(token_env.to_owned())
}

fn validate_token_value<F>(token_env: &str, token_lookup: &F) -> Result<String, McpTransportError>
where
    F: Fn(&str) -> Option<String>,
{
    let Some(value) = token_lookup(token_env) else {
        return Err(McpTransportError::MissingTokenValue {
            token_env: token_env.to_owned(),
        });
    };
    let value = value.trim();
    if value.is_empty() {
        return Err(McpTransportError::InvalidTokenValue {
            token_env: token_env.to_owned(),
            reason: REASON_EMPTY_TOKEN_VALUE,
        });
    }

    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOOPBACK_LISTEN: &str = "127.0.0.1:4700";
    const NON_LOOPBACK_LISTEN: &str = "0.0.0.0:4700";
    const TOKEN_ENV_NAME: &str = "STEVENARELLA_TEST_MCP_TOKEN";
    const TOKEN_VALUE: &str = "secret-token";

    #[test]
    fn loopback_auth_does_not_require_token() {
        let auth = validate_tcp_auth(LOOPBACK_LISTEN.parse().unwrap(), None, &|_| None).unwrap();

        assert_eq!(auth, TcpAuth::NotRequiredForLoopback);
        assert_eq!(auth.required_token(), None);
    }

    #[test]
    fn non_loopback_auth_accepts_trimmed_token_env_and_value() {
        let auth = validate_tcp_auth(
            NON_LOOPBACK_LISTEN.parse().unwrap(),
            Some(" STEVENARELLA_TEST_MCP_TOKEN "),
            &|name| {
                assert_eq!(name, TOKEN_ENV_NAME);
                Some(format!(" {TOKEN_VALUE} "))
            },
        )
        .unwrap();

        assert_eq!(auth.required_token(), Some(TOKEN_VALUE));
    }

    #[test]
    fn non_loopback_auth_rejects_empty_token_env_name() {
        let err = validate_tcp_auth(NON_LOOPBACK_LISTEN.parse().unwrap(), Some("  "), &|_| {
            Some(TOKEN_VALUE.to_owned())
        })
        .unwrap_err();

        assert_eq!(
            err,
            McpTransportError::InvalidTokenEnvName {
                reason: REASON_EMPTY_TOKEN_ENV_NAME,
            }
        );
    }

    #[test]
    fn request_token_match_is_exact() {
        let params = serde_json::json!({ "token": TOKEN_VALUE });
        let wrong_params = serde_json::json!({ "token": "wrong" });

        assert!(request_has_token(Some(&params), TOKEN_VALUE));
        assert!(!request_has_token(Some(&wrong_params), TOKEN_VALUE));
        assert!(!request_has_token(None, TOKEN_VALUE));
    }
}
