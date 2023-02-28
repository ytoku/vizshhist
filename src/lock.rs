use std::fs::{File, OpenOptions};
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context as _, Result};
use nix::fcntl::{fcntl, FcntlArg};

// lockhistfile function in zsh
// https://github.com/zsh-users/zsh/blob/73d317384c9225e46d66444f93b46f0fbe7084ef/Src/hist.c#L3156

struct FcntlLockGuard {
    _file: File,
}

fn fcntl_lock(path: &Path) -> Result<FcntlLockGuard> {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .context("Failed to open the hist file")?;
    let fd = file.as_raw_fd();
    let flock = libc::flock {
        l_type: 0,
        l_whence: 0,
        l_start: 0,
        l_len: 0,
        l_pid: 0,
    };
    while let Err(errno) = fcntl(fd, FcntlArg::F_SETLKW(&flock)) {
        if errno != nix::errno::Errno::EINTR {
            bail!("Failed to lock the hist file: {}", errno.desc())
        }
    }

    Ok(FcntlLockGuard { _file: file })
}

pub struct HistFileLocker {
    histfile_path: PathBuf,
}

impl HistFileLocker {
    pub fn new(histfile_path: &Path) -> Self {
        HistFileLocker {
            histfile_path: histfile_path.to_owned(),
        }
    }

    pub fn lock_during<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() -> Result<()>,
    {
        let _fcntl_lock = fcntl_lock(&self.histfile_path)?;
        // TODO: symlink based lock

        f()
    }
}
