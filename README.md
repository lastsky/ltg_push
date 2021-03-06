# ltg_push [![crates.io](https://img.shields.io/crates/v/ltg_push.svg)](https://crates.io/crates/ltg_push) [![travis-ci.org](https://api.travis-ci.org/Ralvke/ltg_push.svg?branch=master)](https://travis-ci.org/Ralvke/ltg_push)
Push log files' updates to Telegram

## Use
1. Install Rust
2. Clone this
3. `cargo build --release`
4. Write config
5. Get chat ID
    1. Create bot
    2. Send any message to bot
    3. `ltg_push <custom path to config file> getChatID`
    4. Put received chat id to config
6. `ltg_push <custom path to config file>`

## Config
```yaml
telegram:
    bot: 123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11 # bot token
    chat_id: 123456789 # chat id (see Use.5)
files: # paths to watching files
  - path: /var/some/log
    match: "critical" # push updates only with this regex
  - path: /var/awesome/other_log
```

## Systemd service
1. Copy `ltg_push.service` to `/etc/systemd/system`
2. `systemctl daemon-reload`
3. Copy `ltg_push` to `/usr/bin`
4. Copy config to `/etc/ltg_push.yml`
