# Frostty Installation

1. [Prerequisites](#prerequisites)
    1. [Source Code](#clone-the-source-code)
    2. [Rust Compiler](#install-the-rust-compiler-with-rustup)
    3. [Dependencies](#dependencies)
        1. [Debian/Ubuntu](#debianubuntu)
        2. [Arch Linux](#arch-linux)
        3. [Fedora](#fedora)
        4. [CentOS/RHEL 7](#centosrhel-7)
        5. [openSUSE](#opensuse)
        6. [Slackware](#slackware)
        7. [Void Linux](#void-linux)
        8. [FreeBSD](#freebsd)
        9. [OpenBSD](#openbsd)
        10. [Solus](#solus)
        11. [NixOS/Nixpkgs](#nixosnixpkgs)
        12. [Gentoo](#gentoo)
        13. [Clear Linux](#clear-linux)
        14. [GNU Guix](#gnu-guix)
        15. [Alpine Linux](#alpine-linux)
        16. [Windows](#windows)
        17. [Other](#other)
2. [Building](#building)
    1. [Linux/Windows/BSD](#linux--windows--bsd)
    2. [macOS](#macos)
3. [Post Build](#post-build)
    1. [Terminfo](#terminfo)
    2. [Desktop Entry](#desktop-entry)
    3. [Manual Page](#manual-page)
    4. [Shell completions](#shell-completions)
        1. [Zsh](#zsh)
        2. [Bash](#bash)
        3. [Fish](#fish)

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
