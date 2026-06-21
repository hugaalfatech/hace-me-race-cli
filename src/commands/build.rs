use crate::lib::{BuildCfg, BuildTarget, CliError, CliResponse};
use std::path::PathBuf;

/// Execute the build command.
pub fn execute_build(target: BuildTarget, cfg: BuildCfg) -> Result<CliResponse, CliError> {
    println!("Building with profile: {} and features: {:?}", cfg.profile, cfg.features);
    println!("Source: {:?}", target.source);
    println!("Output: {:?}", target.output);

    Ok(CliResponse {
        status: "success".to_string(),
        message: "Build completed successfully".to_string(),
        data: None,
    })
}

/// Parse build arguments into BuildTarget and BuildCfg.
pub fn parse_build_args(args: &[String]) -> Result<(BuildTarget, BuildCfg), CliError> {
    let mut target = BuildTarget {
        source: PathBuf::from("."),
        output: PathBuf::from("dist"),
    };
    let mut cfg = BuildCfg {
        profile: "debug".to_string(),
        features: vec![],
    };

    for i in 0..args.len() {
        match args[i].as_str() {
            "--source" | "-s" if i + 1 < args.len() => {
                target.source = PathBuf::from(&args[i + 1]);
            }
            "--output" | "-o" if i + 1 < args.len() => {
                target.output = PathBuf::from(&args[i + 1]);
            }
            "--profile" if i + 1 < args.len() => {
                cfg.profile = args[i + 1].clone();
            }
            "--features" if i + 1 < args.len() => {
                cfg.features = args[i + 1].split(',').map(String::from).collect();
            }
            _ => {}
        }
    }

    Ok((target, cfg))
}
