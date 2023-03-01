mod lock;
mod meta;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{ensure, Context as _, Result};
use tempfile::NamedTempFile;

use crate::lock::HistFileLocker;
use crate::meta::{metafy, unmetafy};

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    histfile: Option<PathBuf>,
}

fn run_command(command: &mut Command) -> Result<()> {
    let exit_status = command.spawn()?.wait()?;
    ensure!(exit_status.success(), "Command failed");
    Ok(())
}

#[inline]
fn transform_file<F>(src_path: &Path, dst_path: &Path, f: F) -> Result<()>
where
    F: FnMut(&[u8], &mut Vec<u8>),
{
    let mut f = f;

    let mut reader = BufReader::new(File::open(src_path)?);
    let mut writer = BufWriter::new(File::create(dst_path)?);

    let mut rbuf = Vec::<u8>::with_capacity(1024);
    let mut wbuf = Vec::<u8>::with_capacity(1024);

    loop {
        rbuf.clear();
        let read_length = reader.read_until(b'\n', &mut rbuf)?;
        if read_length == 0 {
            break;
        }

        f(&rbuf, &mut wbuf);

        writer.write_all(&wbuf)?;
    }

    Ok(())
}

fn unmetafy_file(src_path: &Path, dst_path: &Path) -> Result<()> {
    transform_file(src_path, dst_path, unmetafy)
}

fn metafy_file(src_path: &Path, dst_path: &Path) -> Result<()> {
    transform_file(src_path, dst_path, metafy)
}

fn is_empty(path: &Path) -> Result<bool> {
    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        return Ok(true);
    }
    if metadata.len() > 1 {
        return Ok(false);
    }
    // whether the content is just "\n"
    let mut buf = [0u8; 1];
    file.read_exact(&mut buf)?;
    Ok(buf[0] == b'\n')
}

pub fn run(args: Args) -> Result<i32> {
    let Some(histfile) = &args.histfile.or_else(|| {
        env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(".zsh_history"))
    }) else {
        eprintln!("no HOME environment variable");
        return Ok(1);
    };

    let temp_file = NamedTempFile::new().context("Failed to craete a temporary file")?;
    let temp_file_path = temp_file.path();

    let histfile_locker = HistFileLocker::new(histfile);
    histfile_locker.lock_during(false, || unmetafy_file(histfile, temp_file_path))?;

    // We want to force vim to open the file in UTF-8 encoding.
    // But unfortunately --cmd option will be overwritten by .vimrc file.
    run_command(
        // TODO: make configurable. EDITOR / config file
        Command::new("vim")
            .arg("--cmd")
            .arg("set fileencodings=utf-8")
            .arg(temp_file_path),
    )?;
    if is_empty(temp_file_path)? {
        println!("Cancelled");
        return Ok(0);
    }

    // TODO: new record check

    histfile_locker.lock_during(true, || metafy_file(temp_file_path, histfile))?;
    Ok(0)
}
