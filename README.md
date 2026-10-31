# todot 🦀

A lightweight, responsive, and crash-resilient terminal To-Do application built with **Rust** and **Ratatui**.

---

## ✨ Features

- **As you know:** Yet another todo tool as you know.
- **Local JSON:** The tool uses local json file for read&write.

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

---

## 🛠 Tech Stack

- **Ratatui**: TUI layout engine
- **Crossterm**: Terminal backend
- **color-eyre**: Error handling

## 📄 License

**MIT**
