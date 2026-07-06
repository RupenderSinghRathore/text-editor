### external crates used:

- **crossterm**: provides low level abstraction over ansi escape sequences and very low system calls to interact with the tty.

### Flow

run() - entry-point of editor in main function
↓
initialize() - enter into raw and alt screen mode
↓
load_file() - for any arg passed load file in buffer
↓
render() - draw the initial screen (either the file or wel-come screen)
↓
repl() - evaluates the keystrokes in an infinite loop
↓
terminate() - exit alt screen and restore cooked mode

### Components

```
src
├── editor
│   ├── terminal.rs
│   ├── view
│   │   ├── buffer.rs
│   │   ├── editing.rs
│   │   └── file.rs
│   └── view.rs
├── editor.rs
├── logger.rs
└── main.rs
```

**Graph of crates**

```tree
editor
├── terminal
├── view
│   ├── buffer.rs
│   ├── editing.rs
│   └── file.rs
└── view.rs

```

- **main.rs** is just startup glue.
- **editor.rs** owns the app loop and input handling.
- **terminal.rs** wraps crossterm.
- **view.rs** renders the screen.
- **editing.rs** edits the buffer/file.
- **buffer.rs** holds file contents.
- **file.rs** load and save files.
