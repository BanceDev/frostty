# Frostty Installation

## Prerequisites

### Clone the source code

Before compiling Frostty, you'll have to first clone the source code:

```sh
git clone https://github.com/BanceDev/frostty.git
cd frostty
```

### Install the Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

3. To make sure you have the right Rust compiler installed, run

   ```sh
   rustup override set stable
   rustup update stable
   ```

### Dependencies

These are the minimum dependencies required to build Alacritty, please note
that with some setups additional dependencies might be desired.

If you're running Wayland with an Nvidia GPU, you'll likely want the EGL
drivers installed too (these are called `libegl1-mesa-dev` on Ubuntu).

#### Debian/Ubuntu

```sh
apt install just cmake g++ pkg-config libfreetype6-dev libfontconfig1-dev libxcb-xfixes0-dev libxkbcommon-dev python3
```

#### Arch Linux

```sh
pacman -S just cmake freetype2 fontconfig pkg-config make libxcb libxkbcommon python
```

#### Other

If you build Frostty on another distribution, we would love some help
filling in this section of the README.

## Building and Installation

```sh
just build-release
sudo just install
```
