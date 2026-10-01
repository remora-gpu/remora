//! `remora` command-line interface.

use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "remora", version, about = "Use NVIDIA GPUs over the network", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run a program with CUDA calls forwarded to a remote GPU server.
    Run {
        /// Program and its arguments.
        #[arg(trailing_var_arg = true, required = true)]
        program: Vec<String>,
    },
    /// Start a GPU server on this machine.
    Server,
    /// Diagnose network, versions and driver setup.
    Doctor,
    /// Print version and protocol information.
    Version,
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Version => {
            println!(
                "remora {} (protocol {})",
                env!("CARGO_PKG_VERSION"),
                remora_proto::PROTOCOL_VERSION
            );
            ExitCode::SUCCESS
        }
        Command::Run { .. } | Command::Server | Command::Doctor => {
            eprintln!("remora: this command is not implemented yet; see the roadmap");
            ExitCode::from(2)
        }
    }
}
