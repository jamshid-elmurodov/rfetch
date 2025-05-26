# macOS (x86_64)
cargo build --target x86_64-apple-darwin --release

# macOS (ARM64)
cargo build --target aarch64-apple-darwin --release

# Linux (386)
cargo build --target i686-unknown-linux-gnu --release

# Linux (x86_64)
cargo build --target x86_64-unknown-linux-gnu --release

# Linux (ARM)
cargo build --target armv7-unknown-linux-gnueabihf --release

# Linux (ARM64)
cargo build --target aarch64-unknown-linux-gnu --release