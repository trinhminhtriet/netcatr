use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "netcatr", version, arg_required_else_help(true))]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Command,
       // #[clap(short, long)]
       // verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(alias = "l")]
    Listen {
        /// Interactive
        #[clap(short, long, name = "interactive")]
        interactive: bool,

        /// Block exit signals like CTRL-C
        #[clap(short, long, conflicts_with = "local-interactive")]
        block_signals: bool,

        /// Local interactive
        #[clap(
            short,
            long,
            name = "local-interactive",
            conflicts_with = "interactive"
        )]
        local_interactive: bool,

        /// Execute command when connection received
        #[clap(short, long)]
        exec: Option<String>,

        // Host:ip, IP if only 1 value provided
        #[clap(num_args = ..=2)]
        host: Vec<String>,
    },

    /// Connect to the controlling host
    #[clap(alias = "c")]
    Connect {
        /// The shell to use
        #[clap(short, long)]
        shell: String,

        // Host:ip, IP if only 1 value provided
        #[clap(num_args = ..=2)]
        host: Vec<String>,
    },

    /// Grab service banner from remote host
    #[clap(alias = "b")]
    Banner {
        /// Protocol to use (auto, http, https, ftp, ssh, smtp, telnet, raw)
        #[clap(short, long, default_value = "auto")]
        protocol: String,

        /// Connection timeout in seconds
        #[clap(short, long, default_value = "5")]
        timeout: u64,

        /// Verbose output
        #[clap(short, long)]
        verbose: bool,

        /// Save output to file
        #[clap(short = 'o', long)]
        output: Option<String>,

        /// Grab banners from multiple ports (comma-separated)
        #[clap(short = 'm', long)]
        multiple_ports: Option<String>,

        // Host:port, port if only 1 value provided
        #[clap(num_args = ..=2)]
        host: Vec<String>,
    },
}
