# ltg_push - Push updates from log files to Telegram

## Use
1. Install Rust
2. Clone this
3. `cargo build --release`
4. `ltg_push <custom path to config file>` (default: `config.yml`)

## Config
```yaml
files: # paths to watching files
  - path: /var/some/log
  - path: /var/awesome/log
```
