# ltg_push - Push updates from log files to Telegram

## Use
1. Install Rust
2. Clone this
3. `cargo build --release`
4. `ltg_push <custom path to config file>` (default: `config.yml`)

## Config
```yaml
telegram:
    bot: 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11 # bot token
    chat_id: 123456789 # chat id
files: # paths to watching files
  - path: /var/some/log
  - path: /var/awesome/log
```
