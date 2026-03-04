use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::config;

pub fn run(version: Option<&str>) -> Result<()> {
    let install_url = config::install_url();

    println!("Self update uses install.sh from:");
    println!("{}", install_url);

    let mut script = format!("curl -fsSL '{}' | sh", install_url);
    if let Some(value) = version {
        if !is_safe_version(value) {
            bail!("--version must match [A-Za-z0-9._-]+");
        }
        script.push_str(" -s -- --version ");
        script.push_str(value);
    }

    let output = Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .context("failed to execute self-update install command")?;

    if !output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }
    if !output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }

    if !output.status.success() {
        bail!("self-update failed");
    }

    Ok(())
}

fn is_safe_version(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '.' || ch == '_' || ch == '-')
}
