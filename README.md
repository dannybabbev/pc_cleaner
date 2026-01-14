# PC Cleaner

- Minimal and performant PC cleaner
- Simple and auditable code
- Safe: moves directories to trash, does not hard delete
- Minimal dependencies

Don't trust, verify!

## Features

- **Recursive scanning** - Finds cache directories throughout your filesystem
- **Time-based filtering** - Filter by last accessed time (1, 3, or 6 months ago)
- **Size reporting** - See exactly how much space you'll reclaim
- **Safe deletion** - Moves directories to trash (recoverable if needed)

## Usage
```
cargo run
```

### Notes
- Tested an ran on MacOS only; However, it should work on Windows and Linux