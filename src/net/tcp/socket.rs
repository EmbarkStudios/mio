use crate::net::{TcpStream, TcpListener};
use crate::sys;

use std::io;
use std::mem;
use std::net::SocketAddr;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd, FromRawFd};

/// TODO: dox
#[derive(Debug)]
pub struct TcpSocket {
    sys: sys::tcp::TcpSocket,
}

impl TcpSocket {
    /// TODO: dox
    pub fn new_v4() -> io::Result<TcpSocket> {
        sys::tcp::new_v4_socket().map(|sys| TcpSocket {
            sys
        })
    }

    /// TODO: dox
    pub fn new_v6() -> io::Result<TcpSocket> {
        sys::tcp::new_v6_socket().map(|sys| TcpSocket {
            sys
        })
    }

    pub(crate) fn new_for_addr(addr: SocketAddr) -> io::Result<TcpSocket> {
        if addr.is_ipv4() {
            TcpSocket::new_v4()
        } else {
            TcpSocket::new_v6()
        }
    }

    /// TODO: dox
    pub fn bind(&self, addr: SocketAddr) -> io::Result<()> {
        sys::tcp::bind(self.sys, addr)
    }

    /// TODO: dox
    pub fn connect(self, addr: SocketAddr) -> io::Result<TcpStream> {
        let stream = sys::tcp::connect(self.sys, addr)?;

        // Don't close the socket
        mem::forget(self);
        Ok(TcpStream::from_std(stream))
    }

    /// TODO: dox
    pub fn listen(self, backlog: u32) -> io::Result<TcpListener> {
        let listener = sys::tcp::listen(self.sys, backlog)?;

        // Don't close the socket
        mem::forget(self);
        Ok(TcpListener::from_std(listener))
    }

    // Private for now, but this could be made public eventually
    #[cfg(not(windows))]
    pub(crate) fn set_reuseaddr(&self, reuseaddr: bool) -> io::Result<()> {
        sys::tcp::set_reuseaddr(self.sys, reuseaddr)
    }
}

impl Drop for TcpSocket {
    fn drop(&mut self) {
        sys::tcp::close(self.sys);
    }
}

#[cfg(unix)]
impl AsRawFd for TcpSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.sys
    }
}

#[cfg(unix)]
impl FromRawFd for TcpSocket {
    /// Converts a `RawFd` to a `TcpStream`.
    ///
    /// # Notes
    ///
    /// The caller is responsible for ensuring that the socket is in
    /// non-blocking mode.
    unsafe fn from_raw_fd(fd: RawFd) -> TcpSocket {
        TcpSocket { sys: fd }
    }
}