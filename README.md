# Rustshot-gtk
Version: 0.5.0

## Description
Simple screenshot program that works on SwayWM. 
Uses Grim. AH! It also supports screen recording!
Not tested on other DEs, feel free to do it.

![image](./images/capture1.png)

## Motivation
I couldn't get Flameshot to work on sway (every screenshot was half black). 
I decided to create my own program using Rust and GTK4.

## Features
Version 0.5:
- Take screenshots and copy to clipboard;
- Take screenshots and save to file;
- screen recording;
- Annotations:
    - Boxes and circles;
    - Arrows and lines;
    - Numbered annotations;
    - Free-hand writing;
- Possibility to change annotations colors;

## Requirements
- [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) 
- fontawesome, grim, wf-recorder:
```{bash}
sudo dnf install fontawesome-fonts-all grim wf-recorder
```

## Installation
Clone this repo 

```{bash}
git clone https://github.com/sebastiano123-c/rustshot-gtk.git
```

Compile and save to `~/.local/bin` 
```{bash}
cd rustshot-gtk
cargo build --release && cp ./target/release/rustshot-gtk ~/.local/bin 
```

Add in `~/.config/sway/config` to display the program in fullscreen
```{bash}
# bind key (change to whatever key bind you prefer)
bindsym Shift+$mod+s exec $take_screenshot

# opens in fullscreen
for_window [app_id="rustshot-gtk"] border pixel 0, floating enable, fullscreen disable, move absolute position 0 0
```

## TODO
For version 0.5.1 
- add the video recording with wf-recorder:
    - add a help window that says how to exit the recording (by pressing 'esc')
    - is it possible to have better resolution?
    - is it possible to move the frame when moving the screen shot box?
    - add recording settings in the toolbox settings
    
For version 0.6.0
- add pixelated square boxes
- move the toolbox across the screen when in fullscreen 

## NOT SO IMPORTANT FIXES
- add the opportunity to draw screenshot area continuously (Not really important, probably will not be implemented)
- reduce the toolbox buttons size (I cannot go below 50px because the min size is 50 for buttons, probably will not be implemented)
- add a way to catch eye attention when fullscreen is enabled (now that the toolbox buttons are a lot, probably useless)
- add a button the toggle fullscreen back to previous size (is this really useful?)

## Tested on
- SwayWM
