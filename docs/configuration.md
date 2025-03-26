# Configuration

## Colors

All colors in frostty configuration are written in hex format.
Both ```#rrbbgg``` and ```#rrbbggaa``` formats are accepted.
Alpha is assumed to be max if not provided.

### Application Colors

To set the base app colors in frostty use the following format in your frostty.toml

```toml
[colors.app]
background = "#000000" # app background color
active = "#000000" # active terminal border color
inactive = "#000000" # inactive terminal border color
```

### Primary Colors

To set the primary colors for the terminal use the following format in your frostty.toml

```toml
[colors.primary]
foreground = "#000000"
background = "#000000"
dim_foreground = "#000000"
bright_foreground = "#000000"
```

### Terminal Colors

To set the terminal colors you can set normal, bright, and dim colors.

```toml
# use the same format for colors.bright and colors.dim
[colors.normal]
black = "#000000"
red = "#000000"
green = "#000000"
yellow = "#000000"
blue = "#000000"
magenta = "#000000"
cyan = "#000000"
white = "#000000"
```

## Font

Table for configuring the terminal fonts. You can configure a font as follows.

```toml
[font]
family = "JetBrainsMono Nerd Font Mono"
size = 14.0 # floating point number
```

## General

Table for general settings that don't fit into another subcategory

### Wallpaper

To set the wallpaper in frostty first place an image into the .config/frostty directory and then set the wallpaper variable under the general table.

```toml
[general]
wallpaper = "wallpaper.png"
```
