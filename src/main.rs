use crate::input::Command;
use crate::listener::{listen, Mode, Opts};
use crate::banner::{grab_banner, BannerOpts, Protocol};
use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use std::io::{stdout, Write};
use std::fs::OpenOptions;

mod input;
mod listener;
mod banner;  // New module

#[cfg(unix)]
mod unixshell;

#[cfg(windows)]
mod winshell;

fn host_from_opts(host: Vec<String>) -> Result<(String, String), String> {
    let fixed_host = if host.len() == 1 {
        
        let param = host.get(0).unwrap();
        if param.contains(':') {
            let parts: Vec<&str> = param.split(':').collect();
            if parts.len() == 2 {
                (parts[0].to_string(), parts[1].to_string())
            } else {
                return Err("Invalid host:port format".to_string());
            }
        } else {
            
            ("0.0.0.0".to_string(), param.to_string())
        }
    } else if let [host, port] = &host[..] {
        (host.to_string(), port.to_string())
    } else {
        return Err("Missing host or invalid format".to_string());
    };

    Ok(fixed_host)
}

fn grab_multiple_banners(host: &str, ports: &str, opts: &BannerOpts) -> Result<String, Box<dyn std::error::Error>> {

   use colored::Colorize;

    let mut results = String::new();
    
    for port_str in ports.split(',') {
        let port = port_str.trim();
        if port.is_empty() {
            continue;
        }

        results.push_str(&format!("\n{} {}:{}\n", "=== Banner for".cyan(), host.green(), port.cyan()));
        results.push_str(&format!("{}\n", "=".repeat(50).cyan()));

        let banner_opts = BannerOpts {
            host: host.to_string(),
            port: port.to_string(),
            timeout: opts.timeout,
            protocol: opts.protocol.clone(),
            verbose: opts.verbose,
        };

        match grab_banner(&banner_opts) {
            Ok(_) => {
                results.push_str("Banner grabbed successfully\n");
            }
            Err(e) => {
                results.push_str(&format!("Error: {}\n", e));
            }
        }
        results.push('\n');
    }

    Ok(results)
}

fn save_to_file(filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(filename)?;
    
    file.write_all(content.as_bytes())?;
    log::info!("Results saved to {}", filename);
    Ok(())
}

fn main() {
    // Configure logger
    if let Err(err) = Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                .warn(Color::Yellow)
                .info(Color::BrightGreen)
                .error(Color::Red);

            out.finish(format_args!(
                "{}{} {}",
                colors.color(record.level()).to_string().to_lowercase(),
                ":",
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .level(log::LevelFilter::Info)
        .chain(stdout())
        .apply()
    {
        println!("Failed to initialize logger: {}", { err });
        return;
    }

    let opts = input::Opts::parse();

    match opts.command {
        Command::Listen {
            interactive,
            block_signals,
            local_interactive,
            exec,
            host,
        } => {
            let (host, port) = match host_from_opts(host) {
                Ok(value) => value,
                Err(err) => {
                    log::error!("{}", err);
                    return;
                }
            };

            let opts = Opts {
                host,
                port,
                exec,
                block_signals,
                mode: if interactive {
                    Mode::Interactive
                } else if local_interactive {
                    Mode::LocalInteractive
                } else {
                    Mode::Normal
                },
            };

            if let Err(err) = listen(&opts) {
                log::error!("{}", err);
            };
        }
        Command::Connect { shell, host } => {
            let (host, port) = match host_from_opts(host) {
                Ok(value) => value,
                Err(err) => {
                    log::error!("{}", err);
                    return;
                }
            };

            #[cfg(unix)]
            if let Err(err) = unixshell::shell(host, port, shell) {
                log::error!("{}", err);
            }

            #[cfg(windows)]
            if let Err(err) = winshell::shell(host, port, shell) {
                log::error!("{}", err);
            }

            #[cfg(not(any(unix, windows)))]
            {
                log::error!("This feature is not supported on your platform");
            }
        }
        Command::Banner {
            protocol,
            timeout,
            verbose,
            output,
            multiple_ports,
            host,
        } => {
            let (host, port) = match host_from_opts(host) {
                Ok(value) => value,
                Err(err) => {
                    log::error!("{}", err);
                    return;
                }
            };

            let banner_opts = BannerOpts {
                host: host.clone(),
                port: port.clone(),
                timeout,
                protocol: Protocol::from_str(&protocol),
                verbose,
            };

            if let Some(ports) = multiple_ports {
                // Multiple port scanning
                match grab_multiple_banners(&host, &ports, &banner_opts) {
                    Ok(results) => {
                        if let Some(output_file) = output {
                            if let Err(e) = save_to_file(&output_file, &results) {
                                log::error!("Failed to save to file: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Error during multi-port banner grabbing: {}", e);
                    }
                }
            } else {
                // Single port banner grabbing
                if let Err(err) = grab_banner(&banner_opts) {
                    log::error!("{}", err);
                } else if let Some(output_file) = output {
                    let content = format!("Banner grab results for {}:{}\n", host, port);
                    if let Err(e) = save_to_file(&output_file, &content) {
                        log::error!("Failed to save to file: {}", e);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    use super::unixshell;

    use std::io::ErrorKind;

    #[test]
    #[cfg(unix)]
    fn revshell_bad_port() {
        assert_eq!(
            unixshell::shell(
                "0.0.0.0".to_string(),
                "420692223".to_string(),
                "bash".to_string()
            )
            .map_err(|e| e.kind()),
            Err(ErrorKind::InvalidInput)
        )
    }

    #[test]
    fn test_host_from_opts_single_port() {
        let host = vec!["8080".to_string()];
        let result = host_from_opts(host).unwrap();
        assert_eq!(result, ("0.0.0.0".to_string(), "8080".to_string()));
    }

    #[test]
    fn test_host_from_opts_host_port() {
        let host = vec!["192.168.1.1".to_string(), "8080".to_string()];
        let result = host_from_opts(host).unwrap();
        assert_eq!(result, ("192.168.1.1".to_string(), "8080".to_string()));
    }

    #[test]
    fn test_host_from_opts_colon_format() {
        let host = vec!["192.168.1.1:8080".to_string()];
        let result = host_from_opts(host).unwrap();
        assert_eq!(result, ("192.168.1.1".to_string(), "8080".to_string()));
    }
}
