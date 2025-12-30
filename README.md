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

# text
```bash 
    cargo test -- --nocapture
    # cargo run genpass -l 32 --uppercase > fixtures/blake3.txt
    # cargo run -- text sign -k ./fixtures/blake3.txt
    # 输入baotao 结果是 Fe1QxfINfMKR_vNvPS5lZ1ZTvFtKjVKSnLXO_kEMyVQ

    echo -n "baotao" | cargo run -- text verify -k ./fixtures/blake3.txt --sig Fe1QxfINfMKR_vNvPS5lZ1ZTvFtKjVKSnLXO_kEMyVQ

    #输入baotao 结果是 true
    echo -n "baotao" | cargo run text sign --format blake3 --key fixtures/blake3.txt

    cargo run text generate --output-path ./fixtures
```