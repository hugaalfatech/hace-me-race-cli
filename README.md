# HACE ME Race CLI

CLI tool for HACE engine race operations with Bridge and Observer patterns.

## Build

```bash
cargo build --release
```

## Usage

```bash
# Run a race
cargo run -- run --actor <actor-id> --method <method>

# Build artifact
cargo run -- build --source <path> --output <path>

# Deploy artifact
cargo run -- deploy --territory <territory> --endpoint <endpoint>

# Start server
cargo run -- serve --port <port> --host <host>
```

## Architecture

- **Bridge Pattern**: `RaceCliDispatcher` for extensible command dispatching
- **Observer Pattern**: `RaceEventObserver` for event notification
- **Dynamic Injection**: `ExecutionConfig` for environment-based configuration