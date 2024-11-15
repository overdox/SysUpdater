use std::process::Stdio;
use std::io::{self};
use colored::*;
use tokio::task;
use tokio::process::Command as TokioCommand;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    if !nix::unistd::Uid::effective().is_root() {
        eprintln!("{}", "This script must be run as root!".red());
        std::process::exit(1);
    }

    // Run updates in parallel
    let system_update = task::spawn(async { update_system().await });
    let flatpak_update = task::spawn(async { update_flatpak().await });

    // Wait for both tasks to complete
    system_update.await??;
    flatpak_update.await??;

    println!("{}", "System updates have been completed.".green());
    check_reboot().await?;
    Ok(())
}

async fn run_cmd_with_output(command: &str, prefix: &str) -> Result<()> {
    println!("{}", format!("Executing: {}", command).cyan());
    let mut cmd = TokioCommand::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = cmd.stdout.take().unwrap();
    let stderr = cmd.stderr.take().unwrap();

    let stdout_reader = tokio::io::BufReader::new(stdout);
    let stderr_reader = tokio::io::BufReader::new(stderr);

    tokio::select! {
        _ = read_stream(stdout_reader, prefix, false) => {},
        _ = read_stream(stderr_reader, prefix, true) => {},
    }

    let status = cmd.wait().await?;
    if !status.success() {
        eprintln!("{}", format!("Command failed: {}", command).red());
        anyhow::bail!("Command failed: {}", command);
    }
    Ok(())
}

async fn read_stream<R: tokio::io::AsyncBufReadExt + Unpin>(
    mut reader: R,
    prefix: &str,
    is_error: bool,
) {
    let prefix_colored = if is_error {
        prefix.red().bold()
    } else {
        prefix.white().bold()
    };
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        println!("{} {}", prefix_colored, line);
    }
}

async fn update_system() -> Result<()> {
    println!("{}", "Starting system update:".cyan());
    println!("{}", "=".repeat(80).cyan());
    run_cmd_with_output("dnf5 update --refresh -y", "[System]").await?;
    run_cmd_with_output("dnf5 -y autoremove", "[System]").await?;
    println!("{}", "=".repeat(80).cyan());
    println!("{}", "System update done...".green());
    Ok(())
}

async fn update_flatpak() -> Result<()> {
    println!("{}", "Starting flatpak update:".cyan());
    println!("{}", "=".repeat(80).cyan());
    run_cmd_with_output("flatpak update -y", "[Flatpak]").await?;
    run_cmd_with_output("flatpak uninstall --unused", "[Flatpak]").await?;
    println!("{}", "=".repeat(80).cyan());
    println!("{}", "Flatpak update done...".green());
    Ok(())
}

async fn check_reboot() -> Result<()> {
    let output = TokioCommand::new("sh")
        .arg("-c")
        .arg("dnf5 needs-restarting")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let exit_status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout);

    if exit_status.success() {
        println!("{}", "Reboot is not necessary.".green());
    } else if exit_status.code() == Some(1) {
        println!("{}", "Reboot is required to fully utilize these updates.".red());
        println!("\nChoose an action:\n1. Reboot\n2. Exit the terminal");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        match choice.trim() {
            "1" => run_cmd_with_output("sudo reboot", "[Reboot]").await?,
            "2" => println!("{}", "Ok, bye!".green()),
            _ => {
                eprintln!("{}", "Invalid choice. Exiting.".red());
                std::process::exit(1);
            }
        }
    } else {
        eprintln!(
            "{}",
            format!(
                "Unexpected exit status from 'dnf5 needs-restarting': {}",
                exit_status
            )
            .red()
        );
        anyhow::bail!("Unexpected exit status from 'dnf5 needs-restarting'");
    }
    Ok(())
}
