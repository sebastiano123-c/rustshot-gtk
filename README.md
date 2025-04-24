# Rustshot-gtk
Version: 0.2.0

## Description
Simple screenshot program that works SwayWM. 
Uses Grim.
Not tested on other DEs.

## Motivation
I couldn't get Flameshot to work on sway (every screenshot was half black). 
I decided to create my own program using Rust and GTK4.

## Requirements
- fontawesome
```{bash}
sudo dnf install fontawesome-fonts-all
```
- grim 

```{bash}
sudo dnf install grim
```

Compile and save to `~/.local/bin` 
```{bash}
cargo build --release && cp ./target/release/rustshot-gtk ~/.local/bin 
```

## TODO
For version 0.3.0
- add a way to catch eye attention when fullscreen is enabled.
- add a button the toggle fullscreen back to previous size
- add freehand drawing
- add pixelated square boxes

## NOT SO IMPORTANT FIXES
- add the opportunity to draw screenshot area continuously (Not really important, probably will not be implemented)
- reduce the toolbox icons size (I cannot go below 50px because the min size is 50 for buttons, probably will not be implemented)
