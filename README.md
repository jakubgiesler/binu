# binu

A tiny TUI for finding and running binaries on your system. Type to search, read the man page on the right, hit enter to run.

![demo](demo.gif)

## What it does

Scans your standard binary directories (`/usr/bin`, `/bin`, `/sbin`, etc.), lets you fuzzy-search through all of them, shows the man page inline, and lets you tweak the command before running it.

## Usage

```
binu
```

- **Type** to filter binaries
- **↑ / ↓** to navigate or scroll the man page
- **Enter** to open the command editor
- **Enter again** to run, **Esc** to cancel
