# Discord Package Watcher

Watches a directory for new Discord `.deb` package downloads and prompts the user for immediate installation.

Currently supports **Debian-based systems only**, but the code is straightforward and can be adapted to other environments with minimal changes.

## Dependencies

This tool requires the following system dependencies to function:

- `zenity` - to display GUI prompts
- `polkit` (for `pkexec`) to request privilege elevation for installation
- `dpkg` - for installing `.deb` packages
- `systemd-run` - to launch Discord

Make sure these are installed on your system before running the watcher.

## Installation

### Build and install with Makefile

Use the provided Makefile to install and manage the watcher and its systemd user service:

```sh
make install
```

This will:

- Build the Rust binary (`cargo install --path .`)
- Create necessary directories (`~/.config/systemd/user` and `~/Downloads/discord`)
- Copy the systemd service file to the user systemd directory
- Reload and enable the service, starting it immediately

### Remove/uninstall

To disable and remove the service, run:

```sh
make remove
```

This will:

- Disable and stop the systemd service
- Remove the service file from your user systemd directory
- Reload the daemon

#### Note

Make sure your user environment supports systemd user services. If user lingering is disabled, enable it by running:

```sh
loginctl enable-linger $USER
```

## How it works

The systemd user service runs a background Rust program that watches a specified directory (by default, ~/Downloads/discord) for new discord*.deb files.

1. File Detection
    Watches for new `.deb` files matching `discord*.deb`.

2. User Prompt
    When detected, prompts the user via `zenity` for installation confirmation.

3. Privilege Elevation
    If the user agrees, invokes `pkexec dpkg --install <path-to-deb>` to install the package with elevated privileges.

4. Launch Discord
    After installation, runs `systemd-run --user --scope /usr/bin/discord` to start Discord outside the service cgroup.

## Configuration

The default watched directory is:

```text
~/Downloads/discord
```

To change it, edit the `ExecStart` line inside the `.service` file accordingly.
