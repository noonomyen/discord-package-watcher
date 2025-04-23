# Discord Package Watcher

Watches a directory for new Discord `.deb` package downloads and prompts the user for immediate installation.

Currently supports **Debian-based systems only**, but the code is straightforward and can be adapted to other environments with minimal changes.

## Dependencies

This tool requires the following system dependencies to function:

- `zenity` - to display GUI prompts
- `polkit` (for `pkexec`) to request privilege elevation for installation
- `dpkg` - for installing `.deb` packages
- `gtk-launch` - to launch the installed Discord application (relies on `.desktop` file integration)

Make sure these are installed on your system before running the watcher.

## Installation

1. **Install the binary using Cargo**:

   ```sh
   cargo install --path .
   ```

   > The binary will be available at `~/.cargo/bin/discord-package-watcher`.

2. **Set up the systemd user service (Linux)**:

   ```sh
   mkdir -p ~/.config/systemd/user
   mkdir -p ~/Downloads/discord
   cp systemd/discord-package-watcher.service ~/.config/systemd/user/
   systemctl --user daemon-reload
   systemctl --user enable --now discord-package-watcher.service
   ```

   This will watch your `~/Downloads` folder for new `.deb` files and automatically prompt you to install them.

3. **Make sure your user environment supports systemd user services**
   If your system does not enable user services by default, you may need to run:

   ```sh
   loginctl enable-linger $USER
   ```

## Configuration

The default watched directory is:

```text
~/Downloads/discord
```

To change it, edit the `ExecStart` line inside the `.service` file accordingly.
