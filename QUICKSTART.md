# DeveLlamaGUI Pro - Quick Start Guide

## Installation

### 1. Install Rust (if not already installed)
Download and run the installer from: https://rustup.rs/

### 2. Build the Application
Open Command Prompt or PowerShell in the project folder:

```batch
cargo build --release
```

Wait for the build to complete (may take 5-10 minutes on first run).

### 3. Run the Application
```batch
target\release\DeveLlamaGUI-Pro.exe
```

Or double-click the executable in File Explorer.

## First Time Setup

### Configure Paths

1. **Model Path**: Click "Browse" next to "Model" field
   - Navigate to your GGUF model files (default: `H:\AI\models`)
   - Select a `.gguf` file

2. **Server Path**: Click "Browse" next to "Server" field
   - Navigate to your llama.cpp folder (default: `H:\AI\llama.cpp`)
   - Select `llama-server.exe`

### Recommended Settings for RTX 4060/4090

**RTX 4060 (8GB VRAM):**
- GPU Layers: 999 (full offload)
- Context Size: 4096-8192
- Flash Attention: ✓ Enabled
- KV Cache: f16

**RTX 4090 (24GB VRAM):**
- GPU Layers: 999 (full offload)
- Context Size: 16384-32768
- Flash Attention: ✓ Enabled
- KV Cache: f16 or q8_0 for larger models

## Starting the Server

1. Click the green **START** button
2. Wait for "Running" status to appear
3. Click **Open WebUI** to access the chat interface

## Stopping the Server

Click the red **STOP** button to safely shut down the server.

## Using Presets

1. Select a preset from the dropdown in the right panel
2. Click "Apply" to load the preset parameters
3. Or click "Save" to save current settings as a new preset

## Quick Commands

Use the Console panel at the bottom:
- Type `/clear` to clear the console
- Type `/help` for available commands
- Use Quick Commands buttons for common actions

## Troubleshooting

### Server won't start
- Check that model path is correct
- Verify llama-server.exe exists
- Check Windows Defender/antivirus isn't blocking the executable

### Out of memory
- Reduce GPU Layers
- Lower Context Size
- Use q4_0 or q8_0 KV cache type

### Slow performance
- Enable Flash Attention
- Increase Batch Size
- Check GPU is being used (not CPU fallback)

## Tips

- Settings are automatically saved between sessions
- Use the Command Preview panel to see the full command being executed
- Copy commands to clipboard for debugging or batch scripts
- Press Escape to close the "All Commands" dialog
