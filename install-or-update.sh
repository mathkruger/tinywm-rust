CP() {
    mkdir -p $(dirname "$2") && cp "$1" "$2"
}

echo "--- Building executable ---"
cargo build -r

echo "--- Copying executable to your HOME folder ---"
CP target/release/tinywm-rust $HOME/.tinywm-rust/tinywm-rust

echo "--- Copying desktop entry to your sessions folder ---"
sudo cp tinywm.desktop /usr/share/xsessions/tinywm.desktop

echo "Installation/Update process finishid. Log out and Log in again using TinyWM!"