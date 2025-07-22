use colored::Colorize;
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::io::Result;

pub struct BannerOpts {
    pub host: String,
    pub port: String,
    pub timeout: u64,
    pub protocol: Protocol,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    Auto,
    HTTP,
    HTTPS,
    FTP,
    SSH,
    SMTP,
    Telnet,
    Raw,
}

impl Protocol {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "http" => Protocol::HTTP,
            "https" => Protocol::HTTPS,
            "ftp" => Protocol::FTP,
            "ssh" => Protocol::SSH,
            "smtp" => Protocol::SMTP,
            "telnet" => Protocol::Telnet,
            "raw" => Protocol::Raw,
            _ => Protocol::Auto,
        }
    }

// Still working on this.
// Currently triggers a dead_code warning.
    pub fn get_default_port(&self) -> u16 {
        match self {
            Protocol::HTTP => 80,
            Protocol::HTTPS => 443,
            Protocol::FTP => 21,
            Protocol::SSH => 22,
            Protocol::SMTP => 25,
            Protocol::Telnet => 23,
            _ => 80,
        }
    }

    pub fn detect_from_port(port: u16) -> Self {
        match port {
            21 => Protocol::FTP,
            22 => Protocol::SSH,
            23 => Protocol::Telnet,
            25 => Protocol::SMTP,
            80 => Protocol::HTTP,
            443 => Protocol::HTTPS,
            _ => Protocol::Raw,
        }
    }
}

pub fn grab_banner(opts: &BannerOpts) -> Result<()> {
    let address = format!("{}:{}", opts.host, opts.port);
    
    if opts.verbose {
        log::info!("Attempting to connect to {}", address.cyan());
    }

    // Parse address
    let socket_addr = address.to_socket_addrs()?.next()
        .ok_or_else(|| std::io::Error::new(
            std::io::ErrorKind::InvalidInput, 
            "Invalid address"
        ))?;

    // Determine protocol
    let protocol = match opts.protocol {
        Protocol::Auto => Protocol::detect_from_port(opts.port.parse().unwrap_or(80)),
        _ => opts.protocol.clone(),
    };

    if opts.verbose {
        log::info!("Using protocol: {:?}", protocol);
    }

    // Connect with timeout
    let stream = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(opts.timeout))?;
    stream.set_read_timeout(Some(Duration::from_secs(opts.timeout)))?;
    stream.set_write_timeout(Some(Duration::from_secs(opts.timeout)))?;

    match protocol {
        Protocol::HTTP | Protocol::HTTPS => grab_http_banner(stream, &opts.host)?,
        Protocol::FTP => grab_ftp_banner(stream)?,
        Protocol::SSH => grab_ssh_banner(stream)?,
        Protocol::SMTP => grab_smtp_banner(stream)?,
        Protocol::Telnet => grab_telnet_banner(stream)?,
        Protocol::Raw | Protocol::Auto => grab_raw_banner(stream)?,
    }

    Ok(())
}

fn grab_http_banner(mut stream: TcpStream, host: &str) -> Result<()> {
    let request = format!("HEAD / HTTP/1.1\r\nHost: {}\r\nUser-Agent: netcatr/1.0\r\nConnection: close\r\n\r\n", host);
    
    stream.write_all(request.as_bytes())?;
    
    let mut response = String::new();
    let mut reader = BufReader::new(stream);
    
    // Read response headers
    loop {
        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 || line.trim().is_empty() {
            break;
        }
        response.push_str(&line);
    }

    print_banner_info("HTTP", &response);
    Ok(())
}

fn grab_ftp_banner(stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(stream);
    let mut banner = String::new();
    reader.read_line(&mut banner)?;
    
    print_banner_info("FTP", &banner);
    Ok(())
}

fn grab_ssh_banner(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
    
    print_banner_info("SSH", &banner);
    Ok(())
}

fn grab_smtp_banner(stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(stream);
    let mut banner = String::new();
    reader.read_line(&mut banner)?;
    
    print_banner_info("SMTP", &banner);
    Ok(())
}

fn grab_telnet_banner(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
    
    print_banner_info("Telnet", &banner);
    Ok(())
}

fn grab_raw_banner(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
    
    print_banner_info("Raw", &banner);
    Ok(())
}

fn print_banner_info(protocol: &str, banner: &str) {
    println!("{} {}", "Protocol:".green().bold(), protocol.cyan());
    println!("{}", "Banner:".green().bold());
    println!("{}", banner.trim());
    
    // Extract additional info based on protocol
    if protocol == "HTTP" {
        extract_http_info(banner);
    } else if protocol == "SSH" {
        extract_ssh_info(banner);
    } else if protocol == "FTP" {
        extract_ftp_info(banner);
    }
}

fn extract_http_info(banner: &str) {
    for line in banner.lines() {
        if line.to_lowercase().starts_with("server:") {
            println!("{} {}", "Server:".yellow().bold(), 
                    line.split_once(':').unwrap_or(("", "")).1.trim().bright_white());
        } else if line.to_lowercase().starts_with("x-powered-by:") {
            println!("{} {}", "Powered by:".yellow().bold(), 
                    line.split_once(':').unwrap_or(("", "")).1.trim().bright_white());
        }
    }
}

fn extract_ssh_info(banner: &str) {
    if let Some(version_info) = banner.lines().next() {
        if version_info.starts_with("SSH-") {
            println!("{} {}", "SSH Version:".yellow().bold(), 
                    version_info.trim().bright_white());
        }
    }
}

fn extract_ftp_info(banner: &str) {
    if banner.starts_with("220") {
        println!("{} {}", "FTP Ready:".yellow().bold(), 
                banner.trim().bright_white());
    }
}
