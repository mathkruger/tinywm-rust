
CP() {
    mkdir -p $(dirname "$2") && cp "$1" "$2"
}

cargo build -r
CP target/release/tinywm-rust $HOME/.tinywm-rust/tinywm-rust

echo "TinyWM Compiled and Copied to the right directory!"