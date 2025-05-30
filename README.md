<img src="https://pub-4e4118614197441ca01a142347434959.r2.dev/rfetch-banner.png" alt="">

<h2 align=center>rfetch – The system information tool for the terminal</h2>


### Installation
Available only Linux and macOS
```
curl -fsSL https://github.com/jamshid-elmurodov/rfetch/releases/download/1.0.0/install.sh | sudo bash
```

### Usage
To start rfetch:
```bash
rfetch
```

To edit the configuration:
```bash
vim ~/.config/rfetch/config.toml
```

### Configuration 
You can dynamically configure colors, ascii image, etc.
```toml
# Available colors: black, red, green, yellow, blue, magenta, cyan, white
#                   bright-black, bright-red, bright-green, bright-yellow, 
#                   bright-blue, bright-magenta, bright-cyan, bright-white
# If you don't specify a color, white will be used by default.

# If no parameters are specified, default values will be used automatically.

# Default value: bright-yellow
titles_color = "bright-yellow"

# Default value: white
infos_color = "white"

# The separator is ":"
# Default value: white
separator_color = "white"

# Path to the ASCII image source file (only .txt supported).
# Default value: default
image_source = "/Users/jamshidelmurodov/.config/rfetch/default.txt"

# Color of the ASCII image
# Default value: blue
image_color = "blue"

# The width of the space between the image and the text (1 gap = 1 space)
# Default value: 2
gap = 2

# Whether to show color blocks
# Default value: true
show_color_blocks = true
```

<h3>Preview</h3>
<img src="https://pub-4e4118614197441ca01a142347434959.r2.dev/rfetch-preview.png">
