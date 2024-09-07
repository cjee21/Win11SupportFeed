# Win11SupportFeed

Command-line application written in Rust that fetches and converts [Windows 11 Microsoft Support feed](https://support.microsoft.com/en-us/feed/atom/4ec863cc-2ecd-e187-6cb3-b50c6545db92) into a readable HTML file.

## Usage

Run `Windows11_SupportFeed_to_HTML.exe` and `Windows 11 Support Feed.html` will be created in the same folder.

## Pre-built binaries

The pre-built binaries can be found at the [releases page](https://github.com/cjee21/Win11SupportFeed/releases).

It is built with the following command:

```ps
cargo build --release
```

## References

- [Feed Picker - Microsoft Support](https://support.microsoft.com/en-us/rss-feed-picker)
- [RSS reader tutorial (Rust for Windows with VS Code)](https://learn.microsoft.com/en-us/windows/dev-environment/rust/rss-reader-rust-for-windows)
