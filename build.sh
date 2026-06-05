echo "[*] Building..."
cargo build --release

echo "[*] Installing frame_lsp..."
cargo install --path crates/frame_lsp --force

echo "[*] Installing frame_cli..."
cargo install --path crates/frame_cli --force

echo "[*] Done!"
