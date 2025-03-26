# Features

This is a list of the unqie features of Frostty outside of its terminal emulation.

## Implemented

### Dynamic Tiling

You can open multiple tiled terminals by using the CTRL + SHIFT + N and close them
by using CTRL + SHIFT + Q. Tiled windows and also be resized by clicking and dragging
on the pane splits.

### Custom Wallpaper

By placing an image in the config folder and setting the wallpaper you can set
an image to be the wallpaper for your terminal. This is best paired with a lower
opacity terminal background color. See [configuration](./configuration.md) for more
information.

### Opening URLs with the mouse

You can open URLs with your mouse by clicking on them. The modifiers required to
be held and program which should open the URL can be setup in the configuration
file. If an application captures your mouse clicks, which is indicated by a
change in mouse cursor shape, you're required to hold <kbd>Shift</kbd> to bypass
that.

## Planned Features

### Workspaces

Have the ability to have multiple workspaces to swap back and forth between akin
to a tiling window manager.

### Images

Implement the kitty image protocol for image rendering in the terminal.
