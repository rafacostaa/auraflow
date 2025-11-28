# ✨ AuraFlow

A smart, cross-platform mouse jiggler app built with Tauri (Rust + Web UI) that keeps your computer awake by automatically detecting idle time and moving the mouse.

## Features

- 🎯 **Auto-detect idle time** - Only jiggles when you're away
- 🖱️ **Smart mouse movement** - Random, natural-looking movements
- ⚙️ **Customizable settings** - Adjust idle threshold and jiggle interval
- 🪶 **Lightweight** - Small bundle size (~3-5MB)
- 🚀 **No dependencies** - Portable, no Python or runtime required
- 💻 **Cross-platform** - Works on Windows, macOS, and Linux

## Screenshots

[Add screenshots here]

## Installation

### Download Pre-built Binary
1. Go to [Releases](../../releases)
2. Download the appropriate version for your OS:
   - **macOS**: `.dmg` or `.app`
   - **Windows**: `.msi` or `.exe`
   - **Linux**: `.deb` or `.AppImage`
3. Install and run!

### Build from Source

#### Prerequisites
- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (16+)
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

#### Steps
```bash
# Clone the repository
git clone https://github.com/rafacostaa/auraflow.git
cd auraflow

# Install dependencies
npm install

# Run in development mode
npm run dev

# Build for production
npm run build
```

The built app will be in `src-tauri/target/release/bundle/`

## Usage

1. **Launch the app** - Open AuraFlow
2. **Configure settings** (optional):
   - **Idle Threshold**: How long to wait before starting (default: 120 seconds)
   - **Jiggle Interval**: How often to move the mouse when idle (default: 60 seconds)
3. **Click Start** - The app will monitor your activity
4. **Walk away** - When idle time is reached, it will automatically jiggle the mouse
5. **Return** - As soon as you move the mouse, it pauses automatically

## Settings

- **Idle Threshold**: Time in seconds before auto-jiggle starts
- **Jiggle Interval**: Time in seconds between each mouse movement when idle

## Development

### Project Structure
```
auraflow/
├── src-tauri/          # Rust backend (mouse control logic)
│   ├── src/
│   │   └── main.rs     # Main Rust code
│   ├── Cargo.toml      # Rust dependencies
│   └── tauri.conf.json # Tauri configuration
├── ui/                 # Web UI (HTML/CSS/JS)
│   ├── index.html
│   ├── style.css
│   └── script.js
├── main.py            # Original Python prototype
└── package.json       # Node.js configuration
```

### Technologies Used
- **Tauri**: Cross-platform desktop app framework
- **Rust**: Backend logic and mouse control
- **Enigo**: Cross-platform mouse/keyboard control library
- **HTML/CSS/JS**: Simple, modern UI

## License

MIT License - feel free to use and modify!

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built with ❤️ using [Tauri](https://tauri.app/)
