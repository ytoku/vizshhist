use std::env;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, ensure, Context as _, Result};

use crate::config::Config;

const DEFAULT_EDITOR_COMMAND: &str = "/usr/bin/editor";

fn run_command(command: &mut Command) -> Result<()> {
    let exit_status = command.spawn()?.wait()?;
    ensure!(exit_status.success(), "Command failed");
    Ok(())
}

fn find_editor(config: &Config) -> Result<Vec<String>> {
    if let Some(editor) = &config.vizshhist.editor {
        if !editor.is_empty() {
            return shell_words::split(editor).context("Invalid editor command");
        }
    }
    for key in &["VISUAL", "EDITOR"] {
        let Some(value) = env::var_os(key) else { continue };
        if value.is_empty() {
            continue;
        };

        let command = value
            .to_str()
            .with_context(|| format!("Invalid string in {}", key))?;

        // VISUAL and EDITOR environment variables may contain
        // command line options at least in visudo:
        // https://github.com/sudo-project/sudo/blob/b013711e489b917b80d73d42656b3bc05c26d3e7/plugins/sudoers/editor.c#L137
        return shell_words::split(command).context("Invalid editor command");
    }
    Ok(vec![DEFAULT_EDITOR_COMMAND.to_owned()])
}

pub fn run_editor<P: AsRef<Path>>(file_path: P, config: &Config) -> Result<()> {
    let editor = find_editor(config)?;
    if editor.is_empty() {
        bail!("Invalid empty editor command");
    }
    run_command(
        Command::new(&editor[0])
            .args(&editor[1..])
            .arg(file_path.as_ref()),
    )
}
