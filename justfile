cl:
    cargo update
    cargo clean

b: cl
    cargo build --release
    cp target/release/lyred.exe lyred.exe

push msg:
    git commit -m "{{msg}}"
    git push origin main:main