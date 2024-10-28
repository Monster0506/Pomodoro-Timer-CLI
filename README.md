# Pomodoro Timer CLI (Rust)

A simple command-line Pomodoro timer built in Rust. This timer helps you break down work sessions into intervals of focused work followed by short breaks, enhancing productivity with time-based goal setting.

## Features

- **Customizable Intervals**: Set your own work and break durations.
- **Automatic Session Reset**: Automatically starts the next session after each break.
- **Simple CLI**: Easy to use and minimal command-line interface.

## Installation

1. **Clone the repository**

   ```bash
   git clone https://github.com/yourusername/pomodoro-timer-cli.git
   ```

2. **Navigate into the directory**

   ```bash
   cd pomodoro-timer-cli
   ```

3. **Build the project**

   ```bash
   cargo build --release
   ```

4. **Run the executable**

   ```bash
   ./target/release/pomodoro-timer
   ```

## Usage

The Pomodoro timer has default work and break intervals but can be customized using command-line arguments.

```bash
./pomodoro-timer [OPTIONS]
```

### Options

- `-w`, `--work`: Set the work duration in minutes (default is 25 minutes)
- `-b`, `--break-time`: Set the break duration in minutes (default is 5 minutes)
- `-l`, `--long-break`: Set the long break duration after a set number of sessions (default is 15 minutes)
- `-c`, `--cycles`: Number of work/break cycles before a long break (default is 4 cycles)

#### Debug Mode

For testing purposes, a `--debug` flag has been added, which interprets each minute as a second. This allows you to quickly test different intervals without waiting for full-length sessions.

```bash
./pomodoro-timer --work 2 --break 1 --cycles 2 --debug
```

In this example:

- The work interval is set to 2 seconds (interpreted as 2 minutes without the flag).
- Short breaks are set to 1 second.
- After 2 cycles, a long break will be triggered if specified.

Use `--debug` to enable this feature and remove it for normal timing.

### Example

```bash
./pomodoro-timer --work 30 --break 10 --cycles 3
```

In this example, the timer will set:

- 30-minute work intervals
- 10-minute short breaks
- A long break after every 3 cycles

## Contributing

Feel free to contribute by opening an issue or submitting a pull request. Contributions are always welcome!

## Future Improvements

- **Audio Alert (Optional)**: Notifies when a session ends (if sound support is available on your system).
- **Cycle Tracking**: Implement an option to save session data to a text file if users want to track productivity.
- **Session Persistence**: Some users might appreciate saving session data (like total cycles completed) as a log or text file. This could allow them to track their productivity across sessions, which adds a layer of functionality without complicating the CLI.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
