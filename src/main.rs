//! SysUpdater - Production-ready Fedora system updater
//!
//! A robust tool for automating system, Flatpak, and firmware updates
//! with proper error handling, logging, and user feedback.

use std::{path::PathBuf, process::ExitCode, sync::Arc, time::Duration};
use clap::Parser;
use colored::Colorize;
use tokio::{process::Command, sync::Mutex};
use tracing::{debug, error, info, warn, Level};

mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum UpdateError {
        #[error("Must run as root. Use: sudo sysupdater")]
        NotRoot,
        #[error("No network connectivity")]
        NoNetwork,
        #[error("Command failed: {cmd}\n  Exit code: {code}\n  Details: {details}")]
        CommandFailed { cmd: String, code: i32, details: String },
        #[error("Command not found: {0}")]
        CommandNotFound(String),
        #[error("Configuration error: {0}")]
        Config(String),
        #[error("IO error: {0}")]
        Io(#[from] std::io::Error),
        #[error("Operation cancelled by user")]
        Cancelled,
    }

    pub type Result<T> = std::result::Result<T, UpdateError>;
}

mod config {
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct Config {
        pub system: SystemConfig,
        pub flatpak: FlatpakConfig,
        pub firmware: FirmwareConfig,
        pub logging: LoggingConfig,
        pub network: NetworkConfig,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct SystemConfig {
        pub enabled: bool,
        pub auto_remove: bool,
        pub refresh: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct FlatpakConfig {
        pub enabled: bool,
        pub remove_unused: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct FirmwareConfig {
        pub enabled: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct LoggingConfig {
        pub file: PathBuf,
        pub level: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct NetworkConfig {
        pub check_url: String,
        pub timeout_secs: u64,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                system: SystemConfig::default(),
                flatpak: FlatpakConfig::default(),
                firmware: FirmwareConfig::default(),
                logging: LoggingConfig::default(),
                network: NetworkConfig::default(),
            }
        }
    }

    impl Default for SystemConfig {
        fn default() -> Self {
            Self { enabled: true, auto_remove: true, refresh: true }
        }
    }

    impl Default for FlatpakConfig {
        fn default() -> Self {
            Self { enabled: true, remove_unused: true }
        }
    }

    impl Default for FirmwareConfig {
        fn default() -> Self {
            Self { enabled: false }
        }
    }

    impl Default for LoggingConfig {
        fn default() -> Self {
            Self {
                file: PathBuf::from("/var/log/sysupdater.log"),
                level: "info".into(),
            }
        }
    }

    impl Default for NetworkConfig {
        fn default() -> Self {
            Self {
                check_url: "https://fedoraproject.org".into(),
                timeout_secs: 10,
            }
        }
    }

    impl Config {
        pub fn load(path: Option<&PathBuf>) -> Self {
            let paths = [
                path.cloned(),
                Some(PathBuf::from("/etc/sysupdater.toml")),
                dirs::config_dir().map(|p| p.join("sysupdater/config.toml")),
            ];

            for p in paths.into_iter().flatten() {
                if p.exists() {
                    if let Ok(content) = std::fs::read_to_string(&p) {
                        if let Ok(cfg) = toml::from_str(&content) {
                            tracing::info!("Loaded config from {}", p.display());
                            return cfg;
                        }
                    }
                }
            }
            Self::default()
        }
    }
}

mod cli {
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser, Debug, Clone)]
    #[command(name = "sysupdater", version, about = "Fedora System Update Automation", long_about = None)]
    #[command(propagate_version = true)]
    pub struct Args {
        /// Check and display available updates without installing
        #[arg(long, short = 'r')]
        pub refresh: bool,

        /// Update everything (system, flatpak, and optionally firmware)
        #[arg(long, short = 'u')]
        pub update_all: bool,

        /// Update only system packages (dnf5)
        #[arg(long)]
        pub update_system: bool,

        /// Update only Flatpak applications
        #[arg(long)]
        pub update_flatpak: bool,

        /// Update only firmware (requires fwupd)
        #[arg(long)]
        pub update_firmware: bool,

        /// Include firmware in --update-all
        #[arg(long, short = 'f')]
        pub firmware: bool,

        /// Dry run - show what would be done without executing
        #[arg(long, short = 'n')]
        pub dry_run: bool,

        /// Skip reboot prompt after updates
        #[arg(long)]
        pub no_reboot_prompt: bool,

        /// Skip network connectivity check
        #[arg(long)]
        pub no_network_check: bool,

        /// Run updates in parallel (may produce interleaved output)
        #[arg(long)]
        pub parallel: bool,

        /// Path to config file
        #[arg(long, short = 'c')]
        pub config: Option<PathBuf>,

        /// Increase verbosity (-v, -vv, -vvv)
        #[arg(long, short = 'v', action = clap::ArgAction::Count)]
        pub verbose: u8,

        /// Quiet mode - minimal output
        #[arg(long, short = 'q')]
        pub quiet: bool,
    }

    impl Args {
        /// Returns true if no action flags were provided
        pub fn is_default(&self) -> bool {
            !self.refresh
                && !self.update_all
                && !self.update_system
                && !self.update_flatpak
                && !self.update_firmware
        }
    }
}

mod system {
    use crate::error::{Result, UpdateError};
    use std::time::Duration;

    pub fn check_root() -> Result<()> {
        if nix::unistd::Uid::effective().is_root() {
            Ok(())
        } else {
            Err(UpdateError::NotRoot)
        }
    }

    pub fn command_exists(cmd: &str) -> bool {
        std::process::Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub async fn check_network(url: &str, timeout: Duration) -> Result<()> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|e| UpdateError::Config(e.to_string()))?;

        client
            .head(url)
            .send()
            .await
            .map_err(|_| UpdateError::NoNetwork)?;

        Ok(())
    }
}

mod updater {
    use crate::error::{Result, UpdateError};
    use colored::Colorize;
    use indicatif::{ProgressBar, ProgressStyle};
    use std::{process::Stdio, sync::Arc, time::Duration};
    use tokio::{
        io::{AsyncBufReadExt, BufReader},
        process::Command,
        sync::Mutex,
    };
    use tracing::{debug, info};

    #[derive(Debug, Clone, Default)]
    pub struct UpdateSummary {
        pub system_updated: bool,
        pub flatpak_updated: bool,
        pub firmware_updated: bool,
        pub errors: Vec<String>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct AvailableUpdates {
        pub system: Vec<String>,
        pub flatpak: Vec<String>,
        pub firmware: Vec<String>,
    }

    impl AvailableUpdates {
        pub fn total_count(&self) -> usize {
            self.system.len() + self.flatpak.len() + self.firmware.len()
        }

        pub fn is_empty(&self) -> bool {
            self.total_count() == 0
        }
    }

    pub struct Updater {
        dry_run: bool,
        quiet: bool,
        summary: Arc<Mutex<UpdateSummary>>,
    }

    impl Updater {
        pub fn new(dry_run: bool, quiet: bool) -> Self {
            Self {
                dry_run,
                quiet,
                summary: Arc::new(Mutex::new(UpdateSummary::default())),
            }
        }

        pub async fn summary(&self) -> UpdateSummary {
            self.summary.lock().await.clone()
        }

        fn create_spinner(&self, msg: &str) -> ProgressBar {
            if self.quiet {
                return ProgressBar::hidden();
            }
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                    .template("{spinner:.cyan} {msg}")
                    .unwrap(),
            );
            pb.set_message(msg.to_string());
            pb.enable_steady_tick(Duration::from_millis(80));
            pb
        }

        async fn run_command(
            &self,
            cmd: &str,
            args: &[&str],
            prefix: &str,
        ) -> Result<Vec<String>> {
            let full_cmd = format!("{} {}", cmd, args.join(" "));
            info!("Executing: {}", full_cmd);

            if self.dry_run {
                println!("{} [DRY RUN] {}", prefix.cyan().bold(), full_cmd);
                return Ok(vec![]);
            }

            let mut child = Command::new(cmd)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        UpdateError::CommandNotFound(cmd.to_string())
                    } else {
                        UpdateError::Io(e)
                    }
                })?;

            let stdout = child.stdout.take().expect("stdout piped");
            let stderr = child.stderr.take().expect("stderr piped");

            let prefix_out = format!("{}", prefix.white().bold());
            let prefix_err = format!("{}", prefix.red().bold());
            let quiet = self.quiet;
            let output_lines = Arc::new(Mutex::new(Vec::new()));
            let lines_clone = output_lines.clone();

            let stdout_handle = tokio::spawn(async move {
                let mut reader = BufReader::new(stdout).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    if !quiet {
                        println!("{} {}", prefix_out, line);
                    }
                    lines_clone.lock().await.push(line);
                }
            });

            let stderr_handle = tokio::spawn(async move {
                let mut reader = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = reader.next_line().await {
                    if !quiet {
                        eprintln!("{} {}", prefix_err, line);
                    }
                    debug!("stderr: {}", line);
                }
            });

            let _ = tokio::join!(stdout_handle, stderr_handle);

            let status = child.wait().await?;
            let lines = output_lines.lock().await.clone();

            if !status.success() {
                let code = status.code().unwrap_or(-1);
                return Err(UpdateError::CommandFailed {
                    cmd: full_cmd,
                    code,
                    details: lines.join("\n"),
                });
            }

            Ok(lines)
        }

        async fn run_command_silent(&self, cmd: &str, args: &[&str]) -> Result<Vec<String>> {
            let output = Command::new(cmd)
                .args(args)
                .output()
                .await
                .map_err(|e| {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        UpdateError::CommandNotFound(cmd.to_string())
                    } else {
                        UpdateError::Io(e)
                    }
                })?;

            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.lines().map(|s| s.to_string()).collect())
        }

        pub async fn check_available_updates(&self) -> Result<AvailableUpdates> {
            let mut updates = AvailableUpdates::default();

            // Check system updates
            if crate::system::command_exists("dnf5") {
                let spinner = self.create_spinner("Checking system updates...");
                if let Ok(lines) = self
                    .run_command_silent("dnf5", &["check-upgrade", "--refresh", "-q"])
                    .await
                {
                    updates.system = lines
                        .into_iter()
                        .filter(|l| !l.is_empty() && !l.starts_with("Last metadata"))
                        .collect();
                }
                spinner.finish_and_clear();
            }

            // Check flatpak updates
            if crate::system::command_exists("flatpak") {
                let spinner = self.create_spinner("Checking Flatpak updates...");
                if let Ok(lines) = self
                    .run_command_silent("flatpak", &["remote-ls", "--updates"])
                    .await
                {
                    updates.flatpak = lines.into_iter().filter(|l| !l.is_empty()).collect();
                }
                spinner.finish_and_clear();
            }

            // Check firmware updates
            if crate::system::command_exists("fwupdmgr") {
                let spinner = self.create_spinner("Checking firmware updates...");
                let _ = self.run_command_silent("fwupdmgr", &["refresh", "--force"]).await;
                if let Ok(lines) = self
                    .run_command_silent("fwupdmgr", &["get-updates", "-y"])
                    .await
                {
                    updates.firmware = lines
                        .into_iter()
                        .filter(|l| l.contains("→") || l.contains("New version"))
                        .collect();
                }
                spinner.finish_and_clear();
            }

            Ok(updates)
        }

        pub async fn update_system(&self) -> Result<()> {
            if !crate::system::command_exists("dnf5") {
                return Err(UpdateError::CommandNotFound("dnf5".into()));
            }

            let spinner = self.create_spinner("Updating system packages...");

            self.run_command("dnf5", &["update", "--refresh", "-y"], "[DNF5]")
                .await?;

            spinner.set_message("Removing unused packages...");
            self.run_command("dnf5", &["autoremove", "-y"], "[DNF5]")
                .await?;

            spinner.finish_with_message("System update complete ✓".green().to_string());
            self.summary.lock().await.system_updated = true;
            Ok(())
        }

        pub async fn update_flatpak(&self) -> Result<()> {
            if !crate::system::command_exists("flatpak") {
                info!("Flatpak not installed, skipping");
                return Ok(());
            }

            let spinner = self.create_spinner("Updating Flatpak applications...");

            self.run_command("flatpak", &["update", "-y"], "[Flatpak]")
                .await?;

            spinner.set_message("Removing unused Flatpak runtimes...");
            self.run_command("flatpak", &["uninstall", "--unused", "-y"], "[Flatpak]")
                .await?;

            spinner.finish_with_message("Flatpak update complete ✓".green().to_string());
            self.summary.lock().await.flatpak_updated = true;
            Ok(())
        }

        pub async fn update_firmware(&self) -> Result<()> {
            if !crate::system::command_exists("fwupdmgr") {
                info!("fwupdmgr not installed, skipping firmware updates");
                return Ok(());
            }

            let spinner = self.create_spinner("Checking for firmware updates...");

            let _ = self
                .run_command("fwupdmgr", &["refresh", "--force"], "[Firmware]")
                .await;

            spinner.set_message("Applying firmware updates...");
            match self
                .run_command("fwupdmgr", &["update", "-y"], "[Firmware]")
                .await
            {
                Ok(_) => {
                    spinner.finish_with_message("Firmware update complete ✓".green().to_string());
                    self.summary.lock().await.firmware_updated = true;
                }
                Err(UpdateError::CommandFailed { code: 2, .. }) => {
                    spinner.finish_with_message("No firmware updates available".yellow().to_string());
                }
                Err(e) => return Err(e),
            }

            Ok(())
        }
    }

    pub async fn check_reboot_required() -> Result<Option<String>> {
        if !crate::system::command_exists("dnf5") {
            return Ok(None);
        }

        let output = Command::new("dnf5")
            .args(["needs-restarting", "-r"])
            .output()
            .await?;

        match output.status.code() {
            Some(0) => Ok(None),
            Some(1) => {
                let details = String::from_utf8_lossy(&output.stdout).to_string();
                Ok(Some(details))
            }
            _ => Ok(None),
        }
    }
}

async fn setup_signal_handler() -> tokio::sync::watch::Receiver<bool> {
    let (tx, rx) = tokio::sync::watch::channel(false);

    tokio::spawn(async move {
        let mut sigint =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
                .expect("Failed to setup SIGINT handler");

        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to setup SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                warn!("Received SIGINT, shutting down...");
            }
            _ = sigterm.recv() => {
                warn!("Received SIGTERM, shutting down...");
            }
        }

        let _ = tx.send(true);
    });

    rx
}

fn setup_logging(verbose: u8, quiet: bool, log_file: &PathBuf) {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let level = if quiet {
        Level::ERROR
    } else {
        match verbose {
            0 => Level::INFO,
            1 => Level::DEBUG,
            _ => Level::TRACE,
        }
    };

    let file_appender = tracing_appender::rolling::daily(
        log_file.parent().unwrap_or(&PathBuf::from("/var/log")),
        log_file.file_name().unwrap_or_default(),
    );

    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_ansi(true)
                .with_filter(EnvFilter::from_default_env().add_directive(level.into())),
        )
        .with(
            fmt::layer()
                .with_target(true)
                .with_ansi(false)
                .with_writer(file_writer)
                .with_filter(EnvFilter::new("info")),
        )
        .init();
}

fn print_banner() {
    println!(
        "{}",
        r#"
╔═══════════════════════════════════════════╗
║           SysUpdater v0.2.0               ║
║     Fedora System Update Automation       ║
╚═══════════════════════════════════════════╝"#
            .cyan()
    );
}

fn print_usage() {
    println!(
        "{}",
        r#"
╔═══════════════════════════════════════════╗
║           SysUpdater v0.2.0               ║
║     Fedora System Update Automation       ║
╚═══════════════════════════════════════════╝"#
            .cyan()
    );

    println!("\n{}\n", "USAGE".yellow().bold());
    println!("    {} [OPTIONS]\n", "sudo sysupdater".green());

    println!("{}\n", "COMMANDS".yellow().bold());

    let commands = [
        ("-r, --refresh", "Check and display available updates"),
        ("-u, --update-all", "Update everything (system + flatpak)"),
        ("    --update-system", "Update only system packages (dnf5)"),
        ("    --update-flatpak", "Update only Flatpak applications"),
        ("    --update-firmware", "Update only firmware"),
    ];

    for (cmd, desc) in commands {
        println!("    {}  {}", cmd.green(), desc);
    }

    println!("\n{}\n", "OPTIONS".yellow().bold());

    let options = [
        ("-f, --firmware", "Include firmware in --update-all"),
        ("-n, --dry-run", "Preview actions without executing"),
        ("    --no-reboot-prompt", "Skip reboot prompt after updates"),
        ("    --no-network-check", "Skip connectivity verification"),
        ("    --parallel", "Run updates concurrently"),
        ("-c, --config <FILE>", "Use custom config file"),
        ("-v, --verbose", "Increase verbosity (-v, -vv, -vvv)"),
        ("-q, --quiet", "Minimal output"),
    ];

    for (opt, desc) in options {
        println!("    {}  {}", opt.cyan(), desc);
    }

    println!("\n{}\n", "EXAMPLES".yellow().bold());

    let examples = [
        ("sysupdater --refresh", "Show what updates are available"),
        ("sysupdater --update-all", "Update system and flatpak"),
        ("sysupdater --update-all -f", "Update everything including firmware"),
        ("sysupdater --update-system", "Update only dnf5 packages"),
        ("sysupdater --dry-run -u", "Preview full update"),
    ];

    for (cmd, desc) in examples {
        println!("    {}  {}", format!("sudo {}", cmd).green(), format!("# {}", desc).dimmed());
    }

    println!(
        "\n{}\n    /etc/sysupdater.toml\n    ~/.config/sysupdater/config.toml\n",
        "CONFIG FILES".yellow().bold()
    );
}

fn print_available_updates(updates: &updater::AvailableUpdates) {
    println!("\n{}", "═".repeat(50).cyan());
    println!("{}", "         Available Updates".cyan().bold());
    println!("{}\n", "═".repeat(50).cyan());

    if updates.is_empty() {
        println!("  {} Your system is up to date!\n", "✓".green().bold());
        return;
    }

    // System packages
    if !updates.system.is_empty() {
        println!(
            "  {} {} package(s)\n",
            "System".yellow().bold(),
            updates.system.len().to_string().white().bold()
        );
        for pkg in updates.system.iter().take(15) {
            let parts: Vec<&str> = pkg.split_whitespace().collect();
            if let Some(name) = parts.first() {
                let version = parts.get(1).unwrap_or(&"");
                println!("    {} {}", "•".dimmed(), format!("{} {}", name, version.dimmed()));
            }
        }
        if updates.system.len() > 15 {
            println!(
                "    {} ...and {} more",
                "•".dimmed(),
                (updates.system.len() - 15).to_string().yellow()
            );
        }
        println!();
    }

    // Flatpak
    if !updates.flatpak.is_empty() {
        println!(
            "  {} {} app(s)\n",
            "Flatpak".yellow().bold(),
            updates.flatpak.len().to_string().white().bold()
        );
        for app in updates.flatpak.iter().take(10) {
            let name = app.split_whitespace().next().unwrap_or(app.as_str());
            println!("    {} {}", "•".dimmed(), name);
        }
        if updates.flatpak.len() > 10 {
            println!(
                "    {} ...and {} more",
                "•".dimmed(),
                (updates.flatpak.len() - 10).to_string().yellow()
            );
        }
        println!();
    }

    // Firmware
    if !updates.firmware.is_empty() {
        println!(
            "  {} {} device(s)\n",
            "Firmware".yellow().bold(),
            updates.firmware.len().to_string().white().bold()
        );
        for fw in &updates.firmware {
            println!("    {} {}", "•".dimmed(), fw);
        }
        println!();
    }

    println!("{}", "═".repeat(50).cyan());
    println!(
        "  Total: {} update(s) available",
        updates.total_count().to_string().green().bold()
    );
    println!(
        "  Run {} to install\n",
        "sudo sysupdater --update-all".cyan()
    );
}

fn print_summary(summary: &updater::UpdateSummary) {
    println!("\n{}", "═".repeat(45).cyan());
    println!("{}", "           Update Summary".cyan().bold());
    println!("{}", "═".repeat(45).cyan());

    let check = "✓".green();
    let skip = "○".yellow();

    println!(
        "  System (dnf5):  {}",
        if summary.system_updated { &check } else { &skip }
    );
    println!(
        "  Flatpak:        {}",
        if summary.flatpak_updated { &check } else { &skip }
    );
    println!(
        "  Firmware:       {}",
        if summary.firmware_updated { &check } else { &skip }
    );

    if !summary.errors.is_empty() {
        println!("\n  {} Errors:", "✗".red());
        for err in &summary.errors {
            println!("    • {}", err.red());
        }
    }

    println!("{}", "═".repeat(45).cyan());
}

async fn prompt_reboot() -> error::Result<()> {
    use std::io::{self, Write};

    println!("\n{}", "A system reboot is recommended.".yellow().bold());
    println!("  1. Reboot now");
    println!("  2. Exit without rebooting");
    print!("\nChoice [1/2]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "1" => {
            info!("User requested reboot");
            Command::new("systemctl").args(["reboot"]).status().await?;
        }
        _ => {
            println!("{}", "Exiting without reboot.".green());
        }
    }

    Ok(())
}

async fn run(args: cli::Args) -> error::Result<()> {
    let config = config::Config::load(args.config.as_ref());
    let shutdown = setup_signal_handler().await;

    // Network check
    if !args.no_network_check {
        info!("Checking network connectivity...");
        let timeout = Duration::from_secs(config.network.timeout_secs);
        system::check_network(&config.network.check_url, timeout).await?;
        debug!("Network check passed");
    }

    let updater = updater::Updater::new(args.dry_run, args.quiet);

    // Handle --refresh: show available updates
    if args.refresh {
        let updates = updater.check_available_updates().await?;
        print_available_updates(&updates);
        return Ok(());
    }

    let summary = Arc::new(Mutex::new(updater::UpdateSummary::default()));

    // Determine what to update
    let do_system = args.update_all || args.update_system;
    let do_flatpak = args.update_all || args.update_flatpak;
    let do_firmware = args.update_firmware || (args.update_all && args.firmware);

    if !args.quiet {
        print_banner();
    }

    // Run updates
    if args.parallel && (do_system || do_flatpak || do_firmware) {
        info!("Running updates in parallel");
        let (sys_res, flat_res, fw_res) = tokio::join!(
            async {
                if do_system { updater.update_system().await } else { Ok(()) }
            },
            async {
                if do_flatpak { updater.update_flatpak().await } else { Ok(()) }
            },
            async {
                if do_firmware { updater.update_firmware().await } else { Ok(()) }
            },
        );

        for res in [sys_res, flat_res, fw_res] {
            if let Err(e) = res {
                summary.lock().await.errors.push(e.to_string());
            }
        }
    } else {
        // Sequential execution (default)
        if do_system {
            if let Err(e) = updater.update_system().await {
                error!("System update failed: {}", e);
                summary.lock().await.errors.push(e.to_string());
            }
        }

        if *shutdown.borrow() {
            return Err(error::UpdateError::Cancelled);
        }

        if do_flatpak {
            if let Err(e) = updater.update_flatpak().await {
                error!("Flatpak update failed: {}", e);
                summary.lock().await.errors.push(e.to_string());
            }
        }

        if *shutdown.borrow() {
            return Err(error::UpdateError::Cancelled);
        }

        if do_firmware {
            if let Err(e) = updater.update_firmware().await {
                error!("Firmware update failed: {}", e);
                summary.lock().await.errors.push(e.to_string());
            }
        }
    }

    // Print summary
    let final_summary = updater.summary().await;
    print_summary(&final_summary);

    // Check if reboot needed
    if !args.no_reboot_prompt && !args.dry_run {
        if let Ok(Some(reason)) = updater::check_reboot_required().await {
            info!("Reboot required: {}", reason);
            prompt_reboot().await?;
        } else {
            println!("\n{}", "No reboot required.".green());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = cli::Args::parse();

    // If no action specified, show usage
    if args.is_default() {
        print_usage();
        return ExitCode::SUCCESS;
    }

    let config = config::Config::load(args.config.as_ref());
    setup_logging(args.verbose, args.quiet, &config.logging.file);

    // Root check (not needed for just showing help)
    if let Err(e) = system::check_root() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        return ExitCode::from(1);
    }

    match run(args).await {
        Ok(()) => {
            info!("Operation completed successfully");
            ExitCode::SUCCESS
        }
        Err(error::UpdateError::Cancelled) => {
            eprintln!("\n{}", "Operation cancelled.".yellow());
            ExitCode::from(130)
        }
        Err(e) => {
            error!("Operation failed: {}", e);
            eprintln!("{} {}", "Error:".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}