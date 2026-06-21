use crate::lib::{DeployTarget, CliError, CliResponse};
use std::path::PathBuf;
use std::process::Command;

/// Execute the deploy command.
pub fn execute_deploy(target: DeployTarget, artifact: &PathBuf) -> Result<CliResponse, CliError> {
    println!("Deploying artifact: {:?}", artifact);
    println!("Territory: {}", target.territory);
    println!("Endpoint: {}", target.endpoint);

    register_in_caw(artifact, &target.territory)?;

    Ok(CliResponse {
        status: "success".to_string(),
        message: format!("Deployed to {} at {}", target.territory, target.endpoint),
        data: None,
    })
}

/// Register artifact in CAW (Central Artifact Warehouse).
fn register_in_caw(artifact: &PathBuf, territory: &str) -> Result<(), CliError> {
    let artifact_name = artifact
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let mut cmd = Command::new("caw");
    cmd.arg("register")
        .arg("--artifact")
        .arg(artifact_name)
        .arg("--territory")
        .arg(territory)
        .arg("--type")
        .arg("cap");

    let output = cmd.output().map_err(|e| CliError::DeployFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(CliError::DeployFailed("CAW registration failed".to_string()));
    }

    Ok(())
}

/// Parse deploy arguments into DeployTarget.
pub fn parse_deploy_args(args: &[String]) -> Result<(DeployTarget, PathBuf), CliError> {
    let mut target = DeployTarget {
        territory: "local".to_string(),
        endpoint: "127.0.0.1:8080".to_string(),
    };
    let mut artifact = PathBuf::from("dist");

    for i in 0..args.len() {
        match args[i].as_str() {
            "--territory" | "-t" if i + 1 < args.len() => {
                target.territory = args[i + 1].clone();
            }
            "--endpoint" | "-e" if i + 1 < args.len() => {
                target.endpoint = args[i + 1].clone();
            }
            "--artifact" | "-a" if i + 1 < args.len() => {
                artifact = PathBuf::from(&args[i + 1]);
            }
            _ => {}
        }
    }

    Ok((target, artifact))
}
