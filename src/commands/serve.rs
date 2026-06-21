use crate::lib::{CliError, CliResponse, ServeOpts};

/// Execute the serve command.
pub fn execute_serve(opts: ServeOpts) -> Result<CliResponse, CliError> {
    println!("Starting server on {}:{}", opts.host, opts.port);

    Ok(CliResponse {
        status: "success".to_string(),
        message: format!("Server started on {}:{}", opts.host, opts.port),
        data: None,
    })
}

/// Parse serve arguments into ServeOpts.
pub fn parse_serve_args(args: &[String]) -> Result<ServeOpts, CliError> {
    let mut opts = ServeOpts {
        port: 8080,
        host: "0.0.0.0".to_string(),
    };

    for i in 0..args.len() {
        match args[i].as_str() {
            "--port" | "-p" if i + 1 < args.len() => {
                opts.port = args[i + 1]
                    .parse()
                    .map_err(|_| CliError::ServeFailed("Invalid port".to_string()))?;
            }
            "--host" | "-h" if i + 1 < args.len() => {
                opts.host = args[i + 1].clone();
            }
            _ => {}
        }
    }

    Ok(opts)
}
