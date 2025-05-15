# solmon

**Real-time Solana Network Monitoring CLI + TUI written in Rust**

`solmon` is a high-performance command-line tool for monitoring Solana blockchain metrics in real time. It queries the Solana JSON-RPC API and presents data through structured CLI output or an interactive terminal dashboard.

---

## Features

- Live TPS monitoring using performance samples from the Solana network
- Current epoch and slot tracking
- Validator block production metrics and success rates
- Terminal UI with live updates, sparkline TPS history, and top validator table
- Efficient background polling with async I/O

---

## Installation

### Install with Cargo (recommended)

```bash
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"
```

---

## Usage

### CLI Commands
```bash
solmon epoch               # Display current epoch and slot information
solmon status              # Show live TPS and validator performance
solmon validator <pubkey>  # View metrics for a specific validator
solmon watch               # Launch interactive terminal UI (press 'q' to quit)
```

---

## Architecture

- `tokio` for async polling and background tasks
- `reqwest` for Solana RPC over HTTP
- `serde` for typed JSON deserialization
- `clap` for CLI argument parsing
- `ratatui` and `crossterm` for the TUI backend
