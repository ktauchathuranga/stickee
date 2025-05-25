# Stickee

A simple and lightweight sticky notes application for Linux, built with Rust and GTK4.

## Features

- **Simple and Clean**: Minimalist interface focused on note-taking
- **Auto-save**: Your notes are automatically saved as you type
- **Persistent Storage**: Notes are saved to `~/.stickee.json`
- **Word Wrap**: Text automatically wraps for better readability
- **GTK4 Integration**: Native look and feel on GNOME and other GTK-based desktops

## Installation

### From AUR (Recommended)

```bash
yay -S stickee
# or
paru -S stickee
```

### Building from Source

#### Prerequisites

- Rust (latest stable)
- GTK4 development libraries
- pkg-config

On Arch Linux:
```bash
sudo pacman -S rust gtk4 pkgconf
```

#### Build

```bash
git clone https://github.com/ktauchathuranga/stickee.git
cd stickee
cargo build --release
```

#### Install

```bash
sudo cp target/release/stickee /usr/local/bin/
```

## Usage

Launch Stickee from your application menu or run `stickee` in the terminal. Start typing your notes - they'll be automatically saved as you type.

## Development

Built with:
- **Rust** - System programming language
- **GTK4** - Cross-platform GUI toolkit
- **Serde** - Serialization framework for data persistence

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

**Ashen Chathuranga** - [ktauchathuranga](https://github.com/ktauchathuranga)

---

*Tested on Arch Linux with GNOME Desktop Environment*
