# todot 🦀

A project-based, lightweight, responsive, and crash-resilient terminal To-Do application built with **Rust** and **Ratatui**.

[![asciicast](https://asciinema.org/a/FJeVh0P85rVfjtth.svg)](https://asciinema.org/a/FJeVh0P85rVfjtth)

---

## ✨ Features

- **As you know:** Yet another todo tool as you know.
- **Project-based storage by default:** The tool automatically creates and read&writes a local `todot.json` in your current working directory.
- **Global storage:** The tool can be used with global `todot.json` in `~/.local/share/todot/` via global flag.

---

## ⌨️ Keybindings

| Key     | Context   | Action                          |
| :------ | :-------- | :------------------------------ |
| `a`     | Main List | Open **Add Task** modal         |
| `e`     | Main List | Open **Edit Task** modal        |
| `d`     | Main List | Open **Delete / Confirm** modal |
| `↓`     | Main List | Move down                       |
| `↑`     | Main List | Move up                         |
| `Enter` | Modal     | Confirm input / Submit          |
| `Esc`   | Modal     | Cancel / Close modal            |
| `q`     | Main List | Quit application                |

---

## 🚀 Getting Started

### Prerequisites

- [Rust & Cargo](https://www.rust-lang.org/tools/install) (latest stable)

### Direct install via Cargo

```bash
cargo install --git https://github.com/thegears/todot
```

### Build & Run

```bash
# Clone the repository
git clone https://github.com/thegears/todot
cd todot

# Run directly
cargo run

# Build release binary
cargo build --release

# Build file will be on todot/target/release/
```

### Pre-built Binary

Download the latest executable directly from [Releases](https://github.com/thegears/todot/releases).

### Usage

Todot runs project-based by default

```bash
todot
```

You can use global storage with -g, --global flag

```bash
todot -g
#or
todot --global
```

---

## 🛠 Tech Stack

- **Ratatui**: TUI layout engine
- **Crossterm**: Terminal backend
- **color-eyre**: Error handling

---

## 📄 License

**MIT**
