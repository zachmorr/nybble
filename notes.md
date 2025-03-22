# Terminal Emulators

## Current Problems

CLI software has communicate using tty & use ascii escape sequences to display marked up text and get input (mouse, cursor & window info).
Current limitations of this method include:

- limited markup (no bold, italic, font size, etc)
- Lossy keyboard input. Control characters are not sent to application, they just add a modifier to the pressed value

Stuff that would be MUCH easier without escapes

- Mouse input (hover, click, scroll, etc)
- Clipboard support
- links
- multimedia
- drag and drop
- window info (focus, size, etc)
- Notifications
- dynamic prompts

## Current Solution

- The "Terminal" will act as an extremely simply display server that is limited to the functionality that is possible using ascii escapes
  - This can be run on device or remote PC
- nybble shell application on host pc will connect to device and emulate terminal app

## Open Questions

### What if shell application attempts to open Nybble desktop app (e.g. on that can't be done in a terminal)?

`$ nyb shell texteditor.exe`

Tought process

- Shell will detect that new window was opened and attempt to open nybble desktop client
- How will shell detect that a window is being opened?

### In the new scheme, will SW communicate with shell or term? Who is responsible for normal term operations?

Thought process

- Current terminal emulators implement
  - scrolling & wrapping
  - window & event APIs through ascii escapes
  - displaying text markup
  - passing keyboard input to stdin
- Current shells implement
  - prompting
  - built in tools
  - scripting language

## What sort of environment will the terminal/shell create for child programs?

Thought process

- Can an object model like powershell be used?
- Each process/job will get its own environment

## Current Terminal environment

- Displays contents written to stdout at cursor location
- Cursor follows characters written to stdout by default
- Provides ability to move cursor anywhere in a X by X character grid
- Allows probing window & mouse info

## Notes

- This cannot be solved by creating more file descriptors (e.g. stdctl,stdui). stdctl won't work because getting window & cursor info involves bi-directional communication between program and shell, while FDs are unidirectional. stdui won't work because terminal has to show output of stdout for backwards compatibility, but if something is also outputting markup out stdui then the two would get garbled together.
- Every program requires full, direct access to the display window to implement all features that escape codes are currently being used for.
- How do I give that ability to software while at the same time letting them completely ignore it and pretend like they only have stdin & stdout?
- The ease of cli tool development is because everything is already set up for the dev. Standard streams are already opened & in almost every language the built in print() will output to stdout by default
- Realisticly, all connections will be from a remote (usb, ip) source
