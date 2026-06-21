pub mod commands;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ─── Constants ───────────────────────────────────────────────────────────

pub const DEFAULT_TERRITORY: &str = "local";
pub const DEFAULT_ENDPOINT: &str = "127.0.0.1:8080";

// ─── Execution Base ──────────────────────────────────────────────────────

/// Configuration provided by the execution environment to avoid hardcoded defaults.
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    pub default_territory: String,
    pub default_endpoint: String,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            default_territory: DEFAULT_TERRITORY.to_string(),
            default_endpoint: DEFAULT_ENDPOINT.to_string(),
        }
    }
}

// ─── Types ───────────────────────────────────────────────────────────────

/// Context for the CLI execution, containing working directory and configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliContext {
    pub working_dir: PathBuf,
    pub config: Config,
}

/// Configuration settings for the RACE CLI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub race_endpoint: String,
}

/// Options for starting the RACE server.
#[derive(Debug, Clone)]
pub struct ServeOpts {
    pub port: u16,
    pub host: String,
}

/// Command to run a specific actor method.
#[derive(Debug, Clone)]
pub struct RunCmd {
    pub actor_id: String,
    pub method: String,
}

/// Parameters passed to a run command.
#[derive(Debug, Clone)]
pub struct Params {
    pub args: Vec<String>,
}

/// Target specification for building an artifact.
#[derive(Debug, Clone)]
pub struct BuildTarget {
    pub source: PathBuf,
    pub output: PathBuf,
}

/// Configuration for the build process.
#[derive(Debug, Clone)]
pub struct BuildCfg {
    pub profile: String,
    pub features: Vec<String>,
}

/// Target specification for deploying an artifact.
#[derive(Debug, Clone)]
pub struct DeployTarget {
    pub territory: String,
    pub endpoint: String,
}

// ─── Responses ───────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct CliResponse {
    pub status: String,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct ServerHandle {
    pub pid: u32,
    pub endpoint: String,
}

// ─── Errors ──────────────────────────────────────────────────────────────

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Build failed: {0}")]
    BuildFailed(String),

    #[error("Deploy failed: {0}")]
    DeployFailed(String),

    #[error("Run failed: {0}")]
    RunFailed(String),

    #[error("Serve failed: {0}")]
    ServeFailed(String),
}

// ─── Traits ──────────────────────────────────────────────────────────────

/// Bridge trait for CLI command dispatching.
/// Allows runtime command registration and extensible dispatching.
pub trait RaceCliDispatcher: Send + Sync {
    /// Dispatch a command with the given arguments.
    fn dispatch(&self, cmd: &str, args: &[String]) -> Result<CliResponse, CliError>;

    /// Get the list of supported commands.
    fn supported_commands(&self) -> Vec<&str>;
}

/// Observer trait for race events.
/// Implements the Observer pattern for event notification.
pub trait RaceEventObserver: Send + Sync {
    /// Called when an event occurs.
    fn on_event(&self, event: &str, data: &serde_json::Value) -> Result<(), CliError>;

    /// Get the observer's name.
    fn name(&self) -> &str;
}

// ─── CLI Observer ────────────────────────────────────────────────────────

/// CLI event observer implementation.
pub struct CliObserver {
    name: String,
}

impl CliObserver {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl RaceEventObserver for CliObserver {
    fn on_event(&self, event: &str, data: &serde_json::Value) -> Result<(), CliError> {
        println!("[{}] Event: {} | Data: {}", self.name, event, data);
        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// ─── Command Handler ─────────────────────────────────────────────────────

/// Handle the RACE command with the given arguments.
pub fn handle_race_command(args: Vec<String>, context: &CliContext) -> Result<CliResponse, CliError> {
    if args.len() < 2 {
        return Err(CliError::InvalidCommand("Usage: race <command> [args]".to_string()));
    }

    let exec_base = ExecutionConfig::default();

    match args[1].as_str() {
        "build" => {
            let target = parse_build_target(&args[2..], &exec_base)?;
            Ok(CliResponse {
                status: "success".to_string(),
                message: format!("Building artifact: {:?}", target),
                data: None,
            })
        }
        "deploy" => {
            let target = parse_deploy_target(&args[2..], &exec_base)?;
            Ok(CliResponse {
                status: "success".to_string(),
                message: format!("Deploying to {} at {}", target.territory, target.endpoint),
                data: None,
            })
        }
        "run" => {
            let cmd = parse_run_command(&args[2..])?;
            Ok(CliResponse {
                status: "success".to_string(),
                message: format!("Running actor: {} method: {}", cmd.actor_id, cmd.method),
                data: None,
            })
        }
        "serve" => {
            let opts = parse_serve_opts(&args[2..])?;
            Ok(CliResponse {
                status: "success".to_string(),
                message: format!("Starting server on {}:{}", opts.host, opts.port),
                data: None,
            })
        }
        _ => Err(CliError::InvalidCommand(args[1].clone())),
    }
}

// ─── Parsers ─────────────────────────────────────────────────────────────

fn parse_build_target(args: &[String], exec_base: &ExecutionConfig) -> Result<BuildTarget, CliError> {
    let mut source = PathBuf::from(".");
    let mut output = PathBuf::from("dist");

    for i in 0..args.len() {
        match args[i].as_str() {
            "--source" | "-s" if i + 1 < args.len() => {
                source = PathBuf::from(&args[i + 1]);
            }
            "--output" | "-o" if i + 1 < args.len() => {
                output = PathBuf::from(&args[i + 1]);
            }
            _ => {}
        }
    }

    Ok(BuildTarget { source, output })
}

fn parse_deploy_target(args: &[String], exec_base: &ExecutionConfig) -> Result<DeployTarget, CliError> {
    let mut territory = exec_base.default_territory.clone();
    let mut endpoint = exec_base.default_endpoint.clone();

    for i in 0..args.len() {
        match args[i].as_str() {
            "--territory" | "-t" if i + 1 < args.len() => {
                territory = args[i + 1].clone();
            }
            "--endpoint" | "-e" if i + 1 < args.len() => {
                endpoint = args[i + 1].clone();
            }
            _ => {}
        }
    }

    if !endpoint.contains(':') {
        return Err(CliError::DeployFailed(
            "Invalid endpoint format: must contain ':port'".to_string(),
        ));
    }

    Ok(DeployTarget { territory, endpoint })
}

fn parse_run_command(args: &[String]) -> Result<RunCmd, CliError> {
    let mut actor_id = String::new();
    let mut method = String::new();

    for i in 0..args.len() {
        match args[i].as_str() {
            "--actor" | "-a" if i + 1 < args.len() => {
                actor_id = args[i + 1].clone();
            }
            "--method" | "-m" if i + 1 < args.len() => {
                method = args[i + 1].clone();
            }
            _ => {}
        }
    }

    if actor_id.is_empty() || method.is_empty() {
        return Err(CliError::RunFailed(
            "Both --actor and --method are required".to_string(),
        ));
    }

    Ok(RunCmd { actor_id, method })
}

fn parse_serve_opts(args: &[String]) -> Result<ServeOpts, CliError> {
    let mut port = 8080u16;
    let mut host = "0.0.0.0".to_string();

    for i in 0..args.len() {
        match args[i].as_str() {
            "--port" | "-p" if i + 1 < args.len() => {
                port = args[i + 1]
                    .parse()
                    .map_err(|_| CliError::ServeFailed("Invalid port".to_string()))?;
            }
            "--host" | "-h" if i + 1 < args.len() => {
                host = args[i + 1].clone();
            }
            _ => {}
        }
    }

    Ok(ServeOpts { port, host })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_config_default() {
        let config = ExecutionConfig::default();
        assert_eq!(config.default_territory, "local");
        assert_eq!(config.default_endpoint, "127.0.0.1:8080");
    }

    #[test]
    fn test_parse_deploy_target_with_defaults() {
        let exec_base = ExecutionConfig::default();
        let target = parse_deploy_target(&["--territory".to_string(), "prod".to_string()], &exec_base).unwrap();
        assert_eq!(target.territory, "prod");
        assert_eq!(target.endpoint, "127.0.0.1:8080");
    }

    #[test]
    fn test_parse_deploy_target_invalid_endpoint() {
        let exec_base = ExecutionConfig::default();
        let result = parse_deploy_target(&["--endpoint".to_string(), "localhost8080".to_string()], &exec_base);
        assert!(result.is_err());
    }
}
