//!
//! Using the inotify syscalls with tokio support
//!

#![warn(missing_docs)]

use std::{
    ffi::{c_int, OsStr},
    io,
    mem::size_of,
    os::{fd::FromRawFd, unix::ffi::OsStrExt},
    path::{Path, PathBuf},
};

use tokio::{fs::File, io::AsyncReadExt};

mod mask;

pub use mask::Mask;

extern "C" {
    fn inotify_init1(flag: c_int) -> c_int;
    fn inotify_add_watch(fd: c_int, buf: *const u8, mask: u32) -> c_int;
    fn inotify_rm_watch(fd: c_int, wd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
}

/// Watch filesytem changes on linux
pub struct INotify {
    fd: c_int,
    file: File,
}

/// A WatchDescriptor
#[derive(Clone, Copy)]
pub struct Watch {
    wd: c_int,
}

/// An event returned by the kernel
#[derive(Debug)]
pub struct Event {
    /// The Watch associated with this event
    pub watch: Watch,

    /// The mask associated with this event
    pub mask: Mask,

    /// A cookie associated with the event
    pub cookie: u32,

    /// A path associated with this event (empty unless disambigous to the kernel)
    pub path: PathBuf,
}

#[repr(C)]
struct EventHeader {
    wd: c_int,
    mask: u32,
    cookie: u32,
    len: u32,
}

impl INotify {
    /// Build a new INotify
    pub fn new() -> io::Result<Self> {
        let fd = unsafe { inotify_init1(0) };

        if fd == -1 {
            return Err(io::Error::from_raw_os_error(fd));
        }

        let file = unsafe { File::from_raw_fd(fd) };

        Ok(Self { fd, file })
    }

    /// Add a file (, or directory) to be watched
    pub fn add(&mut self, path: &Path, mask: Mask) -> io::Result<Watch> {
        let path: &OsStr = path.as_ref();
        let res = unsafe { inotify_add_watch(self.fd, path.as_bytes().as_ptr(), mask.0) };
        if res == -1 {
            return Err(io::Error::from_raw_os_error(res));
        }

        Ok(Watch { wd: res })
    }

    /// remove a watch from this INotify
    pub fn rm(&mut self, watch: Watch) -> io::Result<()> {
        let res = unsafe { inotify_rm_watch(self.fd, watch.wd) };
        if res == -1 {
            return Err(io::Error::from_raw_os_error(res));
        }

        Ok(())
    }

    /// start watching for events
    pub async fn watch(&mut self) -> io::Result<Event> {
        const SIZE: usize = size_of::<EventHeader>();
        let mut buffer = [0u8; SIZE];

        let mut amt = 0;
        while amt < SIZE {
            amt += self.file.read(&mut buffer[amt..SIZE]).await?;
        }

        let header: EventHeader = unsafe { std::mem::transmute(buffer) };
        let total = header.len as usize;
        let mut buffer = [0u8; 0x1000];

        let mut amt: usize = 0;
        while amt < total {
            amt += self.file.read(&mut buffer[amt..total]).await?;
        }

        let os = OsStr::from_bytes(&buffer[0..total]);
        let path = PathBuf::from(os);

        Ok(Event {
            watch: Watch { wd: header.wd },
            mask: Mask(header.mask),
            cookie: header.cookie,
            path,
        })
    }

    /// intentionally close the inotify instance
    pub async fn close(self) -> io::Result<()> {
        std::mem::forget(self.file);
        let res = unsafe { close(self.fd) };

        if res == -1 {
            return Err(io::Error::from_raw_os_error(res));
        }

        Ok(())
    }
}

impl std::fmt::Debug for Watch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Watch").field(&self.wd).finish()?;
        Ok(())
    }
}
