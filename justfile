cl:
    cargo update
    cargo clean

b: cl
    cargo build --release
    cp target/release/lyred.exe lyred.exe

build:
    cargo build --release
    cp ./target/release/lyred.exe ./lyred.exe

run: build
    ./lyred

push msg:
    git commit -m "{{msg}}"
    git push origin main:main