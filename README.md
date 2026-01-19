# Beanybot

A 100% local, cross-platform Twitch bot.

The only supported platform are currently Windows and Linux. Android support will probably happen *at some point*. I do not own any Apple devices and that will probably never change. PRs are always welcome though.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Windows
Requires clang to compile, because of `piper-rs`. To install on Windows run:
```
winget install LLVM.LLVM --source winget
```
