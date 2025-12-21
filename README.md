# Karaoke-Lyric

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-Application-green?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

A lightweight terminal-based karaoke application built with Rust and Ratatui.

</div>

## Features

- Real-time lyric synchronization with color-coded progress
- Smooth character-by-character highlighting
- Progress bar with visual playback indicator
- Pause/Resume playback control
- Adjustable lyric timing with Up/Down arrows
- Customizable song configuration

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Build from source

```bash
git clone https://github.com/traitimtrongvag/Karaoke-Lyric.git
cd Karaoke-Lyric
cargo build --release
```

## Usage

### Running the application

```bash
cargo run
```

### Controls

| Key | Action |
|-----|--------|
| `Space` | Pause/Resume playback |
| `R` | Restart from beginning |
| `↑` | Delay lyrics by +0.1s |
| `↓` | Advance lyrics by -0.1s |
| `Q` | Quit application |

## Configuration

Edit `src/song_config.rs` to customize your karaoke song:

```rust
let title = "Your Song Title".to_string();
let duration = 180.0;  // Total duration in seconds
let start_position = 0.0;  // Starting position in seconds

let lyrics = vec![
    LyricLine {
        text: "First line of lyrics".to_string(),
        start_time: 0.0,
        end_time: 3.0,
    },
    // Add more lines...
];
```

### Time format conversion

Convert MM:SS to seconds:
- 1:30 = 90.0 seconds
- 2:45 = 165.0 seconds
- 4:20 = 260.0 seconds

## Project Structure

```
.
├── src/
│   ├── main.rs          # Core application logic
│   └── song_config.rs   # Song configuration (lyrics & timing)
├── Cargo.toml           # Project dependencies
└── README.md
```

## Dependencies

- `crossterm` - Terminal manipulation
- `ratatui` - Terminal UI framework

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Example

![Example](https://raw.githubusercontent.com/traitimtrongvag/Karaoke-Lyric/main/Example/Example-Lyric.gif)
