//! Development tasks: `cargo xtask <task>`.

use std::process::{Command, ExitCode};

const TASKS: &[(&str, &str)] = &[
    ("ci", "run every check CI runs: fmt, clippy, tests"),
    ("fmt", "format the workspace"),
];

fn cargo(args: &[&str]) -> bool {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    eprintln!("$ cargo {}", args.join(" "));
    Command::new(cargo)
        .args(args)
        .status()
        .is_ok_and(|status| status.success())
}

fn ci() -> bool {
    cargo(&["fmt", "--all", "--check"])
        && cargo(&[
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ])
        && cargo(&["test", "--workspace", "--locked"])
}

fn main() -> ExitCode {
    let task = std::env::args().nth(1);
    let ok = match task.as_deref() {
        Some("ci") => ci(),
        Some("fmt") => cargo(&["fmt", "--all"]),
        _ => {
            eprintln!("usage: cargo xtask <task>\n");
            for (name, help) in TASKS {
                eprintln!("  {name:<6} {help}");
            }
            false
        }
    };
    if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
