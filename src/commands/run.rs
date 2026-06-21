use crate::lib::{CliError, CliResponse, RunCmd, Params};

/// Execute the run command.
pub fn execute_run(cmd: RunCmd, params: Params) -> Result<CliResponse, CliError> {
    println!("Running actor: {}", cmd.actor_id);
    println!("Method: {}", cmd.method);
    println!("Args: {:?}", params.args);

    Ok(CliResponse {
        status: "success".to_string(),
        message: format!("Executed {} on {}", cmd.method, cmd.actor_id),
        data: None,
    })
}

/// Parse run arguments into RunCmd and Params.
pub fn parse_run_args(args: &[String]) -> Result<(RunCmd, Params), CliError> {
    let mut run_cmd = RunCmd {
        actor_id: String::new(),
        method: String::new(),
    };
    let mut params = Params {
        args: vec![],
    };

    for i in 0..args.len() {
        match args[i].as_str() {
            "--actor" | "-a" if i + 1 < args.len() => {
                run_cmd.actor_id = args[i + 1].clone();
            }
            "--method" | "-m" if i + 1 < args.len() => {
                run_cmd.method = args[i + 1].clone();
            }
            _ => {
                params.args.push(args[i].clone());
            }
        }
    }

    if run_cmd.actor_id.is_empty() || run_cmd.method.is_empty() {
        return Err(CliError::RunFailed(
            "Both --actor and --method are required".to_string(),
        ));
    }

    Ok((run_cmd, params))
}
