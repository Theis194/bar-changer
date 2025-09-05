# Bar Changer 🎨

A powerful and intuitive command-line tool for effortlessly managing and switching between Waybar styles and configurations. Organize your themes and bars modularly for maximum customization.

## ✨ Features

- **Modular Design**: Separate color themes from bar layouts for ultimate flexibility.
- **Standardized Colors**: Use a consistent CSS variable naming scheme across all themes.
- **Simple CLI**: Switch bars and styles with simple, memorable commands.
- **Fast & Lightweight**: Built in Rust for blazing-fast performance.

## Install

### Prerequisites
- **Rust Toolchain**: Ensure you have Rust and Cargo installed. [Get it here](https://www.rust-lang.org/tools/install).
- **Waybar**: This tool manages your existing Waybar installation.

### Building from Source

```bash
# Clone repository
git clone https://github.com/Theis194/bar-changer

# cd inside and build
cd bar-changer && cargo build --release

# Add binary to path
cargo install --path .
```

## Usage

### 📁 Project Structure

Using bar-changer uses a directory layout with a focus on modular color styling:

```
.config/waybar/
├── style.css                  # The active, generated style file
├── config                     # The active, generated config file
├── themes/                    # Directory for your color themes
│   ├── dark.css
│   ├── light.css
│   └── catppuccin-mocha.css
└── bars/                      # Directory for your bar layouts
    ├── default.json
    └── minimal.json
```

This approach splits the color styling in to their own *\*.css* files, with a standardized color naming scheme, allowing for high customizability between bars and themes.

### Initializing Bar Changer

```bash
bar-changer -i
# 
bar-changer --init
```

This will trigger the initialization setup process.

#### Alternativ

You can manualy create a config at */home/ur-home-dir/.config/bar-changer/config.toml*

```toml
home_dir = "/home/ur-home-dir"
waybar_dir = ".config/waybar/"
```

### Change the Active Waybar Layout

```bash
bar-changer -b minimal
# or
bar-changer --bar minimal
```

### Change the Active Waybar Theme

```bash
bar-changer -s dark
# or
bar-changer --style dark
```

### Apply a New Look Instantly

```bash
bar-changer -b minimal -s dark
```

For full list of flags see:

```bash
bar-changer --help
```
## 🎨 Creating Themes

Themes are CSS files that define a standardized set of color variables. Below is an example of a Catppuccin Latte theme using the expected naming scheme.

```css
@define-color rosewater #dc8a78;
@define-color flamingo #dd7878;
@define-color pink #ea76cb;
@define-color mauve #8839ef;
@define-color red #d20f39;
@define-color maroon #e64553;
@define-color peach #fe640b;
@define-color yellow #df8e1d;
@define-color green #40a02b;
@define-color teal #179299;
@define-color sky #04a5e5;
@define-color sapphire #209fb5;
@define-color blue #1e66f5;
@define-color lavender #7287fd;
@define-color text #4c4f69;
@define-color subtext1 #5c5f77;
@define-color subtext0 #6c6f85;
@define-color overlay2 #7c7f93;
@define-color overlay1 #8c8fa1;
@define-color overlay0 #9ca0b0;
@define-color surface2 #acb0be;
@define-color surface1 #bcc0cc;
@define-color surface0 #ccd0da;
@define-color base #eff1f5;
@define-color mantle #e6e9ef;
@define-color crust #dce0e8;

@define-color primary @blue;
@define-color primary-content @lavender;
@define-color secondary @mauve;
@define-color secondary-content @pink;
@define-color accent @teal;
@define-color accent-content @sapphire;

@define-color base-100 @base;
@define-color base-200 @mantle;
@define-color base-300 @crust;

@define-color text-dark @text;
@define-color text-light @surface0;

@define-color info @sky;
@define-color success @green;
@define-color warning @yellow;
@define-color warning-100 @peach;
@define-color error @maroon;
```

Your bar configuration files in bars/ should then reference these standard variable names (e.g., {background: @base-100; color: @text-dark;}) to be universally compatible with all themes.

## ❓ FAQ

Q: How do I find my waybar config directory?
A: It's typically located at ~/.config/waybar/. If you have a different path, you may need to use a symbolic link.

Q: Does this tool restart Waybar automatically?
A: Yes! When you run a command, bar-changer updates the active style.css and config files and signals Waybar to reload.

## Closing Remarks

This tool was originally built to scratch my own itch. If you have any suggestions or find a bug, you are more than welcome to open an issue on GitHub!

However, I make no promises to implement every feature request unless it's something I would find useful for my own personal workflow.

Happy theming! 🎨

## 📄 License

This project is licensed under either of

- [MIT License](https://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)

at your option.
