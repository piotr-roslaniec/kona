//! This module contains the local types for the `kona-common` crate.

#[cfg(feature = "std")]
use std::os::fd::{AsRawFd, OwnedFd};

#[cfg(feature = "std")]
use crate::errors::IOError;
use crate::errors::IOResult;

/// File descriptors available to the `client` within the FPVM kernel.
#[derive(Debug)]
pub enum FileDescriptor {
    /// Read-only standard input stream.
    StdIn,
    /// Write-only standaard output stream.
    StdOut,
    /// Write-only standard error stream.
    StdErr,
    /// Read-only. Used to read the status of pre-image hinting.
    HintRead,
    /// Write-only. Used to provide pre-image hints
    HintWrite,
    /// Read-only. Used to read pre-images.
    PreimageRead,
    /// Write-only. Used to request pre-images.
    PreimageWrite,
    #[cfg(feature = "std")]
    /// Other file descriptor.
    Wildcard(OwnedFd),
}

impl From<FileDescriptor> for usize {
    fn from(fd: FileDescriptor) -> Self {
        match fd {
            FileDescriptor::StdIn => 0,
            FileDescriptor::StdOut => 1,
            FileDescriptor::StdErr => 2,
            FileDescriptor::HintRead => 3,
            FileDescriptor::HintWrite => 4,
            FileDescriptor::PreimageRead => 5,
            FileDescriptor::PreimageWrite => 6,
            #[cfg(feature = "std")]
            FileDescriptor::Wildcard(value) => value.as_raw_fd() as Self,
        }
    }
}

impl From<FileDescriptor> for i32 {
    fn from(fd: FileDescriptor) -> Self {
        usize::from(fd) as Self
    }
}

/// A trait for types that support fallible cloning.
pub trait TryClone: Sized {
    /// Clone the value, returning a `Result` to handle cases where cloning may fail.
    fn try_clone(&self) -> IOResult<Self>;
}

impl TryClone for FileDescriptor {
    /// Clone the file descriptor.
    fn try_clone(&self) -> IOResult<Self> {
        match self {
            Self::StdIn => Ok(Self::StdIn),
            Self::StdOut => Ok(Self::StdOut),
            Self::StdErr => Ok(Self::StdErr),
            Self::HintRead => Ok(Self::HintRead),
            Self::HintWrite => Ok(Self::HintWrite),
            Self::PreimageRead => Ok(Self::PreimageRead),
            Self::PreimageWrite => Ok(Self::PreimageWrite),
            #[cfg(feature = "std")]
            Self::Wildcard(fd) => {
                Ok(Self::Wildcard(fd.try_clone().map_err(|_| IOError(fd.as_raw_fd()))?))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_descriptor_into_usize() {
        assert_eq!(usize::from(FileDescriptor::StdIn), 0);
        assert_eq!(usize::from(FileDescriptor::StdOut), 1);
        assert_eq!(usize::from(FileDescriptor::StdErr), 2);
        assert_eq!(usize::from(FileDescriptor::HintRead), 3);
        assert_eq!(usize::from(FileDescriptor::HintWrite), 4);
        assert_eq!(usize::from(FileDescriptor::PreimageRead), 5);
        assert_eq!(usize::from(FileDescriptor::PreimageWrite), 6);
        assert_eq!(usize::from(FileDescriptor::Wildcard(7)), 7);
    }

    #[test]
    fn test_file_descriptor_into_i32() {
        assert_eq!(i32::from(FileDescriptor::StdIn), 0);
        assert_eq!(i32::from(FileDescriptor::StdOut), 1);
        assert_eq!(i32::from(FileDescriptor::StdErr), 2);
        assert_eq!(i32::from(FileDescriptor::HintRead), 3);
        assert_eq!(i32::from(FileDescriptor::HintWrite), 4);
        assert_eq!(i32::from(FileDescriptor::PreimageRead), 5);
        assert_eq!(i32::from(FileDescriptor::PreimageWrite), 6);
        assert_eq!(i32::from(FileDescriptor::Wildcard(7)), 7);
    }
}
