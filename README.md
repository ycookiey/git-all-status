# git-all-status

TUI dashboard to monitor git status across multiple repositories.

![Rust](https://img.shields.io/badge/Rust-stable-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

## Features

- Scan multiple directories for git repositories
- Real-time status display: branch, dirty state, staged/unstaged/untracked files, ahead/behind counts
- Vim-style keybindings with mouse support
- Incremental background scanning with disk cache for instant startup
- Detail pane with file-level change breakdown
- Search filtering and sort modes (dirty-first / name / last commit)
- Launch lazygit directly on selected repository
- Cross-platform (Windows / macOS / Linux)

## Install

### GitHub Releases

Download a pre-built binary from [Releases](https://github.com/ycookiey/git-all-status/releases).

### Scoop (Windows)

```
scoop bucket add yscoopy https://github.com/ycookiey/yscoopy
scoop install git-all-status
```

### Build from source

```
cargo install --git https://github.com/ycookiey/git-all-status
```

## Configuration

Create `~/.config/git-all-status/config.toml` (on Windows: `%APPDATA%\git-all-status\config.toml`):

```toml
scan_dirs = [
    "~/projects",
    "~/work",
]
exclude = ["node_modules", ".cache", "vendor"]
interval_secs = 300
max_depth = 3
```

| Key | Description | Default |
|-----|-------------|---------|
| `scan_dirs` | Directories to scan for git repos | `["~/projects"]` |
| `exclude` | Directory names to skip | `[]` |
| `interval_secs` | Auto-refresh interval in seconds | `300` |
| `max_depth` | Max directory depth for scanning | `3` |

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` / `G` | Jump to top / bottom |
| `Ctrl+d/u` | Half-page down / up |
| `Ctrl+f/b` | Full-page down / up |
| `Tab` | Switch pane (list ↔ detail) |
| `/` | Search repositories |
| `s` | Cycle sort mode |
| `f` | Toggle dirty-only filter |
| `c` | Copy repo path to clipboard |
| `r` | Refresh scan |
| `Enter` | Open lazygit in selected repo |
| `?` | Toggle help overlay |
| `q` | Quit |

## License

[MIT](LICENSE)
