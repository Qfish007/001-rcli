# RCLI

## 插件

cargo add structopt
cargo add clap --features derive  // --features 指定哪个feature

# csv
```bash
    cargo run csv -i ./assets/juventus.csv --format json
    cargo run csv -i ./assets/juventus.csv --format yaml
```

# genpass
```bash
    cargo run genpass -l 16 --uppercase
```

# base64
```bash
    cargo run base64 encode <<< "hello world"
    echo "dGVzdAo=" | cargo run base64 decode
    cargo run -- base64 encode --format urlsafe -i Cargo.toml

    # 使用管道输入： echo "test" | cargo run base64 encode
    # 使用 here-string： cargo run base64 encode <<< "test"
    # 直接输入（需要按 Ctrl+D 结束输入）： cargo run base64 encode
```
