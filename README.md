# Loading Screen

This Rust library provides functionality to display a loading screen while running tasks. It includes two primary functions:

- **`with_loading_screen`**: Displays a loading screen animation on a separate thread while executing a provided task synchronously.
- **`with_loading_screen_async`**: Displays a loading screen animation asynchronously while executing a provided task asynchronously.

The default loading screen animation is a donut spinner, but you can add your own if you're a little bitch who can't handle a donut.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
loading_screen = "0.1"
```
